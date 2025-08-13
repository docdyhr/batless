# Contributing to batless

First off, thanks for taking the time to contribute! 🎉

The following is a set of guidelines for contributing to batless. These are mostly guidelines, not rules. Use your best judgment, and feel free to propose changes to this document in a pull request.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Issue Guidelines](#issue-guidelines)

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible using our bug report template.

### Suggesting Features

Feature suggestions are welcome! Please use our feature request template and provide:

- A clear description of the feature
- The problem it solves
- Example usage
- Any alternatives you've considered

### Contributing Code

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for your changes
5. Ensure all tests pass
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Git
- A terminal/shell

### Getting Started

1. **Clone the repository**

   ```bash
   git clone https://github.com/your-username/batless.git
   cd batless
   ```

2. **Install dependencies**

   ```bash
   cargo build
   ```

3. **Run tests**

   ```bash
   cargo test
   ```

4. **Run the application**

   ```bash
   cargo run -- --help
   ```

### Development Commands

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run integration tests
cargo test --test integration_tests

# Run coverage (local)
cargo llvm-cov --all-features --workspace report

# Run quality subset
cargo clippy --all-targets --all-features -- -D warnings
cargo machete || true

# Build release version
cargo build --release

# Run with sample file
cargo run -- examples/demo.py --mode=highlight --max-lines=10
```

## Pull Request Process

1. **Before submitting:**
   - Ensure your code compiles without warnings
   - Run `cargo fmt` to format your code
   - Run `cargo clippy` and fix any issues
   - Add tests for new functionality
   - Update documentation if needed

2. **PR Requirements:**
   - Fill out the PR template completely
   - Include a clear description of changes
   - Reference any related issues
   - Ensure CI checks pass

3. **Review Process:**
   - At least one maintainer review is required
   - Address feedback promptly
   - Keep the PR up to date with the main branch

## Coding Standards

### Rust Style

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/style-guide/)
- Use `cargo fmt` for consistent formatting
- Use `cargo clippy` and address all warnings
- Prefer explicit types when it improves readability
- Use meaningful variable and function names

### Code Organization

- Keep functions focused and small
- Use modules to organize related functionality
- Put tests in the same file as the code they test (for unit tests)
- Put integration tests in the `tests/` directory
- Document public APIs with rustdoc comments

### Error Handling

- Use `Result<T, E>` for functions that can fail
- Provide meaningful error messages
- Use `?` operator for error propagation
- Don't use `unwrap()` or `expect()` in library code
- Handle edge cases gracefully

### Performance

- Prefer streaming over loading entire files
- Use appropriate data structures
- Avoid unnecessary allocations
- Profile before optimizing
- Maintain memory efficiency

## Testing Guidelines

### Unit Tests

- Test all public functions
- Test edge cases and error conditions
- Use descriptive test names
- Keep tests focused and independent
- Use the `#[cfg(test)]` attribute for test modules

Example:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_file_basic() {
        // Test implementation
    }
}
```

### Integration Tests

- Test CLI functionality end-to-end
- Test different operating systems in CI
- Test various file types and edge cases
- Use temporary files for test data

### Test Coverage

- Aim for high test coverage
- Focus on critical paths and edge cases
- Run coverage reports locally when possible

## Documentation

### Code Documentation

- Document all public functions and types
- Use rustdoc format (`///` or `/** */`)
- Include examples in documentation
- Explain complex algorithms or business logic

### User Documentation

- Update README.md for user-facing changes
- Add examples for new features
- Update CHANGELOG.md for all changes
- Keep documentation concise but complete

### API Documentation

```rust
/// Process a file according to the given configuration.
///
/// # Arguments
///
/// * `file_path` - Path to the file to process
/// * `config` - Configuration for processing
///
/// # Returns
///
/// Returns `FileInfo` containing processed data and metadata.
///
/// # Errors
///
/// Returns an error if the file cannot be read or processed.
///
/// # Examples
///
/// ```
/// let config = BatlessConfig::default();
/// let info = process_file("src/main.rs", &config)?;
/// ```
pub fn process_file(file_path: &str, config: &BatlessConfig) -> Result<FileInfo, Box<dyn std::error::Error>> {
    // Implementation
}
```

## Issue Guidelines

### Bug Reports

- Use the bug report template
- Provide minimal reproduction steps
- Include environment details
- Attach relevant files if helpful

### Feature Requests

- Use the feature request template
- Explain the problem you're trying to solve
- Provide examples of how it would work
- Consider backwards compatibility

### Questions

- Check existing documentation first
- Use discussions for general questions
- Be specific about what you're trying to achieve

## Release Process

Releases follow semantic versioning:

- **MAJOR** version for incompatible API changes
- **MINOR** version for backwards-compatible functionality
- **PATCH** version for backwards-compatible bug fixes

### Maintainer Release Steps

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create release PR
4. Tag release after merge
5. GitHub Actions handles the rest

## Recognition

Contributors will be recognized in:

- CHANGELOG.md for significant contributions
- README.md contributors section
- Release notes for major features

## Getting Help

- Check the [README](README.md) first
- Look through existing [issues](https://github.com/your-username/batless/issues)
- Start a [discussion](https://github.com/your-username/batless/discussions)
- Join our community channels (if available)

## License

By contributing to batless, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to batless! 🦇
