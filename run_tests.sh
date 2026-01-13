#!/bin/bash
# Test automation script for bmssp-py
# Runs Rust and Python test suites with proper environment setup

set -euo pipefail  # Exit on error, undefined vars, pipe failures

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Track results
RUST_TEST_EXIT=0
PYTHON_TEST_EXIT=0
BUILD_EXIT=0

# Get script directory (project root)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "========================================="
echo "BMSSP Test Automation Script"
echo "========================================="
echo ""

# ============================================================================
# Prerequisites Check
# ============================================================================

echo -e "${BLUE}Checking prerequisites...${NC}"

# Check Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Cargo not found. Please install Rust toolchain.${NC}"
    echo "   Install with: brew install rust"
    exit 1
fi
echo -e "${GREEN}✅ Cargo found: $(cargo --version)${NC}"

if ! command -v rustc &> /dev/null; then
    echo -e "${RED}❌ Rustc not found. Please install Rust toolchain.${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Rustc found: $(rustc --version)${NC}"

# Check Python
if ! command -v python3 &> /dev/null; then
    echo -e "${RED}❌ Python3 not found. Please install Python 3.9+.${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Python3 found: $(python3 --version)${NC}"

# Check virtual environment
VENV_DIR="$SCRIPT_DIR/python/.venv"
if [ ! -d "$VENV_DIR" ]; then
    echo -e "${RED}❌ Virtual environment not found at $VENV_DIR${NC}"
    echo "   Please run setup script or create venv manually"
    exit 1
fi
echo -e "${GREEN}✅ Virtual environment found${NC}"

# Check maturin in venv
if [ ! -f "$VENV_DIR/bin/maturin" ]; then
    echo -e "${RED}❌ Maturin not found in virtual environment${NC}"
    echo "   Please install dependencies: source python/.venv/bin/activate && pip install -r requirements.txt"
    exit 1
fi
echo -e "${GREEN}✅ Maturin found in virtual environment${NC}"

echo ""

# ============================================================================
# Python Environment Setup
# ============================================================================

echo -e "${BLUE}Setting up Python environment...${NC}"

# Activate virtual environment
source "$VENV_DIR/bin/activate"
echo -e "${GREEN}✅ Virtual environment activated${NC}"

# Install optional dependencies for full test coverage
echo ""
echo -e "${BLUE}Installing optional test dependencies...${NC}"
set +e
pip install scipy --quiet 2>&1
SCIPY_INSTALL_EXIT=$?
set -e
if [ $SCIPY_INSTALL_EXIT -eq 0 ]; then
    echo -e "${GREEN}✅ SciPy installed${NC}"
else
    echo -e "${YELLOW}⚠️  SciPy installation failed (some tests will be skipped)${NC}"
fi

# Build Python bindings
echo ""
echo -e "${BLUE}Building Python bindings with maturin...${NC}"
cd "$SCRIPT_DIR/python"

# Temporarily disable exit on error for build (we'll check exit code manually)
set +e
maturin develop --manifest-path "$SCRIPT_DIR/rust/bmssp-py/Cargo.toml" --quiet 2>&1
BUILD_EXIT=$?
set -e

if [ $BUILD_EXIT -ne 0 ]; then
    echo -e "${RED}❌ Failed to build Python bindings${NC}"
    echo "   Try running manually: cd python && maturin develop"
    exit 1
fi

echo -e "${GREEN}✅ Python bindings built successfully${NC}"

# Verify bindings
echo -e "${BLUE}Verifying Python bindings...${NC}"
if ! python -c "import _bmssp" 2>/dev/null; then
    echo -e "${RED}❌ Failed to import _bmssp module${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Python bindings verified${NC}"
echo ""

# ============================================================================
# Rust Tests Execution
# ============================================================================

echo "========================================="
echo -e "${BLUE}Running Rust Tests${NC}"
echo "========================================="
echo ""

cd "$SCRIPT_DIR/rust/bmssp-core"

