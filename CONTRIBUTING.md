# Contributing to SecureVault

Thank you for your interest in contributing to SecureVault! We welcome contributions from the community.

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code. Please be respectful and constructive in all interactions.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* Use a clear and descriptive title
* Describe the exact steps to reproduce the problem
* Provide specific examples to demonstrate the steps
* Describe the behavior you observed and what behavior you expected
* Include any error messages or stack traces

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* Use a clear and descriptive title
* Provide a step-by-step description of the suggested enhancement
* Provide specific examples to demonstrate the use case
* Explain why this enhancement would be useful

### Pull Requests

1. Fork the repository and create your branch from `main`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes
5. Make sure your code follows the existing style
6. Issue the pull request

### Development Process

1. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR-USERNAME/securevault.git
   cd securevault
   ```

2. **Create a Branch**
   ```bash
   git checkout -b feature/my-new-feature
   ```

3. **Make Your Changes**
   - Write clean, readable code
   - Follow Rust conventions
   - Add tests for new functionality
   - Update documentation as needed

4. **Test Your Changes**
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

5. **Commit Your Changes**
   ```bash
   git add .
   git commit -m "Add some feature"
   ```

6. **Push to Your Fork**
   ```bash
   git push origin feature/my-new-feature
   ```

7. **Create a Pull Request**
   - Go to the original repository
   - Click "New Pull Request"
   - Select your fork and branch
   - Provide a clear description of your changes

## Coding Standards

### Rust Style Guide

- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- Use `cargo fmt` to format your code
- Use `cargo clippy` to catch common mistakes
- Write idiomatic Rust code

### Code Quality

- All code must pass `cargo test`
- All code must pass `cargo clippy` with no warnings
- All code must be formatted with `cargo fmt`
- New features must include tests
- Public APIs must be documented with rustdoc comments

### Testing

- Write unit tests for all new functionality
- Ensure existing tests still pass
- Aim for high test coverage
- Test edge cases and error conditions

### Documentation

- Document all public APIs with rustdoc
- Include examples in documentation
- Update README.md if adding new features
- Keep documentation up-to-date with code changes

## Commit Messages

- Use clear and descriptive commit messages
- Start with a verb in the imperative mood (e.g., "Add", "Fix", "Update")
- Keep the first line under 72 characters
- Provide additional details in the body if needed

Example:
```
Add transfer functionality between wallets

- Implement transfer method in CustodySystem
- Add validation for transfer amounts
- Add tests for transfer operations
- Update documentation with transfer examples
```

## Review Process

All submissions require review. We use GitHub pull requests for this purpose. Reviewers will:

- Check code quality and style
- Verify tests pass
- Review documentation
- Suggest improvements if needed

## Questions?

Feel free to open an issue with your question or reach out to the maintainers.

## License

By contributing to SecureVault, you agree that your contributions will be licensed under the MIT License.
