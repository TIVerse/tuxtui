# Contributing to tuxtui

Thank you for your interest in contributing to tuxtui! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to a Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to eshanized@proton.me.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check existing issues. When creating a bug report, include:

- A clear, descriptive title
- Detailed steps to reproduce
- Expected vs. actual behavior
- Environment information (OS, Rust version, tuxtui version)
- Code samples or test cases

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- A clear, descriptive title
- A detailed description of the proposed functionality
- Examples of how the enhancement would be used
- Any relevant examples from other libraries

### Pull Requests

1. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/tuxtui
   cd tuxtui
   ```

2. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make Changes**
   - Write clear, focused commits
   - Follow the coding style (run `cargo fmt`)
   - Add tests for new functionality
   - Update documentation as needed

4. **Test Your Changes**
   ```bash
   cargo xtask check  # Runs fmt, clippy, tests, and doc
   ```

5. **Commit with Conventional Commits**
   ```
   feat: add new widget
   fix: resolve layout bug
   docs: update README
   test: add widget tests
   refactor: simplify buffer logic
   ```

6. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   ```

## Development Setup

### Prerequisites

- Rust 1.85.0 or later
- cargo-deny (optional): `cargo install cargo-deny`
- cargo-tarpaulin (optional, for coverage): `cargo install cargo-tarpaulin`

### Building

```bash
cargo build --workspace
```

### Running Tests

```bash
cargo test --workspace
```

### Running Examples

```bash
cargo run --example hello_world
```

### Developer Tasks

Use the `xtask` utility for common tasks:

```bash
cargo xtask fmt      # Format code
cargo xtask lint     # Run clippy
cargo xtask test     # Run tests
cargo xtask doc      # Build docs
cargo xtask check    # Run all checks
```

## Coding Standards

### Rust Style

- Follow the [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings

### Code Organization

- Keep functions small and focused
- Use descriptive variable names
- Add inline comments for complex logic
- Group related functionality into modules

### Documentation

- Document all public APIs with doc comments
- Include examples in doc comments where helpful
- Use `///` for item documentation
- Use `//!` for module documentation

### Testing

- Write unit tests for new functionality
- Use property-based tests where appropriate (with `proptest`)
- Add integration tests for complex features
- Include snapshot tests for widget rendering (with `insta`)

### Performance

- Avoid unnecessary allocations
- Use `&str` instead of `String` where possible
- Leverage `Cow` for flexible ownership
- Profile code for hotspots before optimizing

## Commit Message Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat(widgets): add tree widget
fix(layout): correct overlap calculation
docs: update widget examples
test: add paragraph wrapping tests
```

## Pull Request Process

1. **Ensure CI Passes**: All tests, lints, and checks must pass
2. **Update Documentation**: Update README, docs, and examples if needed
3. **Add Changelog Entry**: Note your changes (automated via release-plz)
4. **Request Review**: Maintainers will review your PR
5. **Address Feedback**: Make requested changes
6. **Squash if Needed**: Maintainers may ask you to squash commits

## Project Structure

```
tuxtui/
├─ crates/
│  ├─ tuxtui/              # Main crate
│  ├─ tuxtui-core/         # Core types
│  ├─ tuxtui-widgets/      # Widgets
│  ├─ tuxtui-crossterm/    # Crossterm backend
│  ├─ tuxtui-termion/      # Termion backend
│  ├─ tuxtui-termwiz/      # Termwiz backend
│  └─ tuxtui-macros/       # Macros
├─ examples/               # Example applications
├─ xtask/                  # Developer tasks
└─ .github/                # CI/CD workflows
```

## Release Process

Releases are automated via `release-plz`:
- Conventional commits trigger version bumps
- CHANGELOGs are generated automatically
- PRs are created for releases

## Getting Help

- Open an issue for questions
- Check existing documentation
- Review examples in the `examples/` directory

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to tuxtui! 🎉
