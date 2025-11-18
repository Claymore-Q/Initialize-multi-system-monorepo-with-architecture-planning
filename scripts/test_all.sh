#!/bin/bash
set -euo pipefail

echo "Running all tests..."

# Unit tests
echo "Running unit tests..."
cargo test --workspace --all-features --lib --verbose

# Integration tests
echo "Running integration tests..."
cargo test --workspace --all-features --test '*' --verbose

# Doc tests
echo "Running doc tests..."
cargo test --workspace --all-features --doc --verbose

echo "All tests passed!"
