# Contributing to EdgeFlow

Thank you for your interest in contributing to EdgeFlow! We welcome contributions from the community and are excited to work with you.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Community](#community)

## 📜 Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## 🚀 Getting Started

### Prerequisites

- **Rust 1.75+**: Install from [rustup.rs](https://rustup.rs/)
- **Git**: For version control
- **Docker** (optional): For containerized development

### Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/edgeflow.git
   cd edgeflow
   ```

2. **Install Dependencies**
   ```bash
   # Install Rust toolchain
   rustup update stable
   rustup component add clippy rustfmt
   
   # Build the project
   cargo build
   ```

3. **Run Tests**
   ```bash
   cargo test --workspace
   ```

4. **Start Development Server**
   ```bash
   cargo run -- start --config examples/example.hcl
   ```

## 🤝 How to Contribute

### 🐛 Reporting Bugs

1. **Check existing issues** to avoid duplicates
2. **Use the bug report template** when creating new issues
3. **Provide detailed information**:
   - EdgeFlow version
   - Operating system
   - Rust version
   - Steps to reproduce
   - Expected vs actual behavior
   - Relevant logs or error messages

### 💡 Suggesting Features

1. **Check the roadmap** and existing feature requests
2. **Use the feature request template**
3. **Provide clear use cases** and benefits
4. **Consider implementation complexity**

### 🔧 Code Contributions

We welcome code contributions! Here are the areas where you can help:

#### 🔌 Plugin Development
- Create new plugins for the EdgeFlow ecosystem
- Improve existing plugin functionality
- Add WebAssembly plugin examples

#### 🏗️ Core Features
- Performance optimizations
- New gateway features
- AI/ML integrations
- Security enhancements

#### 📚 Documentation
- API documentation
- Tutorials and guides
- Code examples
- Translation improvements

#### 🧪 Testing
- Unit tests
- Integration tests
- Performance benchmarks
- Edge case testing

## 🔄 Pull Request Process

### 1. Preparation
- Create a feature branch from `main`
- Use descriptive branch names: `feature/ai-security-plugin` or `fix/memory-leak`
- Keep changes focused and atomic

### 2. Development
- Follow our [coding standards](#coding-standards)
- Write tests for new functionality
- Update documentation as needed
- Ensure all tests pass

### 3. Submission
- **Commit Message Format**:
  ```
  type(scope): description
  
  Longer description if needed
  
  Fixes #123
  ```
  
  Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

- **Pull Request Template**:
  - Clear title and description
  - Link to related issues
  - List of changes
  - Testing instructions
  - Screenshots (if applicable)

### 4. Review Process
- Automated checks must pass (CI/CD, tests, linting)
- At least one maintainer review required
- Address feedback promptly
- Squash commits before merging

## 📏 Coding Standards

### Rust Code Style
- Use `rustfmt` for formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Follow Rust naming conventions
- Write idiomatic Rust code

### Code Quality
- **Error Handling**: Use `Result` types, avoid panics
- **Documentation**: Document public APIs with `///`
- **Performance**: Consider performance implications
- **Safety**: Minimize `unsafe` code, justify when used

### Example Code Style
```rust
/// Processes AI inference requests with timeout handling.
/// 
/// # Arguments
/// * `request` - The inference request to process
/// * `timeout` - Maximum processing time in milliseconds
/// 
/// # Returns
/// * `Ok(response)` - Successful inference response
/// * `Err(error)` - Processing error or timeout
pub async fn process_inference(
    request: InferenceRequest,
    timeout: Duration,
) -> Result<InferenceResponse, InferenceError> {
    // Implementation here
}
```

## 🧪 Testing Guidelines

### Test Categories
- **Unit Tests**: Test individual functions and modules
- **Integration Tests**: Test component interactions
- **End-to-End Tests**: Test complete workflows
- **Performance Tests**: Benchmark critical paths

### Running Tests
```bash
# Run all tests
cargo test --workspace

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run performance tests
cargo test --release --test performance
```

### Writing Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_inference_processing() {
        let request = InferenceRequest::new("test prompt");
        let result = process_inference(request, Duration::from_secs(30)).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.content.is_empty());
    }
}
```

## 📖 Documentation

### Documentation Types
- **API Documentation**: Rust doc comments (`///`)
- **User Guides**: Markdown files in `docs/`
- **Examples**: Working code examples in `examples/`
- **README**: Project overview and quick start

### Writing Documentation
- Use clear, concise language
- Provide practical examples
- Include error handling
- Test all code examples
- Update related documentation when making changes

## 🏗️ Project Structure

```
edgeflow/
├── crates/
│   ├── edgeflow-gateway/     # Main gateway service
│   ├── edgeflow-plugins-api/ # Plugin API definitions
│   └── edgeflow-request-id/  # Example plugin
├── docs/                     # Documentation
├── examples/                 # Configuration examples
├── scripts/                  # Build and utility scripts
├── ui/                       # Web dashboard
└── tests/                    # Integration tests
```

## 🌟 Recognition

Contributors are recognized in several ways:
- Listed in `CONTRIBUTORS.md`
- Mentioned in release notes
- GitHub contributor statistics
- Special recognition for significant contributions

## 💬 Community

### Getting Help
- **GitHub Discussions**: For questions and general discussion
- **Discord**: Real-time chat with the community
- **GitHub Issues**: For bug reports and feature requests

### Communication Guidelines
- Be respectful and inclusive
- Use clear, descriptive titles
- Provide context and examples
- Search before posting
- Follow up on your issues and PRs

## 📞 Contact

- **Maintainers**: [@louloulin](https://github.com/louloulin)
- **Email**: contribute@edgeflow.ai
- **Discord**: [EdgeFlow Community](https://discord.gg/edgeflow)

## 📄 License

By contributing to EdgeFlow, you agree that your contributions will be licensed under the same MIT and Apache 2.0 dual license as the project.

---

Thank you for contributing to EdgeFlow! Together, we're building the future of edge AI infrastructure. 🚀
