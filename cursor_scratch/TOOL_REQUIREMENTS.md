# Tool Requirements and Installation Guide

This document outlines all command-line tools and dependencies required to implement the plan in `SPEC.md`. All tools should be installed and configured for use within this project directory.

## Project Setup Strategy

This project uses:
- **Homebrew (brew)** as the primary package manager for system tools on macOS
- **Python virtual environment** for Python dependencies (project-local installation)
- **rustup** for Rust toolchain management (can be installed via brew or rustup.sh)

## Required Tools Status

### ✅ Already Installed (System)

| Tool | Current Version | Target Version | Purpose | Status |
|------|----------------|----------------|---------|--------|
| `python3` | 3.9.6 | 3.13.x (latest stable) | Python interpreter | ✅ Installed (consider upgrading) |
| `pip3` | 21.2.4 | Latest | Python package manager | ✅ Installed |
| `pytest` | 9.0.1 | Latest | Python testing framework | ✅ Installed (system) |
| `numpy` | 2.0.2 | Latest | Array operations (CSR arrays, weights) | ✅ Installed (system) |
| `scipy` | 1.13.1 | Latest | Optional comparison tests | ✅ Installed (system) |

**Note**: System Python packages will be re-installed in the project virtual environment for isolation.

### ❌ Missing Required Tools

| Tool | Target Version | Purpose | Installation Method |
|------|----------------|---------|---------------------|
| **Rust toolchain** (`rustc` + `cargo`) | 1.85.x (latest stable) | Building Rust crates (`bmssp-core`, `bmssp-py`) | `brew install rust` (recommended) or `rustup` |
| **maturin** | 1.9.6+ (latest) | Building Python-Rust bindings (pyo3) and wheels | `pip install maturin` (in venv) or `cargo install maturin` |

### ⚠️ Missing Optional Tools

| Tool | Target Version | Purpose | When Needed |
|------|----------------|---------|-------------|
| `pytest-benchmark` | Latest | Python benchmarking | Milestone 4+ performance benchmarking |
| `NetworkX` | Latest | Baseline for Python benchmarks | Benchmark comparisons (optional) |

## Tool Details and Installation Instructions

### Core Build Tools

1. **Rust toolchain** (`rustc`, `cargo`)
   - **Target Version**: 1.85.x (latest stable)
   - **Required for**: Building `rust/bmssp-core/` and `rust/bmssp-py/` crates
   - **Spec reference**: Lines 60-61, 161-277 (Rust core design)
   - **Installation (macOS - Recommended)**:
     ```bash
     brew install rust
     ```
   - **Alternative (rustup - allows toolchain management)**:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     source $HOME/.cargo/env
     ```
   - **Verify**: `rustc --version` should show 1.85.x or later

2. **maturin**
   - **Target Version**: 1.9.6+ (latest)
   - **Required for**: Building Python-Rust bindings (pyo3) and creating distribution wheels
   - **Spec reference**: Line 278 ("Python bindings (pyo3 + maturin)"), Line 310 (wheel building)
   - **Installation (in project venv - Recommended)**:
     ```bash
     source venv/bin/activate  # After creating venv
     pip install maturin
     ```
   - **Alternative (system-wide via cargo)**:
     ```bash
     cargo install maturin
     ```
   - **Note**: Install after Rust toolchain is available
   - **Verify**: `maturin --version` should show 1.9.6 or later

### Development/Testing Tools

3. **pytest-benchmark**
   - **Target Version**: Latest
   - **Required for**: Python benchmarking (Python benchmark plan, line 369)
   - **Installation**: Installed in project virtual environment (see setup script)
   - **Command**: `pip install pytest-benchmark`

4. **criterion** (Rust crate, not CLI tool)
   - **Required for**: Rust benchmarking (Benchmark plan, line 355)
   - **Installation**: Added as a dev dependency in `Cargo.toml`, installed via `cargo build --benches`
   - **Note**: This is a Rust crate dependency, not a command-line tool

5. **proptest** (Rust crate, not CLI tool)
   - **Optional for**: Property-based testing (Testing plan, line 340)
   - **Installation**: Added as a dev dependency in `Cargo.toml`, installed via `cargo build`
   - **Note**: This is a Rust crate dependency, not a command-line tool

6. **NetworkX** (Python package)
   - **Target Version**: Latest
   - **Optional for**: Baseline comparison in benchmarks (Benchmark plan, line 378)
   - **Installation**: Installed in project virtual environment (see setup script)
   - **Command**: `pip install networkx`
   - **Note**: Only needed for benchmark comparisons, not core functionality

## Installation Instructions

### Prerequisites Check

First, verify Homebrew is installed:
```bash
brew --version  # Should show Homebrew version
# If not installed: /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### Step-by-Step Installation

