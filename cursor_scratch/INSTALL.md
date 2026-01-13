# Installation Instructions

This document provides step-by-step instructions for setting up the bmssp development environment.

## Quick Start

Run the automated setup script:

```bash
./setup.sh
```

## Manual Installation

If the automated script fails or you prefer manual installation, follow these steps:

### 1. Install Rust Toolchain

**Option A: Via Homebrew (macOS - Recommended)**
```bash
brew install rust
```

**Option B: Via rustup (Cross-platform)**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Verify installation:**
```bash
rustc --version  # Should show 1.85.x or later
cargo --version  # Should show 1.85.x or later
```

### 2. Set Up Python Virtual Environment

```bash
# Create virtual environment
python3 -m venv venv

# Activate virtual environment
source venv/bin/activate  # On macOS/Linux
# On Windows: venv\Scripts\activate

# Upgrade pip
pip install --upgrade pip

# Install dependencies
pip install -r requirements.txt
```

### 3. Verify Installation

```bash
# System tools
rustc --version
cargo --version

# Python tools (with venv activated)
python --version
pip --version
maturin --version
pytest --version

# Python packages
python -c "import numpy; print('numpy:', numpy.__version__)"
python -c "import scipy; print('scipy:', scipy.__version__)"
python -c "import pytest_benchmark; print('pytest-benchmark: installed')"
python -c "import networkx; print('networkx:', networkx.__version__)"
```

## Troubleshooting

### Homebrew Permission Issues

If you encounter permission errors with Homebrew (like `/opt/homebrew/Cellar is not writable`), you may need to fix permissions:

```bash
sudo chown -R $(whoami) /opt/homebrew
sudo chown -R $(whoami) /Users/$(whoami)/Library/Caches/Homebrew
sudo chown -R $(whoami) /Users/$(whoami)/Library/Logs/Homebrew
```

Alternatively, use rustup instead of Homebrew for Rust installation.

### Python Virtual Environment Issues

If you encounter issues with the virtual environment:

```bash
# Remove and recreate
rm -rf venv
python3 -m venv venv
source venv/bin/activate
pip install --upgrade pip
pip install -r requirements.txt
```

### Missing System Python

If Python 3 is not available, install it:

```bash
# macOS
brew install python@3.13

# Or use system Python 3.9+ (usually pre-installed on macOS)
```

## Next Steps

After successful installation:

1. Activate the virtual environment: `source venv/bin/activate`
2. Review `SPEC.md` for the project requirements
3. Review `TOOL_REQUIREMENTS.md` for detailed tool information
