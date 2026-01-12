#!/bin/bash
set -e

echo "========================================="
echo "Setting up bmssp development environment"
echo "========================================="
echo ""

# Check for Homebrew
if ! command -v brew &> /dev/null; then
    echo "❌ Homebrew not found."
    echo "   Please install Homebrew from https://brew.sh"
    echo "   Or run: /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
    exit 1
else
    echo "✅ Homebrew found: $(brew --version | head -n1)"
fi

# Install Rust (if not installed)
if ! command -v rustc &> /dev/null; then
    echo ""
    echo "Installing Rust via Homebrew..."
    brew install rust
    # Source cargo env if rustup was used
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
    fi
else
    echo "✅ Rust already installed: $(rustc --version)"
fi

# Verify Rust installation
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found after Rust installation"
    exit 1
fi
echo "✅ Cargo found: $(cargo --version)"

# Check Python version
PYTHON_VERSION=$(python3 --version 2>&1 | awk '{print $2}')
echo "✅ Python found: $PYTHON_VERSION"

# Create virtual environment
if [ ! -d "venv" ]; then
    echo ""
    echo "Creating Python virtual environment..."
    python3 -m venv venv
    echo "✅ Virtual environment created in venv/"
else
    echo "✅ Virtual environment already exists in venv/"
fi

# Activate virtual environment
echo ""
echo "Activating virtual environment..."
source venv/bin/activate

# Upgrade pip
echo ""
echo "Upgrading pip to latest version..."
pip install --upgrade pip --quiet

# Install Python dependencies
echo ""
echo "Installing Python dependencies from requirements.txt..."
if [ -f "requirements.txt" ]; then
    pip install -r requirements.txt
else
    echo "⚠️  requirements.txt not found, installing packages directly..."
    pip install numpy scipy pytest maturin pytest-benchmark networkx
fi

# Verify installations
echo ""
echo "========================================="
echo "Verification"
echo "========================================="
echo ""

# System tools
echo "System tools:"
rustc --version
cargo --version
echo ""

# Python tools
echo "Python tools:"
python --version
pip --version
echo ""

# Installed packages
echo "Installed packages:"
maturin --version
pytest --version
python -c "import numpy; print('numpy:', numpy.__version__)"
python -c "import scipy; print('scipy:', scipy.__version__)"
python -c "import pytest_benchmark; print('pytest-benchmark: installed')"
python -c "import networkx; print('networkx:', networkx.__version__)"
echo ""

echo "========================================="
echo "✅ Setup complete!"
echo "========================================="
echo ""
echo "To activate the virtual environment in the future, run:"
echo "  source venv/bin/activate"
echo ""
echo "To deactivate, run:"
echo "  deactivate"
echo ""
