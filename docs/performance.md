# Performance Guide

## When to Use BMSSP

BMSSP is designed for scenarios where you need to:
- Compute many SSSP calls on the same graph structure
- Handle dynamic weights (weights change frequently)
- Simulate outages and scenarios
- Work with large sparse graphs (hundreds of thousands to millions of edges)

## Performance Characteristics

### Current Implementation

The current implementation uses Dijkstra's algorithm as a baseline. Full BMSSP algorithm implementation is in progress.

**Dijkstra Baseline:**
- Time complexity: O(m log n) where m = edges, n = vertices
- Space complexity: O(n + m)
- Good for: Small to medium graphs, single SSSP calls

### Expected BMSSP Performance

**BMSSP (when fully implemented):**
- Time complexity: O(m + n log n) expected
- Space complexity: O(n + m)
- Good for: Large sparse graphs, repeated SSSP calls

## Optimization Tips

1. **Use CSR format directly**: If you have CSR arrays, use `Graph.from_csr()` to avoid edge list conversion overhead

2. **Reuse graphs**: Graph topology is immutable - create once, reuse for multiple SSSP calls with different weights

3. **Use enabled masks for outages**: Instead of modifying weights, use the `enabled` parameter for faster outage simulation

4. **Choose appropriate precision**: Use `f32` when precision is sufficient, `f64` when higher precision is needed

5. **Batch operations**: When running multiple scenarios, reuse the graph object

## Benchmarks

Run benchmarks with:

```bash
# Rust benchmarks
cd rust/bmssp-core
cargo bench

# Python benchmarks
cd python
pytest benchmarks/ -v
```

## Memory Usage

- Graph storage: O(n + m) for CSR format
- SSSP computation: O(n) for distances and predecessors
- Total: O(n + m) space

For graphs with millions of edges, memory usage is primarily determined by the graph structure itself.
