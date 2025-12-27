# Contributing to Spectra

Thank you for your interest in contributing to Spectra! This document provides guidelines and protocols for contributing to the project.

## Core Principles

Spectra adheres to two non-negotiable principles:

1. **Performance First**: If it slows down the scan, it doesn't get merged.
2. **Safety First**: Code must be secure, tested, and not introduce vulnerabilities.

## Getting Started

### Prerequisites

- **Rust**: 1.70+ (stable channel recommended)
- **Node.js**: 20 LTS (for frontend development)
- **Git**: For version control
- **Cargo**: Comes with Rust installation

### Setting Up Your Development Environment

1. **Fork and Clone**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/spectra.git
   cd spectra
   ```

2. **Build the Project**:
   ```bash
   # Build all workspace crates
   cargo build

   # Build with release optimizations
   cargo build --release
   ```

3. **Run Tests**:
   ```bash
   # Run all tests
   cargo test --workspace

   # Run specific crate tests
   cargo test -p spectra-core
   cargo test -p spectra-cli
   ```

4. **Set Up Frontend** (for GUI development):
   ```bash
   cd app
   npm ci --legacy-peer-deps
   npm run tauri dev
   ```

## Development Workflow

### Before You Start

1. **Check existing issues**: Look for related issues or feature requests
2. **Open a discussion**: For major changes, open an issue first to discuss
3. **Create a branch**: Use descriptive branch names (e.g., `feat/add-compression-detection`, `fix/entropy-calculation`)

### Code Quality Standards

#### Rust Code

All Rust code must pass the following checks:

```bash
# Format check
cargo fmt --all -- --check

# Linting with strict warnings
cargo clippy --workspace -- -D warnings

# Compilation check
cargo check --workspace

# Test suite
cargo test --workspace
```

**Tip**: Run `validate-refactor.bat` (Windows) to run all checks at once.

#### Frontend Code

All frontend code must pass:

```bash
cd app

# Type checking
npx tsc --noEmit

# Build check
npm run build
```

### Coding Standards

#### Rust

- **Follow Rust conventions**: Use `cargo fmt` and `cargo clippy`
- **Error handling**: Use `anyhow` for CLI, `Result<T, E>` for libraries
- **Documentation**: Add doc comments for public APIs
- **Tests**: Write unit tests for new functionality
- **Performance**: Benchmark performance-critical code
- **No unsafe code**: Unless absolutely necessary and well-documented

Example:
```rust
/// Calculates Shannon entropy for the given byte slice.
///
/// # Arguments
/// * `data` - Byte slice to analyze
///
/// # Returns
/// Entropy value between 0.0 (uniform) and 8.0 (maximum randomness)
pub fn calculate_entropy(data: &[u8]) -> f64 {
    // Implementation
}
```

#### TypeScript

- **Strict mode**: All code must pass TypeScript strict checks
- **Type safety**: Avoid `any` types, use proper interfaces
- **Components**: Follow React best practices and hooks patterns
- **Tests**: Write component tests for new UI features

### Security Guidelines

- **No credential leaks**: Never commit API keys, passwords, or tokens
- **Input validation**: Validate all user input and file paths
- **Avoid common vulnerabilities**:
  - No SQL injection (use parameterized queries)
  - No command injection (sanitize shell inputs)
  - No path traversal (validate file paths)
  - No XSS in frontend (sanitize user content)

### Performance Guidelines

- **Benchmark before and after**: Use `cargo bench` for performance-critical changes
- **Profile when necessary**: Use profiling tools to identify bottlenecks
- **Memory efficiency**: Minimize allocations in hot paths
- **Parallel processing**: Leverage Rayon or jwalk for I/O operations
- **Test at scale**: Ensure changes work with 1M+ files

## Making Changes

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples**:
```
feat(core): Add parallel directory scanning with jwalk

Implements multi-threaded scanning using jwalk for improved
performance on large file systems.

Closes #42
```

```
fix(cli): Handle permission errors gracefully

