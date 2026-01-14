# BMSSP: Fast Single-Source Shortest Paths

![Tests](https://github.com/tripp-smith/bmssp-py/workflows/Tests/badge.svg)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Python 3.9+](https://img.shields.io/badge/python-3.9+-blue.svg)

A Python package providing fast single-source shortest path (SSSP) computation using the BMSSP algorithm, with a high-performance Rust backend.

## Status

**Beta (v0.1.0)** - This package is ready for use. The API is stable, but we welcome feedback and contributions.

## Features

- Fast SSSP computation using BMSSP algorithm
- Optimized for large sparse graphs (hundreds of thousands to millions of edges)
- Support for dynamic weights and edge outages
- Clean Python API with NumPy integration
- Rust backend for maximum performance
- Support for both f32 and f64 precision
- Predecessor tracking for path reconstruction

## Installation

**Note**: This package is not yet available on PyPI. For detailed installation instructions, see the [Installation Guide](docs/installation.md).

### Quick Installation

```bash
# Install Rust toolchain first
brew install rust  # or use rustup

# Clone repository and install
cd bmssp-py/python
maturin develop
```

## Quick Start

```python
import numpy as np
from bmssp import Graph, sssp

# Create a graph from edges
n = 4
edges = np.array([[0, 1], [1, 2], [0, 2], [2, 3]], dtype=np.int64)
weights = np.array([1.0, 2.0, 1.5, 1.0], dtype=np.float32)

graph, _ = Graph.from_edges(n, edges, weights=weights)

# Compute shortest paths from vertex 0
result = sssp(graph, weights, source=0)
print(result.dist)  # Distances from source to each vertex

# With path reconstruction
result = sssp(graph, weights, source=0, return_predecessors=True)
from bmssp import reconstruct_path
path = reconstruct_path(result.pred, target=3)
print(f"Path: {path}")
```

## Performance Highlights

- **Optimized for large sparse graphs**: Handles hundreds of thousands to millions of edges efficiently
- **Fast repeated computations**: Ideal for scenario analysis with many SSSP calls
- **Dynamic weight updates**: Update weights without rebuilding graph topology
- **Rust backend**: High-performance implementation with minimal Python overhead

For detailed performance information, see the [Performance Guide](docs/performance.md).

## Example: Grid Network Optimization

See `python/examples/grid_pipeline.py` for a complete example demonstrating:
- Building grid networks
- Applying load flows and congestion models
- Handling outages
- Recomputing paths

## Documentation

- [Installation Guide](docs/installation.md) - Detailed installation instructions
- [Tutorial](docs/tutorial.md) - Step-by-step guide with examples
- [API Reference](docs/api.md) - Complete API documentation
- [Performance Guide](docs/performance.md) - Performance characteristics and optimization tips
- [Algorithm Description](docs/algorithm.md) - BMSSP algorithm overview

## Development

For development setup, install the Rust toolchain and use `maturin develop` from the `python/` directory.

## Citation

If you use this implementation in your research, please cite the original paper:

> Ran Duan, Jiayi Mao, Xiao Mao, Xinkai Shu, Longhui Yin. "Breaking the Sorting Barrier for Directed Single-Source Shortest Paths." *arXiv preprint arXiv:2504.17033* (2025).  
> [https://arxiv.org/abs/2504.17033](https://arxiv.org/abs/2504.17033)

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
