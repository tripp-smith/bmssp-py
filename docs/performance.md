# Performance Guide

## Overview

BMSSP is designed for fast single-source shortest path computation on large sparse graphs, especially when many SSSP calls are needed (scenario analysis, repeated queries, etc.).

## Performance Characteristics

### When BMSSP Excels

- **Large sparse graphs**: Hundreds of thousands to millions of edges
- **Repeated calls**: Many SSSP computations with different sources or scenarios
- **Dynamic weights**: Frequent weight updates without topology changes
- **Outage simulation**: Fast re-routing after edge failures

### When to Use Alternatives

- **Small graphs** (< 1000 vertices): Dijkstra may be faster due to lower overhead
- **Single SSSP call**: Dijkstra is simpler and often faster
- **Dense graphs**: Other algorithms may be more appropriate
- **Negative weights**: BMSSP does not support negative weights (use Bellman-Ford)

## Benchmarking

The package includes benchmarks in:
- `rust/bmssp-core/benches/` - Rust-level benchmarks using criterion
- `python/benchmarks/` - Python-level benchmarks using pytest-benchmark

Run benchmarks:
```bash
# Rust benchmarks
cd rust/bmssp-core
cargo bench

# Python benchmarks
cd python
pytest benchmarks/ --benchmark-only
```

## Optimization Tips

1. **Use CSR format**: Building graphs from CSR is faster than edge lists
2. **Reuse graphs**: Graph topology is immutable - build once, reuse for many SSSP calls
3. **Update weights in-place**: Modify weight arrays rather than rebuilding graphs
4. **Use enabled masks**: For outages, use enabled masks rather than rebuilding topology
5. **Choose appropriate precision**: Use f32 for speed, f64 for precision

## Memory Usage

- Graph structure: O(m + n) space
- SSSP computation: O(n) additional space for distances and predecessors
- Enabled masks: O(m) space

## Scaling Behavior

Expected performance scales as:
- Time: O(m + n log n) expected for sparse graphs
- Space: O(m + n)

For very large graphs, consider:
- Graph partitioning
- Approximate algorithms for exploratory analysis
- Batch processing of multiple SSSP calls
