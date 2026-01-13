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
  - `dijkstra_vs_bmssp` - Comparison of BMSSP vs Dijkstra algorithms
  - `block_heap_bench` - Comparison of heap implementations (BTreeSet vs BinaryHeap)
- `python/benchmarks/` - Python-level benchmarks using pytest-benchmark

Run benchmarks:
```bash
# Rust benchmarks
cd rust/bmssp-core
cargo bench

# Specific benchmark
cargo bench --bench dijkstra_vs_bmssp
cargo bench --bench block_heap_bench

# Python benchmarks
cd python
pytest benchmarks/ --benchmark-only
```

## Performance Improvements

Recent optimizations (Milestone 4) include:

1. **Fast BlockHeap**: Replaced BTreeSet-based heap with BinaryHeap-based implementation using stale entry tracking, providing better performance for decrease-key operations
2. **State Reuse API**: Added `BmsspState` for buffer reuse across multiple SSSP calls, reducing allocations for repeated computations
3. **API-level fast paths**: Use `bmssp_sssp` (without predecessors) when predecessor tracking isn't needed

## Optimization Tips

1. **Use CSR format**: Building graphs from CSR is faster than edge lists
2. **Reuse graphs**: Graph topology is immutable - build once, reuse for many SSSP calls
3. **Update weights in-place**: Modify weight arrays rather than rebuilding graphs
4. **Use enabled masks**: For outages, use enabled masks rather than rebuilding topology
5. **Choose appropriate precision**: Use f32 for speed, f64 for precision
6. **Use state reuse for repeated calls**: For performance-critical scenarios with many SSSP calls, use `BmsspState` to avoid allocations between calls (see State Reuse API below)

## State Reuse API

For scenarios with many repeated SSSP calls (e.g., scenario analysis, repeated queries), use the state-based API to avoid allocations:

```rust
use bmssp_core::{BmsspState, bmssp_sssp_with_state, CsrGraph};

// Create reusable state (sized for largest graph you'll use)
let mut state = BmsspState::new(max_vertices);

// Reuse state across multiple calls
for source in sources {
    let distances = bmssp_sssp_with_state(&mut state, &graph, &weights, source, None)?;
    // Process distances...
}
```

The state-based API (`bmssp_sssp_with_state` and `bmssp_sssp_with_preds_and_state`) reuses internal buffers, reducing allocation overhead for repeated calls.

## Feature Flags

The crate supports optional feature flags for compilation-time customization:

- **Default features**: `["f32", "f64"]` - Supports both f32 and f64 (default)
- **f32-only / f64-only**: Placeholder flags (code uses generics, so compile-time specialization is limited)
- **no-predecessors**: Placeholder flag (predecessor tracking is optional at API level: use `bmssp_sssp` instead of `bmssp_sssp_with_preds`)
- **fast-math**: Placeholder flag (would enable compiler fast-math optimizations, requires build configuration)

Note: Due to the generic design of the codebase, most feature flags provide API-level optimizations rather than compile-time code elimination. Use the appropriate API functions (`bmssp_sssp` vs `bmssp_sssp_with_preds`) to avoid unnecessary computations.

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
