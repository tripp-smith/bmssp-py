# BMSSP: Fast Single-Source Shortest Paths

A Python package providing fast single-source shortest path (SSSP) computation using the BMSSP algorithm, with a high-performance Rust backend.

## Status

🚧 **Under Development** - This package is currently being implemented.

## Features

- Fast SSSP computation using BMSSP algorithm
- Optimized for large sparse graphs (hundreds of thousands to millions of edges)
- Support for dynamic weights and edge outages
- Clean Python API with NumPy integration
- Rust backend for maximum performance
- Support for both f32 and f64 precision
- Predecessor tracking for path reconstruction

## Installation

**Note**: This package is not yet available on PyPI. For development installation:

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

## Example: Grid Network Optimization

See `python/examples/grid_pipeline.py` for a complete example demonstrating:
- Building grid networks
- Applying load flows and congestion models
- Handling outages
- Recomputing paths

## Development

For development setup, install the Rust toolchain and use `maturin develop` from the `python/` directory.

## Citation

If you use this implementation in your research, please cite the original paper:

> Ran Duan, Jiayi Mao, Xiao Mao, Xinkai Shu, Longhui Yin. "Breaking the Sorting Barrier for Directed Single-Source Shortest Paths." *arXiv preprint arXiv:2504.17033* (2025).  
> [https://arxiv.org/abs/2504.17033](https://arxiv.org/abs/2504.17033)

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
