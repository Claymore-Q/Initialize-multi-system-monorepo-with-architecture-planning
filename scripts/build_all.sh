#!/bin/bash
set -euo pipefail

echo "Building all systems in workspace..."

cargo build --workspace --all-features --verbose

echo "Build complete!"