Previously, the CLI would crash on permission denied errors.
Now it logs the error and continues scanning.
```

### Pull Request Process

1. **Update documentation**: Update README.md, CHANGELOG.md, and relevant docs
2. **Add tests**: Ensure new features have test coverage
3. **Run CI locally**: Ensure all CI checks pass before pushing
4. **Update CHANGELOG**: Add your changes to the "Unreleased" section
5. **Create PR**: Provide clear description of changes and motivation
6. **Respond to feedback**: Address review comments promptly

#### PR Template

```markdown
## Description
Brief description of the changes

## Motivation
Why is this change needed?

## Changes
- List of specific changes made

## Testing
How was this tested?

## Checklist
- [ ] Tests pass locally (`cargo test --workspace`)
- [ ] Code formatted (`cargo fmt --all -- --check`)
- [ ] Clippy passes (`cargo clippy --workspace -- -D warnings`)
- [ ] Frontend builds (`cd app && npm run build`)
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
```

## CI/CD Pipeline

All pull requests must pass automated CI checks:

### Rust CI
- **Code formatting**: `cargo fmt --all -- --check`
- **Linting**: `cargo clippy --workspace -- -D warnings`
- **Compilation**: `cargo check --workspace`
- **Tests**: `cargo test --workspace` on Ubuntu, Windows, and macOS

### Frontend CI
- **Dependencies**: `npm ci --legacy-peer-deps`
- **Type checking**: `npx tsc --noEmit`
- **Build**: `npm run build`

CI failures will block merging. Fix all issues before requesting review.

## Testing

### Writing Tests

#### Unit Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_calculation() {
        let data = b"AAAAAAAA";
        let entropy = calculate_entropy(data);
        assert!(entropy < 1.0); // Low entropy for uniform data
    }
}
```

#### Integration Tests (Rust)

Place integration tests in `tests/` directory:

```rust
// tests/integration_test.rs
use spectra_core::Scanner;

#[test]
fn test_full_scan_workflow() {
    let scanner = Scanner::new("./test_data", 10);
    let stats = scanner.scan().unwrap();
    assert!(stats.total_files > 0);
}
```

#### Component Tests (React)

```typescript
import { render, screen } from '@testing-library/react';
import { describe, test, expect } from 'vitest';

describe('RiskTreemap', () => {
  test('renders treemap with nodes', () => {
    render(<RiskTreemap data={mockData} />);
    expect(screen.getByText('safe.txt')).toBeInTheDocument();
  });
});
```

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p spectra-core

# Specific test
cargo test test_entropy_calculation

# With output
cargo test -- --nocapture

# Frontend tests (when vitest is configured)
cd app
npm test
```

## Documentation

### Code Documentation

- Add doc comments to all public APIs
- Use examples in doc comments where helpful
- Run `cargo doc --open` to preview documentation

### Project Documentation

Update relevant documentation files:
- `README.md`: Project overview and quick start
- `CHANGELOG.md`: Version history
- `ARCHITECTURE.md`: Technical architecture
- `CONTRIBUTING.md`: This file
- `FAQ.md`: Common questions and answers

## Release Process

(Maintainers only)

1. Update version in all `Cargo.toml` files
2. Update `CHANGELOG.md` with release date
3. Create git tag: `git tag -a v0.x.0 -m "Release v0.x.0"`
4. Push tag: `git push origin v0.x.0`
5. Create GitHub release with changelog notes

## Getting Help

- **Questions**: Open a GitHub Discussion
- **Bugs**: Open a GitHub Issue with reproduction steps
- **Feature Requests**: Open a GitHub Issue with use case
- **Security Issues**: Email maintainers directly (see SECURITY.md)

## Code of Conduct

Be respectful and constructive in all interactions. We're building something useful together.

## License

Spectra is dual-licensed under Apache-2.0 and MIT.

### Your Contributions

By contributing to this project, you agree that your contributions will be dual-licensed under both:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

### Why Dual License?

This dual licensing provides:
- **Flexibility**: Users can choose the license that best fits their needs
- **Compatibility**: MIT is compatible with almost all licenses; Apache-2.0 provides explicit patent grants
- **Industry Standard**: Follows the precedent set by Rust and many Rust ecosystem projects

For more information about the licensing model, see the [LICENSE](LICENSE) file.

---

Thank you for contributing to Spectra! Every contribution, no matter how small, helps make enterprise data governance more accessible and powerful.