#### 1. Install Rust Toolchain (Required)

**Option A: Via Homebrew (Recommended for macOS)**
```bash
brew install rust
```

**Option B: Via rustup (Allows multiple toolchain versions)**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Verify installation:**
```bash
rustc --version  # Should show rustc 1.85.x or later
cargo --version  # Should show cargo 1.85.x or later
```

#### 2. Set Up Python Virtual Environment (Required)

Create and activate a virtual environment in the project directory:
```bash
# From project root directory
python3 -m venv venv
source venv/bin/activate  # On macOS/Linux
# On Windows: venv\Scripts\activate

# Upgrade pip to latest
pip install --upgrade pip
```

#### 3. Install Python Dependencies (Required)

Install all Python packages in the virtual environment:
```bash
# Ensure venv is activated
source venv/bin/activate  # If not already activated

# Install core dependencies
pip install numpy scipy pytest

# Install build tool
pip install maturin

# Install development/testing dependencies
pip install pytest-benchmark networkx
```

**Note**: Create a `requirements.txt` or `pyproject.toml` during implementation to pin versions.

#### 4. Verify All Installations

Run verification script:
```bash
# System tools
rustc --version
cargo --version
brew --version  # If using Homebrew

# Python tools (with venv activated)
python --version  # Should show Python 3.x
pip --version
pytest --version
maturin --version
python -c "import numpy; print('numpy:', numpy.__version__)"
python -c "import scipy; print('scipy:', scipy.__version__)"
python -c "import pytest_benchmark; print('pytest-benchmark: installed')"
python -c "import networkx; print('networkx:', networkx.__version__)"
```

### Automated Setup Script

A setup script (`setup.sh`) will be created during implementation that automates the above steps:

```bash
#!/bin/bash
set -e

echo "Setting up bmssp development environment..."

# Check for Homebrew
if ! command -v brew &> /dev/null; then
    echo "Homebrew not found. Install from https://brew.sh"
    exit 1
fi

# Install Rust (if not installed)
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust via Homebrew..."
    brew install rust
else
    echo "Rust already installed: $(rustc --version)"
fi

# Create virtual environment
if [ ! -d "venv" ]; then
    echo "Creating Python virtual environment..."
    python3 -m venv venv
fi

# Activate virtual environment
echo "Activating virtual environment..."
source venv/bin/activate

# Upgrade pip
pip install --upgrade pip

# Install Python dependencies
echo "Installing Python dependencies..."
pip install numpy scipy pytest maturin pytest-benchmark networkx

# Verify installations
echo ""
echo "Verification:"
rustc --version
cargo --version
maturin --version
pytest --version
python -c "import numpy, scipy, pytest_benchmark, networkx; print('All Python packages installed successfully')"

echo ""
echo "Setup complete! Activate the virtual environment with: source venv/bin/activate"
```

## Platform-Specific Notes

The SPEC mentions building wheels for multiple platforms (lines 305-310):
- manylinux x86_64 and aarch64 (Linux)
- macOS x86_64 and arm64
- Windows x86_64

### macOS Development (Current System)

- **Primary installation method**: Homebrew (`brew install`)
- **Python**: Use system Python 3.9+ or install Python 3.13 via `brew install python@3.13`
- **Rust**: Install via `brew install rust` (recommended) or rustup
- **Virtual environment**: Create `venv/` in project directory for Python dependencies
- **Cross-compilation**: For building wheels for other platforms, use CI/CD (GitHub Actions) with maturin

### Version Compatibility

- **Rust**: 1.85.x (latest stable) - Required for modern Rust features
- **Python**: 3.9+ (3.13 recommended) - Must support pyo3 bindings
- **maturin**: 1.9.6+ - Latest version with security fixes and features
- **numpy**: 2.0+ - Latest API (backward compatible with 1.x)
- **pyo3**: Latest (managed by maturin) - Must be compatible with Python 3.9+

## Summary

### Installation Checklist

**System Tools (via Homebrew or rustup):**
- [ ] Homebrew installed
- [ ] Rust toolchain (1.85.x+) installed
- [ ] Python 3.9+ available (3.13 recommended)

**Project Tools (in virtual environment):**
- [ ] Python virtual environment created (`venv/`)
- [ ] maturin installed in venv
- [ ] numpy, scipy installed in venv
- [ ] pytest installed in venv
- [ ] pytest-benchmark installed in venv (optional, for benchmarks)
- [ ] networkx installed in venv (optional, for benchmarks)

**Action Required**: 
1. Install Rust toolchain via `brew install rust`
2. Create Python virtual environment: `python3 -m venv venv`
3. Activate venv and install Python dependencies
4. Run verification commands to confirm all tools are available

**Implementation Note**: During implementation, create a `setup.sh` script to automate the installation process for new developers.
