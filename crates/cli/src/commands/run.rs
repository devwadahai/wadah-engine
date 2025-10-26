use crate::ui;
use anyhow::Result;
use colored::Colorize;
use dialoguer::{Confirm, Input};
use std::path::Path;
use wadah_pack::PackageExtractor;
use wadah_runtime::{adapters::create_adapter, AgentExecutor};
use wadah_spec::WadahSpec;
use wadah_trace::TraceRecorder;

pub async fn execute(
    package: &str,
    trace_output: Option<&str>,
    prompt: Option<String>,
    interactive: bool,
) -> Result<()> {
    ui::info("Loading agent...");

    let package_path = Path::new(package);

    // Load spec (either from .wpkg or directly from wadah.yaml)
    let spec = if package_path.extension().and_then(|s| s.to_str()) == Some("wpkg") {
        // Extract package to temp directory
        let temp_dir = tempfile::tempdir()?;
        let extractor = PackageExtractor::new(package_path, temp_dir.path());
        extractor.extract()?;

        // Load spec from extracted package
        WadahSpec::from_file(&temp_dir.path().join("wadah.yaml"))?
    } else {
        // Load spec directly
        WadahSpec::from_file(package_path)?
    };

    ui::success(&format!(
        "Loaded agent: {} v{}",
        spec.metadata.name, spec.metadata.version
    ));

    // Create model adapter
    let adapter = create_adapter(
        &spec.runtime.model.provider,
        spec.runtime.model.endpoint.clone(),
        spec.runtime.model.model_id.clone(),
    )?;

    ui::info(&format!(
        "Using model: {} ({})",
        spec.runtime.model.model_id, spec.runtime.model.provider
    ));

    // Setup tracing if requested
    let recorder = if let Some(trace_path) = trace_output {
        let recorder = TraceRecorder::new(
            spec.metadata.name.clone(),
            spec.metadata.version.clone(),
            spec.runtime.model.params.seed,
        )
        .with_output(Path::new(trace_path))?;
        Some(recorder)
    } else {
        None
    };

    // Create executor
    let mut executor = AgentExecutor::new(spec.clone(), adapter)?;

    if let Some(rec) = recorder {
        executor = executor.with_tracing(rec);
    }

    // Interactive mode
    if interactive {
        ui::info("Entering interactive mode (Ctrl+C to exit)");
        println!();

        loop {
            let user_prompt: String = Input::new().with_prompt("You").interact_text()?;

            if user_prompt.trim().is_empty() {
                continue;
            }

            let spinner = ui::create_spinner("Thinking...");

            match executor.execute(user_prompt).await {
                Ok(response) => {
                    spinner.finish_and_clear();
                    println!("{} {}", "Agent:".bright_green().bold(), response);
                    println!();
                }
                Err(e) => {
                    spinner.finish_and_clear();
                    ui::error(&format!("Error: {}", e));
                    println!();
                }
            }

            if !Confirm::new()
                .with_prompt("Continue?")
                .default(true)
                .interact()?
            {
                break;
            }
        }
    } else {
        // Single execution mode
        let user_prompt = prompt.unwrap_or_else(|| "Hello! Please introduce yourself.".to_string());

        ui::info(&format!("Prompt: {}", user_prompt));

        let spinner = ui::create_spinner("Executing...");

        let response = executor.execute(user_prompt).await?;

        spinner.finish_and_clear();

        ui::success("Execution completed");
        println!();
        println!("{}", response);
        println!();

        // Show budget stats
        let stats = executor.get_budget_stats();
        ui::info(&format!("Elapsed: {}s", stats.elapsed_secs));
        ui::info(&format!("Tokens this minute: {}", stats.tokens_this_minute));
        ui::info(&format!("USD spent today: ${:.4}", stats.usd_spent_today));
    }

    // Finish and save trace
    if trace_output.is_some() {
        if let Some(trace) = executor.finish() {
            ui::success(&format!("Trace saved: {}", trace_output.unwrap()));
        }
    }

    Ok(())
}
