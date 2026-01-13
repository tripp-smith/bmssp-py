#!/bin/bash
# Build wheels for the current platform
# This script builds wheels using maturin for local testing

set -euo pipefail

# Get script directory (project root)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "Building wheels for bmssp..."
echo ""

# Check prerequisites
if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo not found. Please install Rust toolchain."
    exit 1
fi

if ! command -v maturin &> /dev/null; then
    echo "Error: Maturin not found. Please install maturin:"
    echo "  pip install maturin"
    exit 1
fi

# Change to python directory
cd python

# Create dist directory if it doesn't exist
mkdir -p dist

# Build wheel
echo "Building wheel with maturin..."
maturin build --release --out dist

echo ""
echo "✅ Wheel built successfully!"
echo "Wheels are in: $(pwd)/dist"
echo ""
echo "To test the wheel:"
echo "  pip install dist/bmssp-*.whl"
echo "  python -c 'import bmssp; print(bmssp.__version__)'"
