# rubo4e justfile
# Install just: cargo install just  (or brew install just on macOS)

# Default: list available recipes
default:
    @just --list

# Generate Rust code from a BO4E JSON Schema release tag (e.g. v202607.0.0)
generate version="v202607.0.0":
    cargo run -p bo4e-generator -- --schema-version {{ version }}
    @echo "Generation complete. Review changes below:"
    git diff src/generated/

# Download a BO4E JSON Schema release snapshot into generator/schemas/<TAG>/
# Usage: just download-schemas v202607.0.0
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

# Lint the feature combinations that `--all-features` cannot reach.
#
# `--all-features` hides two whole classes of defect: code that is dead unless an
# optional dependency is enabled (e.g. the simd-json-only depth pre-scan), and
# bindings that go unread when a feature compiles a function body away (e.g. the
# `time`/`decimal` validators).  Both shipped undetected before this recipe existed.
lint-features:
    #!/usr/bin/env bash
    set -euo pipefail
    combos=(
        # Each feature on its own — `time` alone once failed to build outright.
        "serde" "json" "time" "decimal" "builder" "validate" "schemars"
        "sqlx" "utoipa" "strum" "tracing" "metrics" "identifiers" "versioned"
        # Realistic combinations.
        "versioned,json"
        "versioned,json,time,decimal"
        "versioned,validate"
        "versioned,builder,validate"
        "versioned,json,schemars,utoipa"
        "versioned,json,simd-json"
        "versioned,sqlx"
        "versioned,json,sqlx"
        "versioned,strum,tracing,metrics"
        "json,time,decimal,builder,validate,schemars,utoipa,strum"
    )
    for f in "${combos[@]}"; do
        echo "==> --no-default-features --features $f"
        RUSTFLAGS="-D warnings" cargo check --workspace --all-targets \
            --no-default-features --features "$f"
    done
    echo "All feature combinations compile without warnings."

# Rustdoc build with broken intra-doc links denied
check-docs:
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features

# Verify the crate still builds on its declared MSRV (Cargo.toml rust-version)
check-msrv:
    #!/usr/bin/env bash
    set -euo pipefail
    msrv="$(sed -n 's/^rust-version *= *"\(.*\)"/\1/p' Cargo.toml | head -1)"
    echo "Declared MSRV: $msrv"
    rustup toolchain install "$msrv" --profile minimal
    cargo "+$msrv" check --workspace --all-features

# Guard against runaway code generation
check-codegen-size:
    bash scripts/check_codegen_size.sh

# ─── Documentation site (site/, built with Zola) ─────────────────────────────

# Serve the site locally with live reload
site-serve:
    cd site && zola serve

# Build the site and verify every internal link resolves
site-build:
    cd site && zola check --skip-external-links && zola build

# Build the site and check external links too (slow; needs network)
site-check-links:
    cd site && zola check

# Format check
fmt-check:
    cargo fmt --all -- --check

# Apply formatting
fmt:
    cargo fmt --all

# Full CI suite (mirrors GitHub Actions)
#
# Recipe order tracks .github/workflows/ci.yml.  `test-all` rather than `test`:
# the workflow's primary test job runs --all-features, and the default-feature
# run alone leaves most of the crate uncompiled.  `check-msrv` is deliberately
# omitted — it installs a second toolchain, which is CI's job, not a local
# pre-push gate; run it explicitly before a release.
ci: fmt-check lint lint-features check-strict check-docs test-all test-minimal test-minimal-versioned check-docs-examples check-codegen-size check-docs-drift site-build deny-check
    @echo "All CI checks passed."

# Fail only if regenerating changes the generated output (true drift), regardless
# of any other uncommitted edits in the working tree.  Snapshots src/generated/,
# regenerates + formats, and compares against the snapshot — so it passes on an
# already-regenerated (but uncommitted) tree and fails only when the on-disk code
# genuinely differs from what the generator + schemas produce.
check-docs-drift:
    #!/usr/bin/env bash
    set -euo pipefail
    snapshot="$(mktemp -d)"
    trap 'rm -rf "$snapshot"' EXIT
    cp -R src/generated "$snapshot/before"
    cargo run -p bo4e-generator -- --schema-version v202607.0.0
    cargo fmt --all
    if diff -rq "$snapshot/before" src/generated >/dev/null; then
        echo "src/generated/ is in sync with the generator."
    else
        echo "DRIFT: regenerating changed src/generated/. Review and commit the update:"
        diff -ru "$snapshot/before" src/generated || true
        exit 1
    fi

# Run cargo-deny license/advisory/ban checks
#
# `--all-features` because that is what `cargo-deny-action` defaults to in CI.
# Without it this recipe checks only the default feature set, which leaves the
# optional dependencies (sqlx, simd-json, utoipa, …) out of the graph entirely —
# a duplicate-version ban that CI rejected passed here for exactly that reason.
deny-check:
    cargo deny --all-features check

# Check with RUSTFLAGS=-D warnings (catches broken examples / cfg-gated items)
check-strict:
    RUSTFLAGS="-D warnings" cargo check --workspace --all-targets --all-features

# Compile and run docs-backed usage examples (single build, all example features)
check-docs-examples:
    cargo build --examples --all-features
    cargo run --example serialize --features versioned,json,decimal
    cargo run --example builder --features versioned,builder,json,decimal

# Lightweight JSON perf regression gate for simd-json
check-perf-simd:
    bash scripts/check_json_perf_regression.sh
