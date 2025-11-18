#!/bin/bash
set -euo pipefail

echo "Running linters..."

# Format check
echo "Checking code formatting..."
cargo fmt --all -- --check

# Clippy
echo "Running Clippy..."
cargo clippy --workspace --all-features --all-targets -- -D warnings

echo "Linting complete!"
