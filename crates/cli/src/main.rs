mod commands;
mod ui;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "wadah")]
#[command(about = "Wadah - Contain Intelligence", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    verbose: bool,

    /// Security mode: permissive, standard, strict
    #[arg(long, global = true, default_value = "standard")]
    security: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new agent project
    Init {
        /// Project name
        #[arg(default_value = "my-agent")]
        name: String,

        /// Output directory
        #[arg(short, long, default_value = ".")]
        output: String,

        /// Security level: minimal, standard, strict
        #[arg(long, default_value = "standard")]
        security: String,
    },

    /// Package an agent into a .wpkg file
    Pack {
        /// Path to wadah.yaml manifest
        #[arg(short, long, default_value = "wadah.yaml")]
        manifest: String,

        /// Output .wpkg file path
        #[arg(short, long, default_value = "build/agent.wpkg")]
        output: String,
    },

    /// Verify a .wpkg package integrity
    Verify {
        /// Path to .wpkg file
        package: String,
    },

    /// Run an agent package
    Run {
        /// Path to .wpkg file or wadah.yaml
        package: String,

        /// Output trace file
        #[arg(short, long)]
        trace: Option<String>,

        /// Input prompt
        #[arg(short, long)]
        prompt: Option<String>,

        /// Interactive mode
        #[arg(short, long)]
        interactive: bool,

        /// Override security mode (permissive, standard, strict)
        #[arg(long)]
        security: Option<String>,
    },

    /// Trace commands
    Trace {
        #[command(subcommand)]
        command: TraceCommands,
    },

    /// Push package to OCI registry
    Push {
        /// Package reference (e.g., ghcr.io/org/agent:tag)
        reference: String,

        /// Path to .wpkg file
        #[arg(short, long, default_value = "build/agent.wpkg")]
        package: String,

        /// Use insecure HTTP
        #[arg(long)]
        insecure: bool,
    },

    /// Pull package from OCI registry
    Pull {
        /// Package reference (e.g., ghcr.io/org/agent:tag)
        reference: String,

        /// Output .wpkg file path
        #[arg(short, long, default_value = "build/agent.wpkg")]
        output: String,

        /// Use insecure HTTP
        #[arg(long)]
        insecure: bool,
    },

    /// List available security plugins
    Plugins {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Serve agent with x402 payment requirements
    Serve(commands::serve::ServeArgs),
}

#[derive(Subcommand)]
enum TraceCommands {
    /// Replay a trace file
    Replay {
        /// Path to trace file (.jsonl)
        trace: String,

        /// Path to lockfile
        #[arg(short, long)]
        lock: Option<String>,
    },

    /// View trace statistics
    Stats {
        /// Path to trace file (.jsonl)
        trace: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("wadah={},wadah_runtime={}", log_level, log_level).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Print banner
    println!(
        "{}",
        "╔════════════════════════════════════════╗".bright_cyan()
    );
    println!(
        "{}",
        "║    Wadah - Contain Intelligence        ║".bright_cyan()
    );
    println!(
        "{}",
        "║       A Vessel for Thought             ║".bright_cyan()
    );
    println!(
        "{}",
        "╚════════════════════════════════════════╝".bright_cyan()
    );
    println!();

    // Execute command
    match cli.command {
        Commands::Init {
            name,
            output,
            security,
        } => {
            commands::init::execute(&name, &output, &security).await?;
        }
        Commands::Pack { manifest, output } => {
            commands::pack::execute(&manifest, &output).await?;
        }
        Commands::Verify { package } => {
            commands::verify::execute(&package).await?;
        }
        Commands::Run {
            package,
            trace,
            prompt,
            interactive,
            security: _,
        } => {
            commands::run::execute(&package, trace.as_deref(), prompt, interactive).await?;
        }
        Commands::Trace { command } => match command {
            TraceCommands::Replay { trace, lock } => {
                commands::trace::replay(&trace, lock.as_deref()).await?;
            }
            TraceCommands::Stats { trace } => {
                commands::trace::stats(&trace).await?;
            }
        },
        Commands::Push {
            reference,
            package,
            insecure,
        } => {
            commands::push::execute(&reference, &package, insecure).await?;
        }
        Commands::Pull {
            reference,
            output,
            insecure,
        } => {
            commands::pull::execute(&reference, &output, insecure).await?;
        }
        Commands::Plugins { verbose } => {
            commands::plugins::list(verbose).await?;
        }
        Commands::Serve(args) => {
            commands::serve::execute(args).await?;
        }
    }

    Ok(())
}
