# BMSSP: Fast Single-Source Shortest Paths

A Python package providing fast single-source shortest path (SSSP) computation using the BMSSP algorithm, with a high-performance Rust backend.

## Status

🚧 **Under Development** - This package is currently being implemented.

## Features

- Fast SSSP computation using BMSSP algorithm (currently using Dijkstra baseline)
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

See `INSTALL.md` for detailed setup instructions.

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

See `INSTALL.md` for development setup instructions.

## License

Licensed under either of:
- Apache License, Version 2.0 (LICENSE-APACHE)
- MIT License (LICENSE-MIT)

at your option.
