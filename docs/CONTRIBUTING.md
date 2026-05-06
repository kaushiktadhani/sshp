# Contributing to sshp

Thank you for considering contributing to sshp! This document provides guidelines for contributing to the project.

## Code of Conduct

Be respectful, inclusive, and constructive in all interactions.

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue with:
- A clear, descriptive title
- Steps to reproduce the behavior
- Expected vs actual behavior
- Your environment (OS, Rust version, terminal)
- Relevant SSH config (with sensitive info removed)

### Suggesting Features

Feature requests are welcome! Please open an issue with:
- A clear description of the feature
- Why it would be useful
- Example use cases

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Test thoroughly
5. Commit with clear messages
6. Push to your fork
7. Open a Pull Request

### Development Setup

```bash
# Clone your fork
git clone https://github.com/kaushiktadhani/sshp.git
cd sshp

# Create a branch
git checkout -b feature/my-feature

# Make changes and test
cargo run

# Build and test
cargo build
cargo test

# Format code
cargo fmt

# Check for issues
cargo clippy
```

## Code Style

- Follow Rust naming conventions
- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Write clear comments for complex logic
- Keep functions focused and small

## Testing

- Test your changes with various SSH configs
- Test on different terminal sizes
- Test keyboard navigation thoroughly
- Test edge cases (empty config, many hosts, long names)

## Commit Messages

Use clear, descriptive commit messages:

```
Add feature: support for multiple SSH configs

- Add --config flag to specify custom config file
- Update documentation
- Add tests
```

## Need Help?

Open an issue with the "question" label if you need help or clarification.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
