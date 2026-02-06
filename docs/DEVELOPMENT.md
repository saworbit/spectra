# Development Guide

This guide covers the development setup, workflows, and best practices for Spectra contributors and maintainers.

## Table of Contents

- [Environment Setup](#environment-setup)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Testing](#testing)
- [Debugging](#debugging)
- [Performance Profiling](#performance-profiling)
- [Release Process](#release-process)

## Environment Setup

### Required Tools

1. **Rust Toolchain** (1.70+)
   ```bash
   # Install via rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Update to latest stable
   rustup update stable

   # Set stable as default
   rustup default stable
   ```

2. **Node.js** (20 LTS)
   ```bash
   # Windows (using Chocolatey)
   choco install nodejs-lts

   # macOS (using Homebrew)
   brew install node@20

   # Linux (using nvm)
   nvm install 20
   nvm use 20
   ```

3. **Git**
   ```bash
   # Verify installation
   git --version
   ```

### Optional Tools

- **Cargo Watch**: Auto-rebuild on file changes
  ```bash
  cargo install cargo-watch
  ```

- **Cargo Expand**: Inspect macro expansions
  ```bash
  cargo install cargo-expand
  ```

- **Cargo Flamegraph**: Performance profiling
  ```bash
  cargo install flamegraph
  ```

- **Cargo Audit**: Security vulnerability scanner
  ```bash
  cargo install cargo-audit
  ```

- **VSCode Extensions**:
  - rust-analyzer (Rust language server)
  - CodeLLDB (Rust debugger)
  - Tauri (Tauri app development)
  - ESLint (JavaScript linting)

### Initial Setup

1. **Clone the Repository**
   ```bash
   git clone https://github.com/saworbit/spectra.git
   cd spectra
   ```

2. **Build All Crates**
   ```bash
   cargo build --workspace
   ```

3. **Setup Frontend**
   ```bash
   cd app
   npm ci --legacy-peer-deps
   cd ..
   ```

4. **Run Tests**
   ```bash
   cargo test --workspace
   ```

5. **Verify Installation**
   ```bash
   # Run CLI
   cargo run -p spectra-cli -- --help

   # Run GUI
   cd app && npm run tauri dev
   ```

## Project Structure

```
spectra/
├── .github/
│   └── workflows/          # CI/CD workflows
│       ├── rust-ci.yml     # Rust testing pipeline
│       └── frontend-ci.yml # Frontend testing pipeline
├── spectra-core/           # Core scanning library
│   ├── src/
│   │   └── lib.rs         # Scanner, FileRecord, ScanStats
│   ├── Cargo.toml
│   └── README.md
├── cli/                    # CLI application (spectra-cli)
│   ├── src/
│   │   ├── main.rs        # CLI entry point
│   │   ├── analysis/      # Phase 2: Semantic analysis
│   │   │   ├── entropy.rs
│   │   │   ├── heuristics.rs
│   │   │   ├── semantic.rs
│   │   │   └── mod.rs
│   │   └── governance/    # Phase 3: Policy engine
│   │       ├── engine.rs
│   │       ├── tests.rs
│   │       └── mod.rs
│   ├── Cargo.toml
│   └── USAGE.md
├── server/                 # Spectra Server (Hub)
│   ├── src/
│   │   └── main.rs        # Axum REST API
│   └── Cargo.toml
├── app/                    # Tauri + React GUI
│   ├── src/               # React frontend
│   │   ├── components/
│   │   │   ├── RiskTreemap.tsx
│   │   │   └── __tests__/
│   │   ├── types.ts
│   │   ├── main.tsx
│   │   └── App.tsx
│   ├── src-tauri/         # Rust backend
│   │   ├── src/
│   │   │   └── lib.rs     # Tauri commands
│   │   └── Cargo.toml
│   ├── package.json
│   └── README.md
├── Cargo.toml              # Workspace manifest
├── ARCHITECTURE.md         # Architecture deep-dive
├── CHANGELOG.md           # Version history
├── CONTRIBUTING.md        # Contribution guide
├── DEVELOPMENT.md         # This file
├── FAQ.md                 # Frequently asked questions
└── README.md              # Project overview
```

### Crate Dependencies

```
spectra-server (independent)

spectra-cli
  └── spectra-core

app (Tauri)
  └── spectra-core
```

## Development Workflow

### Daily Development

1. **Start Development Server (GUI)**
   ```bash
   cd app
   npm run tauri dev
   ```

   This starts:
   - Vite dev server with HMR (hot module replacement)
   - Tauri window with live reload
   - Rust compilation watch

2. **Watch Mode (CLI)**
   ```bash
   cargo watch -x 'run -p spectra-cli -- --path ./'
   ```

   Auto-rebuilds on file changes.

3. **Iterative Testing**
   ```bash
   # Run tests on file change
   cargo watch -x test

   # Run specific test
   cargo watch -x 'test test_name'
   ```

### Code Quality Checks

Run before committing:

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --workspace -- -D warnings

# Run all tests
cargo test --workspace

# Security audit
cargo audit

# Frontend checks
cd app
npx tsc --noEmit
npm run build
npm test
```

**Windows**: Use `validate-refactor.bat` to run all checks at once.

### Feature Development

1. **Create Feature Branch**
   ```bash
   git checkout -b feat/your-feature-name
   ```

2. **Implement Feature**
   - Write code following Rust conventions
   - Add unit tests for new functionality
   - Update documentation

3. **Test Locally**
   ```bash
   cargo test --workspace
   cargo clippy --workspace -- -D warnings
   ```

4. **Update Documentation**
   - Add to CHANGELOG.md (Unreleased section)
   - Update README.md if user-facing
   - Add doc comments to public APIs

5. **Commit Changes**
   ```bash
   git add .
   git commit -m "feat: description of feature"
   ```

6. **Push and Create PR**
   ```bash
   git push origin feat/your-feature-name
   ```

## Testing

### Unit Tests

Located in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_creation() {
        let scanner = Scanner::new("/tmp", 10);
        assert_eq!(scanner.limit, 10);
    }
}
```

Run:
```bash
# All unit tests
cargo test --workspace

# Specific crate
cargo test -p spectra-core

# Specific test
cargo test test_scanner_creation

# Show output
cargo test -- --nocapture
```

### Integration Tests

Located in `tests/` directory:

```rust
// tests/integration_test.rs
use spectra_core::Scanner;

#[test]
fn test_full_workflow() {
    let scanner = Scanner::new("./test_data", 100);
    let stats = scanner.scan().unwrap();
    assert!(stats.total_files > 0);
}
```

Run:
```bash
cargo test --test integration_test
```

### Component Tests (React)

Located in `__tests__/` directories:

```typescript
describe('RiskTreemap', () => {
  test('renders correctly', () => {
    render(<RiskTreemap data={mockData} />);
    expect(screen.getByText('test')).toBeInTheDocument();
  });
});
```

Run frontend tests:
```bash
cd app
npm test           # Single run
npm run test:watch # Watch mode
```

### Test Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --workspace --out Html
```

## Debugging

### Rust Debugging (VSCode)

1. **Install CodeLLDB Extension**

2. **Create `.vscode/launch.json`**:
   ```json
   {
     "version": "0.2.0",
     "configurations": [
       {
         "type": "lldb",
         "request": "launch",
         "name": "Debug CLI",
         "cargo": {
           "args": ["build", "-p", "spectra-cli"]
         },
         "args": ["--path", "./"],
         "cwd": "${workspaceFolder}"
       }
     ]
   }
   ```

3. **Set Breakpoints and Press F5**

### Frontend Debugging

1. **Browser DevTools**:
   - Right-click in Tauri window
   - Select "Inspect Element"
   - Use React DevTools extension

2. **Console Logging**:
   ```typescript
   console.log('Debug:', data);
   ```

3. **Tauri DevTools**:
   ```bash
   npm run tauri dev -- --debug
   ```

### Debug Logging

Enable debug logs:

```bash
# Rust (using env_logger or tracing)
RUST_LOG=debug cargo run -p spectra-cli -- --path ./

# Tauri
RUST_LOG=tauri=debug npm run tauri dev
```

## Performance Profiling

### Benchmarking

Create benchmarks in `benches/`:

```rust
// benches/scan_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use spectra_core::Scanner;

fn scan_benchmark(c: &mut Criterion) {
    c.bench_function("scan 1000 files", |b| {
        b.iter(|| {
            let scanner = Scanner::new(black_box("./test_data"), 100);
            scanner.scan().unwrap()
        });
    });
}

criterion_group!(benches, scan_benchmark);
criterion_main!(benches);
```

Run:
```bash
cargo bench
```

### Flamegraph Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph -p spectra-cli -- --path ./large_directory

# Opens flamegraph.svg
```

### Memory Profiling

```bash
# Using valgrind (Linux)
valgrind --tool=massif target/release/spectra-cli --path ./

# Analyze results
ms_print massif.out.*
```

## CI/CD Pipeline

### GitHub Actions Workflows

**Rust CI** (`.github/workflows/rust-ci.yml`):
- Runs on: push to main, pull requests
- Platforms: Ubuntu, Windows, macOS
- Checks: format, clippy, build, test

**Frontend CI** (`.github/workflows/frontend-ci.yml`):
- Runs on: push to main, pull requests
- Checks: type-check, build

### Running CI Locally

Simulate CI checks:

```bash
# Rust checks
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo check --workspace
cargo test --workspace

# Frontend checks
cd app
npm ci --legacy-peer-deps
npx tsc --noEmit
npm run build
```

## Release Process

(Maintainers only)

### Pre-Release Checklist

- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated with release notes
- [ ] Version bumped in all Cargo.toml files
- [ ] Frontend version bumped in package.json

### Version Bump

1. **Update Cargo.toml Files**:
   ```toml
   # Update version in:
   # - Cargo.toml (workspace)
   # - spectra-core/Cargo.toml
   # - cli/Cargo.toml
   # - server/Cargo.toml
   # - app/src-tauri/Cargo.toml
   ```

2. **Update package.json**:
   ```json
   {
     "version": "0.x.0"
   }
   ```

3. **Update CHANGELOG.md**:
   ```markdown
   ## [0.x.0] - YYYY-MM-DD
   (Move Unreleased items here)
   ```

### Create Release

```bash
# Commit version changes
git add .
git commit -m "chore: bump version to v0.x.0"

# Create tag
git tag -a v0.x.0 -m "Release v0.x.0"

# Push changes and tag
git push origin main
git push origin v0.x.0
```

### Build Release Binaries

```bash
# Windows
build-release.bat

# Unix
cargo build --release --workspace

# Binaries in target/release/:
# - spectra-cli.exe (or spectra-cli)
# - spectra-server.exe (or spectra-server)
```

### Create GitHub Release

1. Go to GitHub Releases
2. Click "Draft a new release"
3. Choose tag: v0.x.0
4. Title: "Release v0.x.0"
5. Description: Copy from CHANGELOG.md
6. Attach release binaries
7. Publish release

## Tips & Best Practices

### Performance

- Profile before optimizing
- Use `cargo bench` for micro-benchmarks
- Test with realistic data sizes (1M+ files)
- Minimize allocations in hot paths
- Use parallel iterators (Rayon) judiciously

### Code Quality

- Run `cargo clippy` frequently
- Enable `#![warn(missing_docs)]` for libraries
- Write doc tests in documentation
- Keep functions small and focused
- Use type system for invariants

### Git Workflow

- Small, focused commits
- Descriptive commit messages
- Rebase before merging (if team policy)
- Squash WIP commits
- Reference issues in commits

### Documentation

- Document public APIs thoroughly
- Include examples in doc comments
- Keep README.md updated
- Update ARCHITECTURE.md for design changes
- Add entries to CHANGELOG.md

## Common Issues

### Build Errors

**Error: "libtorch not found"**
```bash
# Don't build with semantic feature
cargo build --release -p spectra-cli
# (without --features semantic)
```

**Error: "linking with `cc` failed"**
```bash
# Update toolchain
rustup update stable

# Clean and rebuild
cargo clean
cargo build
```

### Frontend Issues

**Error: "Cannot find module '@testing-library/react'"**
- Run `npm install` in the `app/` directory to install test dependencies
- Test dependencies (`vitest`, `@testing-library/react`, `@testing-library/jest-dom`, `jsdom`) are in `devDependencies`

**Error: "ERESOLVE peer dependency"**
```bash
# Use legacy peer deps
npm ci --legacy-peer-deps
```

### Performance Issues

**Slow tests**
```bash
# Run in release mode
cargo test --release
```

**Slow builds**
```bash
# Use sccache for caching
cargo install sccache
export RUSTC_WRAPPER=sccache
```

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Tauri Documentation](https://tauri.app/v2/)
- [React Documentation](https://react.dev/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) (unsafe Rust)

## Getting Help

- **Documentation**: Check README.md, ARCHITECTURE.md, FAQ.md
- **Issues**: GitHub Issues for bugs
- **Discussions**: GitHub Discussions for questions
- **Community**: Discord (link coming soon)

---

Happy coding! If you have suggestions for improving this guide, please submit a PR.
