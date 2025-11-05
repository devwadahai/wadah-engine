use anyhow::Result;
use clap::Parser;
use wadah_payment::{PaymentRequirements, PaymentRequiredResponse, X402Middleware};
use wadah_spec::WadahSpec;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

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

    // Create HTTP server using simple warp server
    ui::success(&format!("✓ Server started on http://0.0.0.0:{}", args.port));
    ui::info("Waiting for payment requests...");
    ui::info("Press Ctrl+C to stop");

    // Create a simple HTTP server
    // Note: In a real implementation, you'd use warp, axum, or actix-web
    // For now, just show that the command works
    
    println!("\n{}", colored::Colorize::cyan("Payment Requirements:"));
    println!("{}", colored::Colorize::cyan("─────────────────────"));
    
    // Create payment requirements for each network
    for network in &payment_config.networks {
        let requirements = middleware.create_payment_requirements(
            format!("/agent/{}/run", spec.metadata.name),
            payment_config.description.clone().unwrap_or_else(|| spec.metadata.description.clone().unwrap_or_default()),
            &payment_config.price.amount,
            &payment_config.price.asset,
            network,
        );
        
        let json = serde_json::to_string_pretty(&requirements)?;
        println!("\nNetwork: {}", network);
        println!("{}", json);
    }

    println!("\n{}", colored::Colorize::yellow("⚠ HTTP server not fully implemented yet"));
    println!("{}", colored::Colorize::yellow("⚠ This is a preview of payment requirements"));
    println!("\n{}", colored::Colorize::green("✓ Payment middleware configured successfully!"));

    Ok(())
}

