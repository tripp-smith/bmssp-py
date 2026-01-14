# Installation Guide

This guide provides detailed installation instructions for the BMSSP package.

## System Requirements

- **Python**: 3.9 or higher
- **Rust**: Latest stable version (1.70+ recommended)
- **Operating System**: macOS, Linux, or Windows

## Installation from PyPI

**Note**: The package is not yet available on PyPI. Once released, installation will be:

```bash
pip install bmssp
```

## Installation from Source

### Prerequisites

First, install the Rust toolchain:

**macOS (using Homebrew):**
```bash
brew install rust
```

**Linux/Windows (using rustup - recommended):**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Verify installation:**
```bash
rustc --version
cargo --version
```

### Build and Install

1. **Clone the repository:**
```bash
git clone https://github.com/tripp-smith/bmssp-py.git
cd bmssp-py
```

2. **Install Python dependencies:**
```bash
cd python
pip install -r requirements.txt
```

3. **Build and install the package:**
```bash
maturin develop
```

This will compile the Rust code and install the Python package in development mode.

### Verify Installation

```bash
python -c "import bmssp; print(bmssp.__version__)"
```

## Development Installation

For development work, you may want to install additional dependencies:

```bash
pip install -r requirements.txt
pip install pytest scipy networkx pytest-benchmark
```

Then build in development mode:

```bash
cd python
maturin develop
```

## Platform-Specific Notes

### macOS

- Homebrew installation of Rust works well
- For Apple Silicon (M1/M2), ensure you have the correct architecture
- Universal wheels will be available once the package is published

### Linux

- Use rustup for the most reliable installation
- You may need to install build essentials:
  ```bash
  sudo apt-get install build-essential  # Ubuntu/Debian
  ```

### Windows

- Use rustup for installation
- You may need Visual Studio Build Tools or the C++ build tools
- PowerShell or Git Bash recommended for running commands

## Troubleshooting

### Common Issues

**Problem**: `maturin: command not found`
- **Solution**: Install maturin: `pip install maturin`

**Problem**: Rust compilation errors
- **Solution**: Ensure Rust is up to date: `rustup update stable`

**Problem**: Python version not found
- **Solution**: Ensure Python 3.9+ is installed and in your PATH

**Problem**: Import errors after installation
- **Solution**: Ensure you're using the correct Python environment (virtual environment if used)

**Problem**: Build fails on macOS with "linker not found"
- **Solution**: Install Xcode command line tools: `xcode-select --install`

**Problem**: Build fails on Linux with missing libraries
- **Solution**: Install development packages:
  ```bash
  sudo apt-get install build-essential libssl-dev  # Ubuntu/Debian
  ```

### Getting Help

If you encounter issues not covered here:

1. Check the [GitHub Issues](https://github.com/tripp-smith/bmssp-py/issues)
2. Review the [README](README.md) for basic usage
3. Consult the [Tutorial](tutorial.md) for examples

## Next Steps

After successful installation:

1. Try the [Quick Start](README.md#quick-start) example
2. Read the [Tutorial](tutorial.md) for detailed examples
3. Explore the [API Reference](api.md) for complete documentation
