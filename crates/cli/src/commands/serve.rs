use anyhow::Result;
use axum::{
    Router,
    extract::{Path, State},
    http::{StatusCode, header, HeaderMap},
    response::{IntoResponse, Response, Json},
    routing::{get, post},
};
use clap::Parser;
use serde_json::json;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use wadah_payment::{PaymentRequiredResponse, X402Middleware};
use wadah_spec::WadahSpec;

use crate::ui;

#[derive(Parser)]
pub struct ServeArgs {
    /// Path to agent manifest (wadah.yaml)
    #[arg(value_name = "MANIFEST")]
    pub manifest: PathBuf,

    /// Port to serve on
    #[arg(short, long, default_value = "3402")]
    pub port: u16,

    /// Facilitator URL for payment verification and settlement
    #[arg(long, default_value = "https://facilitator.x402.org")]
    pub facilitator: String,
}

#[derive(Clone)]
struct AppState {
    spec: Arc<WadahSpec>,
    middleware: Arc<X402Middleware>,
}

pub async fn execute(args: ServeArgs) -> Result<()> {
    ui::info("Starting Wadah x402 Payment Server...");

    // Load agent specification
    let spec_content = std::fs::read_to_string(&args.manifest)?;
    let spec = WadahSpec::from_yaml(&spec_content)?;

    // Check if payment is configured
    let payment_config = match &spec.payment {
        Some(config) if config.enabled => config,
        Some(_) => {
            ui::warning("Payment is configured but not enabled");
            return Err(anyhow::anyhow!("Payment not enabled in wadah.yaml"));
        }
        None => {
            ui::error("No payment configuration found in wadah.yaml");
            return Err(anyhow::anyhow!("Missing payment configuration"));
        }
    };

    ui::info(&format!("Agent: {}", spec.metadata.name));
    ui::info(&format!("Price: {} {}", 
        payment_config.price.amount,
        payment_config.price.symbol.as_ref().unwrap_or(&"tokens".to_string())
    ));
    ui::info(&format!("Networks: {}", payment_config.networks.join(", ")));
    ui::info(&format!("Pay To: {}", payment_config.pay_to));

    // Create payment middleware
    let middleware = Arc::new(X402Middleware::new(
        &args.facilitator,
        &payment_config.pay_to,
    ));

    // Create app state
    let state = AppState {
        spec: Arc::new(spec),
        middleware,
    };

    // Build our application with routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/agent/:name/run", post(run_agent_handler))
        .route("/agent/:name/info", get(agent_info_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    ui::success(&format!("✓ Server started on http://{}", addr));
    ui::info("Endpoints:");
    ui::info("  GET  /health              - Health check");
    ui::info("  GET  /agent/:name/info    - Agent information");
    ui::info("  POST /agent/:name/run     - Run agent (requires payment)");
    ui::info("");
    ui::info("Press Ctrl+C to stop");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .await?;

    Ok(())
}

/// Root handler - shows API info
async fn root_handler() -> Json<serde_json::Value> {
    Json(json!({
        "service": "Wadah x402 Payment API",
        "version": "0.1.0",
        "endpoints": {
            "/health": "GET - Health check",
            "/agent/:name/info": "GET - Agent information",
            "/agent/:name/run": "POST - Run agent (requires x402 payment)"
        },
        "payment": {
            "protocol": "x402",
            "version": "1.0"
        }
    }))
}

/// Health check handler
async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Agent info handler - returns agent metadata
async fn agent_info_handler(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if state.spec.metadata.name != name {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(json!({
        "name": state.spec.metadata.name,
        "version": state.spec.metadata.version,
        "description": state.spec.metadata.description,
        "payment": state.spec.payment.as_ref().map(|p| json!({
            "enabled": p.enabled,
            "price": {
                "amount": p.price.amount,
                "symbol": p.price.symbol,
                "decimals": p.price.decimals,
            },
            "networks": p.networks,
            "pay_to": p.pay_to,
        }))
    })))
}

/// Run agent handler - requires x402 payment
async fn run_agent_handler(
    State(state): State<AppState>,
    Path(name): Path<String>,
    headers: HeaderMap,
    body: String,
) -> Result<Response, StatusCode> {
    if state.spec.metadata.name != name {
        return Err(StatusCode::NOT_FOUND);
    }

    // Check for X-PAYMENT header
    if let Some(payment_header) = headers.get("X-PAYMENT") {
        // Payment provided - verify it
        let payment_str = payment_header
            .to_str()
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        
        // Get payment config for verification
        let payment_config = state.spec.payment.as_ref().unwrap();
        let network = payment_config.networks.first()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        // Create requirements for verification
        let requirements = state.middleware.create_payment_requirements(
            format!("/agent/{}/run", name),
            payment_config.description.clone()
                .unwrap_or_else(|| state.spec.metadata.description.clone().unwrap_or_default()),
            &payment_config.price.amount,
            &payment_config.price.asset,
            network,
        );

        // Verify payment with middleware
        match state.middleware.verify_payment(payment_str, &requirements).await {
            Ok(verification) if verification.is_valid => {
                // Payment verified - execute agent
                // TODO: Actually run the agent with wadah-runtime
                let result = json!({
                    "status": "success",
                    "message": "Payment verified and agent executed",
                    "output": "Agent execution result would appear here",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                });

                Ok(Json(result).into_response())
            }
            Ok(verification) => {
                // Payment verification failed
                let error_response = json!({
                    "status": "error",
                    "message": "Payment verification failed",
                    "error": verification.invalid_reason.unwrap_or_else(|| "Invalid payment".to_string())
                });

                let mut response = Json(error_response).into_response();
                *response.status_mut() = StatusCode::PAYMENT_REQUIRED;
                Ok(response)
            }
            Err(err) => {
                // Payment verification error
                let error_response = json!({
                    "status": "error",
                    "message": "Payment verification error",
                    "error": err.to_string()
                });

                let mut response = Json(error_response).into_response();
                *response.status_mut() = StatusCode::PAYMENT_REQUIRED;
                Ok(response)
            }
        }
    } else {
        // No payment provided - return 402 with payment requirements
        let payment_config = state.spec.payment.as_ref().unwrap();
        
        // Get preferred network from query params or use first available
        let network = payment_config.networks.first()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        let requirements = state.middleware.create_payment_requirements(
            format!("/agent/{}/run", name),
            payment_config.description.clone()
                .unwrap_or_else(|| state.spec.metadata.description.clone().unwrap_or_default()),
            &payment_config.price.amount,
            &payment_config.price.asset,
            network,
        );

        let payment_response = PaymentRequiredResponse {
            x402_version: wadah_payment::X402_VERSION,
            accepts: vec![requirements],
            error: None,
        };

        let mut response = Json(payment_response).into_response();
        *response.status_mut() = StatusCode::PAYMENT_REQUIRED;
        
        // Add x402 header
        response.headers_mut().insert(
            header::HeaderName::from_static("www-authenticate"),
            header::HeaderValue::from_static("x402"),
        );

        Ok(response)
    }
}
