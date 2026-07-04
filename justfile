# rubo4e justfile
# Install just: cargo install just  (or brew install just on macOS)

# Default: list available recipes
default:
    @just --list

# Generate Rust code from a BO4E JSON Schema release tag (e.g. v202501.0.0)
generate version="v202501.0.0":
    cargo run -p bo4e-generator -- --schema-version {{ version }}
    @echo "Generation complete. Review changes below:"
    git diff src/generated/

# Download a BO4E JSON Schema release snapshot into generator/schemas/<TAG>/
# Usage: just download-schemas v202501.0.0
download-schemas version:
    bash scripts/download_schemas.sh "{{ version }}"

# Run all tests across the full workspace
test:
    cargo test --workspace

# Run tests with all features enabled
test-all:
    cargo test --workspace --all-features

# Run tests with no default features
test-minimal:
    cargo test --workspace --no-default-features

# Run tests with no default features but versioned schemas enabled (catches struct definition regressions)
test-minimal-versioned:
    cargo test --workspace --no-default-features --features versioned

# Clippy with all features, deny all warnings
lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Format check
fmt-check:
    cargo fmt --all -- --check

# Apply formatting
fmt:
    cargo fmt --all

# Full CI suite (mirrors GitHub Actions)
ci: fmt-check lint test test-minimal test-minimal-versioned check-docs-drift deny-check
    @echo "All CI checks passed."

# Run cargo-deny license/advisory/ban checks
deny-check:
    cargo deny check

# Check with RUSTFLAGS=-D warnings (catches broken examples / cfg-gated items)
check-strict:
    RUSTFLAGS="-D warnings" cargo check --workspace --all-targets --all-features

# Compile and run docs-backed usage examples (single build, all example features)
check-docs-examples:
    cargo build --examples --all-features
    cargo run --example serialize --all-features
    cargo run --example builder --all-features
    cargo run --example migrate --all-features

# Lightweight JSON perf regression gate for simd-json
check-perf-simd:
    bash scripts/check_json_perf_regression.sh
