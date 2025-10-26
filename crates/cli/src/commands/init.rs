use crate::ui;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub async fn execute(name: &str, output_dir: &str, security_level: &str) -> Result<()> {
    let spinner = ui::create_spinner("Initializing project...");

    let output_path = Path::new(output_dir);

    // Create directory structure
    fs::create_dir_all(output_path.join("prompts"))?;

    // Only create security dirs if not minimal
    if security_level != "minimal" {
        fs::create_dir_all(output_path.join("tools"))?;
        fs::create_dir_all(output_path.join("code"))?;
    }

    fs::create_dir_all(output_path.join("build"))?;

    // Create wadah.yaml based on security level
    let wadah_yaml = match security_level {
        "minimal" => create_minimal_spec(name),
        "strict" => create_strict_spec(name),
        _ => create_standard_spec(name),
    };

    fs::write(output_path.join("wadah.yaml"), wadah_yaml)?;

    // Only create ToolCaps for strict mode
    if security_level == "strict" {
        let toolcaps = r#"{
  "version": "0.1",
  "allow": [
    {
      "tool": "http",
      "actions": ["GET"],
      "limits": {
        "per_min": 60
      },
      "domains": ["api.openai.com"]
    }
  ],
  "deny": []
}
"#;
        fs::write(output_path.join("ToolCaps.json"), toolcaps)?;
    }

    // Create example prompt
    let example_prompt = r#"You are a helpful AI assistant.

Be concise, accurate, and helpful in your responses."#;

    fs::write(output_path.join("prompts/system.txt"), example_prompt)?;

    // Create README
    let readme = create_readme(name, security_level);
    fs::write(output_path.join("README.md"), readme)?;

    // Create .gitignore
    let gitignore = r#"# Build artifacts
build/
*.wpkg

# Traces
traces/
*.jsonl

# Environment
.env

# OS
.DS_Store
"#;

    fs::write(output_path.join(".gitignore"), gitignore)?;

    spinner.finish_and_clear();

    ui::success(&format!("Initialized agent: {}", name));
    ui::info(&format!("  Security level: {}", security_level));
    ui::info(&format!("  Location: {}/", output_dir));
    println!();

    // Show appropriate next steps
    match security_level {
        "minimal" => {
            ui::info("Next steps (Quick Start):");
            println!("  1. Set your API key: export OPENAI_API_KEY=...");
            println!("  2. Run your agent: wadah run wadah.yaml --prompt \"Hello!\"");
            println!();
            ui::warning("Running in MINIMAL mode - no security restrictions!");
            println!("  → Use --security standard for production");
        }
        "strict" => {
            ui::info("Next steps (Production Ready):");
            println!("  1. Review ToolCaps.json policies");
            println!("  2. Configure budget limits in wadah.yaml");
            println!("  3. Set API key: export OPENAI_API_KEY=...");
            println!("  4. Run: wadah run wadah.yaml --trace logs/audit.jsonl");
        }
        _ => {
            ui::info("Next steps:");
            println!("  1. Edit wadah.yaml to configure your agent");
            println!("  2. Set API key: export OPENAI_API_KEY=...");
            println!("  3. Run: wadah run wadah.yaml --prompt \"Hello!\"");
            println!();
            ui::info("Tip: Use --security minimal for quick experiments");
        }
    }

    Ok(())
}

fn create_minimal_spec(name: &str) -> String {
    format!(
        r#"apiVersion: wadah.ai/v0.1
kind: Agent
metadata:
  name: {}
  version: 0.1.0
  authors: ["Your Name <you@example.com>"]

runtime:
  model:
    provider: openai
    model_id: gpt-4o-mini

# No security policies - permissive mode
# Perfect for quick experiments and development
"#,
        name
    )
}

fn create_standard_spec(name: &str) -> String {
    format!(
        r#"apiVersion: wadah.ai/v0.1
kind: Agent
metadata:
  name: {}
  version: 0.1.0
  description: "A Wadah agent with standard security"
  authors: ["Your Name <you@example.com>"]
  license: Apache-2.0

runtime:
  model:
    provider: openai
    model_id: gpt-4o-mini
    params:
      temperature: 0.2

# Standard security: budget limits only
plugins:
  - id: security.budgets
    enabled: true
    config:
      usd_per_day: 100.0
      max_duration_secs: 3600
"#,
        name
    )
}

fn create_strict_spec(name: &str) -> String {
    format!(
        r#"apiVersion: wadah.ai/v0.1
kind: Agent
metadata:
  name: {}
  version: 0.1.0
  description: "A production-ready Wadah agent"
  authors: ["Your Name <you@example.com>"]
  license: Apache-2.0
  tags: [production, secure]

runtime:
  model:
    provider: openai
    model_id: gpt-4o-mini
    params:
      temperature: 0.2
      max_tokens: 2048
      seed: 1337

# Strict security: all plugins enabled
plugins:
  - id: security.toolcaps
    enabled: true
  - id: security.budgets
    enabled: true
    config:
      tokens_per_minute: 100000
      usd_per_day: 50.0
      max_duration_secs: 1800
  - id: security.network
    enabled: true
  - id: security.filesystem
    enabled: true
  - id: observability.tracing
    enabled: true

policy:
  toolcaps: ToolCaps.json
  budgets:
    tokens_per_minute: 100000
    usd_per_day: 50.0
    max_duration_secs: 1800
  network:
    allow_domains:
      - "api.openai.com"
  filesystem:
    allow_paths:
      - "./workspace"
    read_only: true

artifacts:
  include:
    - "prompts/**"
    - "code/**"
    - "tools/**"
"#,
        name
    )
}

fn create_readme(name: &str, security_level: &str) -> String {
    format!(r#"# {}

A Wadah agent (security: **{}**).

## Quick Start

```bash
# Set your API key
export OPENAI_API_KEY="your-key-here"

# Run the agent
wadah run wadah.yaml --prompt "Hello!"

# Interactive mode
wadah run wadah.yaml --interactive
```

## Package & Distribute

```bash
# Build package
wadah pack -m wadah.yaml -o build/{}.wpkg

# Run packaged agent
wadah run build/{}.wpkg

# Push to registry
wadah push ghcr.io/username/{}:0.1.0
```

## Security Level: {}

{}

## Documentation

- [Wadah Docs](https://wadah.ai/docs)
- [Security Plugins](https://wadah.ai/docs/plugins)
- [Examples](https://github.com/zenri/wadah/tree/main/templates)

## License

Apache 2.0
"#,
    name,
    security_level,
    name,
    name,
    name,
    security_level,
    match security_level {
        "minimal" => "⚠️  **No security restrictions** - Great for development!\n\n- No budget limits\n- No policy enforcement\n- No tracing\n\n**Warning**: Not recommended for production use.",
        "strict" => "✅ **Full security enabled** - Production ready!\n\n- Tool capability restrictions (ToolCaps)\n- Budget and cost controls\n- Network domain whitelisting\n- Filesystem access controls\n- Full execution tracing",
        _ => "🛡️  **Standard security** - Balanced approach.\n\n- Budget limits enabled\n- Basic cost protection\n- No tool restrictions\n\nUpgrade to `--security strict` for production."
    })
}
