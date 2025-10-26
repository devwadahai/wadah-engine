use anyhow::Result;
use wadah_spec::{SecurityPlugin, PluginConfig};
use crate::ui;
use colored::Colorize;

pub async fn list(verbose: bool) -> Result<()> {
    ui::info("Available Security Plugins");
    println!();
    
    let plugins = vec![
        SecurityPlugin::ToolCaps,
        SecurityPlugin::BudgetLimits,
        SecurityPlugin::NetworkPolicy,
        SecurityPlugin::FilesystemPolicy,
        SecurityPlugin::Tracing,
    ];
    
    for plugin in plugins {
        println!("  {} {}", "•".bright_cyan(), plugin.id().bright_white().bold());
        println!("    {}", plugin.description().dimmed());
        
        if verbose {
            print_plugin_details(&plugin);
        }
        println!();
    }
    
    println!();
    ui::info("Security Presets");
    println!();
    
    println!("  {} {}", "•".bright_green(), "permissive".bright_white().bold());
    println!("    {} No security plugins enabled", "└─".dimmed());
    println!("    {} Best for: Local development, experimentation", "└─".dimmed());
    println!();
    
    println!("  {} {}", "•".bright_yellow(), "standard".bright_white().bold());
    println!("    {} Budget limits only", "└─".dimmed());
    println!("    {} Best for: Most use cases, reasonable safety", "└─".dimmed());
    println!();
    
    println!("  {} {}", "•".bright_red(), "strict".bright_white().bold());
    println!("    {} All security plugins enabled", "└─".dimmed());
    println!("    {} Best for: Production, untrusted agents", "└─".dimmed());
    println!();
    
    println!();
    ui::info("Usage");
    println!();
    println!("  {}", "# Initialize with security level".dimmed());
    println!("  wadah init --security minimal");
    println!("  wadah init --security standard");
    println!("  wadah init --security strict");
    println!();
    println!("  {}", "# Run with override".dimmed());
    println!("  wadah run agent.yaml --security permissive");
    println!();
    println!("  {}", "# Enable specific plugins in wadah.yaml".dimmed());
    println!(r#"  plugins:
    - id: security.budgets
      enabled: true
      config:
        usd_per_day: 10.0
"#.dimmed());
    
    Ok(())
}

fn print_plugin_details(plugin: &SecurityPlugin) {
    match plugin {
        SecurityPlugin::ToolCaps => {
            println!("    {} Restricts which tools agents can use", "└─".dimmed());
            println!("    {} Rate limits tool calls", "└─".dimmed());
            println!("    {} Config: ToolCaps.json", "└─".dimmed());
        }
        SecurityPlugin::BudgetLimits => {
            println!("    {} Prevents excessive API costs", "└─".dimmed());
            println!("    {} Limits: tokens/min, $/day, duration", "└─".dimmed());
        }
        SecurityPlugin::NetworkPolicy => {
            println!("    {} Whitelist/blacklist domains", "└─".dimmed());
            println!("    {} Prevents data exfiltration", "└─".dimmed());
        }
        SecurityPlugin::FilesystemPolicy => {
            println!("    {} Control read/write access", "└─".dimmed());
            println!("    {} Restrict to specific paths", "└─".dimmed());
        }
        SecurityPlugin::Tracing => {
            println!("    {} Record all agent actions", "└─".dimmed());
            println!("    {} Enable deterministic replay", "└─".dimmed());
        }
    }
}

