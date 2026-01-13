#!/bin/bash
# Build wheels for the current platform
# Usage: ./scripts/build-wheels.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

echo "Building wheels for bmssp..."

# Check prerequisites
if ! command -v maturin &> /dev/null; then
    echo "Error: maturin not found. Install with: pip install maturin"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "Error: cargo not found. Install Rust toolchain first."
    exit 1
fi

# Create dist directory
DIST_DIR="$PROJECT_ROOT/python/dist"
mkdir -p "$DIST_DIR"

# Build wheel
echo "Building wheel..."
cd "$PROJECT_ROOT/python"
maturin build --release --out "$DIST_DIR"

echo ""
echo "Wheels built successfully!"
echo "Output directory: $DIST_DIR"
ls -lh "$DIST_DIR"/*.whl 2>/dev/null || echo "No .whl files found (check for errors above)"