# Temporarily disable exit on error to capture test results
set +e
cargo test --all-features 2>&1 | tee /tmp/rust_test_output.txt
RUST_TEST_EXIT=${PIPESTATUS[0]}
set -e

# Parse test results (BSD grep compatible) - sum all test results
# Cargo outputs multiple "test result:" lines, one per test file
RUST_PASSED=$(grep -E 'test result:.*passed' /tmp/rust_test_output.txt | grep -oE '[0-9]+ passed' | grep -oE '[0-9]+' | awk '{sum+=$1} END {print sum+0}')
RUST_FAILED=$(grep -E 'test result:.*failed' /tmp/rust_test_output.txt | grep -oE '[0-9]+ failed' | grep -oE '[0-9]+' | awk '{sum+=$1} END {print sum+0}')

if [ $RUST_TEST_EXIT -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✅ Rust tests completed successfully${NC}"
    echo -e "   Passed: $RUST_PASSED, Failed: $RUST_FAILED"
else
    echo ""
    echo -e "${RED}❌ Rust tests failed${NC}"
    echo -e "   Passed: $RUST_PASSED, Failed: $RUST_FAILED"
fi
echo ""

# ============================================================================
# Python Tests Execution
# ============================================================================

echo "========================================="
echo -e "${BLUE}Running Python Tests${NC}"
echo "========================================="
echo ""

cd "$SCRIPT_DIR/python"

# Ensure venv is still activated
if [ -z "${VIRTUAL_ENV:-}" ]; then
    source "$VENV_DIR/bin/activate"
fi

# Temporarily disable exit on error to capture test results
set +e
pytest tests/ -v 2>&1 | tee /tmp/python_test_output.txt
PYTHON_TEST_EXIT=${PIPESTATUS[0]}
set -e

# Parse test results (pytest format: "X passed, Y failed in Z seconds") - BSD grep compatible
PYTHON_PASSED=$(grep -oE '[0-9]+ passed' /tmp/python_test_output.txt | head -1 | grep -oE '[0-9]+' || echo "0")
PYTHON_FAILED=$(grep -oE '[0-9]+ failed' /tmp/python_test_output.txt | head -1 | grep -oE '[0-9]+' || echo "0")

if [ $PYTHON_TEST_EXIT -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✅ Python tests completed successfully${NC}"
    echo -e "   Passed: $PYTHON_PASSED, Failed: $PYTHON_FAILED"
else
    echo ""
    echo -e "${RED}❌ Python tests failed${NC}"
    echo -e "   Passed: $PYTHON_PASSED, Failed: $PYTHON_FAILED"
fi
echo ""

# ============================================================================
# Summary
# ============================================================================

echo "========================================="
echo -e "${BLUE}Test Summary${NC}"
echo "========================================="
echo ""

# Calculate totals
TOTAL_PASSED=$((RUST_PASSED + PYTHON_PASSED))
TOTAL_FAILED=$((RUST_FAILED + PYTHON_FAILED))

printf "%-20s %10s %10s\n" "Test Suite" "Passed" "Failed"
echo "--------------------------------------------"
printf "%-20s %10s %10s\n" "Rust" "$RUST_PASSED" "$RUST_FAILED"
printf "%-20s %10s %10s\n" "Python" "$PYTHON_PASSED" "$PYTHON_FAILED"
echo "--------------------------------------------"
printf "%-20s %10s %10s\n" "Total" "$TOTAL_PASSED" "$TOTAL_FAILED"
echo ""

# Determine overall status
OVERALL_EXIT=0
if [ $RUST_TEST_EXIT -ne 0 ] || [ $PYTHON_TEST_EXIT -ne 0 ]; then
    OVERALL_EXIT=1
    echo -e "${RED}❌ Some tests failed${NC}"
else
    echo -e "${GREEN}✅ All tests passed!${NC}"
fi

echo ""
echo "========================================="

# Cleanup
rm -f /tmp/rust_test_output.txt /tmp/python_test_output.txt

exit $OVERALL_EXIT
