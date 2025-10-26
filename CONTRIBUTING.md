# Contributing to Wadah

Thank you for your interest in contributing to Wadah! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful, inclusive, and professional. We're building this together.

## Ways to Contribute

1. **Bug Reports**: Found an issue? Open a GitHub issue
2. **Feature Requests**: Have an idea? Start a discussion
3. **Code**: Submit pull requests for fixes or features
4. **Documentation**: Improve docs, examples, or tutorials
5. **Templates**: Create and share agent templates
6. **Testing**: Write tests, try edge cases
7. **Reviews**: Review PRs and provide feedback

## Getting Started

### Development Environment

```bash
# Clone repository
git clone https://github.com/zenri/wadah
cd wadah

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build project
cargo build

# Run tests
cargo test --all

# Install dev tools
cargo install cargo-watch cargo-nextest
```

### Project Structure

```
wadah/
├── crates/
│   ├── cli/          # CLI binary
│   ├── runtime/      # Execution engine
│   ├── spec/         # WadahSpec & ToolCaps
│   ├── pack/         # .wpkg packaging
│   ├── trace/        # OAT tracing
│   └── oci/          # OCI push/pull
├── templates/        # Reference agents
├── docs/             # Documentation
└── examples/         # Code examples
```

## Development Workflow

### 1. Pick an Issue

- Check [GitHub Issues](https://github.com/zenri/wadah/issues)
- Comment to claim an issue
- Ask questions if unclear

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-123
```

### 3. Make Changes

```bash
# Edit code
vim crates/runtime/src/adapters/mod.rs

# Build and test frequently
cargo watch -x build -x test

# Format code
cargo fmt --all

# Check lints
cargo clippy --all-targets --all-features
```

### 4. Write Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_your_feature() {
        let result = your_function();
        assert!(result.is_ok());
    }
}
```

### 5. Update Documentation

- Update relevant `.md` files in `docs/`
- Add inline doc comments: `///`
- Update CHANGELOG.md

### 6. Commit Changes

```bash
git add .
git commit -m "feat: add new model adapter for Anthropic"

# Commit message format:
# <type>: <description>
#
# Types: feat, fix, docs, style, refactor, test, chore
```

### 7. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then open a Pull Request on GitHub.

## Pull Request Guidelines

### PR Title

Use conventional commits format:

```
feat: Add Anthropic Claude adapter
fix: Resolve race condition in trace recorder
docs: Update ToolCaps examples
test: Add integration tests for pack crate
```

### PR Description

Include:

1. **What**: What does this PR do?
2. **Why**: Why is this change needed?
3. **How**: How does it work?
4. **Testing**: How did you test it?
5. **Screenshots**: If UI changes

Example:

```markdown
## What
Adds support for Anthropic Claude models via new adapter.

## Why
Users requested Claude support for cost optimization.

## How
- Implements `ModelAdapter` trait for Claude API
- Adds authentication via `ANTHROPIC_API_KEY`
- Handles streaming and non-streaming responses

## Testing
- Unit tests for adapter
- Manual testing with Claude 3 Opus
- Verified with example agent

## Checklist
- [x] Tests passing
- [x] Documentation updated
- [x] CHANGELOG.md updated
```

### PR Checklist

- [ ] Code compiles without warnings
- [ ] All tests pass: `cargo test --all`
- [ ] New code has tests
- [ ] Documentation updated
- [ ] CHANGELOG.md updated (for user-facing changes)
- [ ] Formatted: `cargo fmt --all`
- [ ] Linted: `cargo clippy --all-targets`

## Coding Standards

### Rust Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Prefer explicit error types over `Box<dyn Error>`
- Use `thiserror` for error definitions

### Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Something went wrong: {0}")]
    SomethingWrong(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, MyError>;
```

### Async/Await

```rust
use tokio;

#[tokio::test]
async fn test_async_function() {
    let result = my_async_function().await;
    assert!(result.is_ok());
}
```

### Documentation

```rust
/// Executes an agent with the given prompt.
///
/// # Arguments
///
/// * `prompt` - The user input prompt
///
/// # Returns
///
/// Returns the agent's response text or an error.
///
/// # Example
///
/// ```no_run
/// let response = executor.execute("Hello".to_string()).await?;
/// ```
pub async fn execute(&mut self, prompt: String) -> Result<String> {
    // ...
}
```

## Testing

### Unit Tests

```bash
# Run all tests
cargo test --all

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Integration Tests

```bash
# Place in tests/ directory
cargo test --test integration_test
```

### Benchmarks

```bash
cargo bench
```

## Creating Templates

When adding new agent templates:

1. Create directory in `templates/your-template/`
2. Include:
   - `wadah.yaml` - Agent manifest
   - `ToolCaps.json` - Policies
   - `README.md` - Usage guide
   - `prompts/system.txt` - System prompt
3. Test thoroughly
4. Document use cases

## Documentation

### Writing Docs

- Clear, concise language
- Include code examples
- Provide context and motivation
- Link to related docs

### Building Docs

```bash
cargo doc --no-deps --open
```

## Release Process

(For maintainers)

1. Update version in `Cargo.toml` files
2. Update `CHANGELOG.md`
3. Create git tag: `git tag -a v0.1.0 -m "Release v0.1.0"`
4. Push tag: `git push origin v0.1.0`
5. CI builds and publishes to crates.io

## Getting Help

- **GitHub Discussions**: Ask questions, share ideas
- **Discord**: Real-time chat (link in README)
- **Issues**: Bug reports and feature requests

## Recognition

Contributors will be:
- Listed in CHANGELOG.md
- Mentioned in release notes
- Added to CONTRIBUTORS.md (if substantial contribution)

## License

By contributing, you agree that your contributions will be licensed under Apache 2.0.

---

Thank you for contributing to Wadah! 🌊

