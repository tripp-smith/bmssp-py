# Milestone 4: Performance Hardening Implementation Plan

## Overview

Milestone 4 focuses on optimizing the BMSSP implementation for performance. The correctness-first approach from Milestone 2 provides a solid foundation; now we optimize while maintaining correctness. The goal is to achieve performance improvements over Dijkstra on large sparse graphs, especially for repeated SSSP calls.

## Current State Assessment

### Baseline Performance Characteristics

**Current Implementation**:
- BlockHeap uses `BTreeSet` + `HashMap` for decrease-key (simple, correct)
- BMSSP uses block-based processing (simpler than full recursive pivot-based)
- Some allocations per SSSP call
- Both f32 and f64 supported (no feature flags)

**Performance Bottlenecks (Expected)**:
- BlockHeap operations (BTreeSet insert/remove overhead)
- Repeated allocations for distance/pred arrays
- No buffer reuse between calls
- No specialized paths for common cases (no predecessors, f32 only, etc.)

## Performance Goals

### Target Improvements

1. **BlockHeap Optimization**: Replace BTreeSet+HashMap with faster structure (pairing heap or binary heap with stale entries)
2. **Allocation Reduction**: Reuse buffers, pre-allocate where possible
3. **Feature Flags**: Allow compilation-time optimizations (f32 vs f64, predecessors optional)
4. **Specialized Paths**: Fast paths for common cases (no predecessors, etc.)

### Benchmark Targets

- Match or exceed Dijkstra performance on small graphs (< 100 vertices)
- Show clear improvement on large sparse graphs (1000+ vertices, E ≈ 5V)
- Demonstrate benefits in repeated call scenarios (100-1000 calls)
- Maintain correctness (all tests must still pass)

## Phase 1: Performance Profiling and Baseline

### Task 1.1: Establish Performance Baselines

**Files**: `rust/bmssp-core/benches/dijkstra_vs_bmssp.rs`

**Action**: Run comprehensive benchmarks to establish current performance

**Benchmarks to Run**:
- Small graphs (10-100 vertices)
- Medium graphs (100-500 vertices)
- Large graphs (500-2000 vertices)
- Sparse graphs (E ≈ 2V to E ≈ 5V)
- Dense graphs (E ≈ 10V)
- Repeated calls (same graph, different sources/weights)

**Metrics to Collect**:
- Time per SSSP call
- Peak memory usage (if possible)
- Scaling behavior (time vs. graph size)
- Comparison: BMSSP vs. Dijkstra

**Deliverable**: Benchmark report showing current performance characteristics

### Task 1.2: Identify Hot Paths

**Tools**: `cargo flamegraph`, `perf`, or similar profiling tools

**Analysis**:
- Identify functions consuming most time
- Identify allocation hotspots
- Identify data structure operations (BlockHeap operations, etc.)
- Identify cache misses or other CPU inefficiencies

**Deliverable**: Profiling report identifying optimization opportunities

## Phase 2: BlockHeap Optimization

### Task 2.1: Implement Pairing Heap (or Binary Heap with Stale Entries)

**File**: `rust/bmssp-core/src/block_heap.rs`

**Current**: `BTreeSet` + `HashMap` for decrease-key

**Options**:

**Option A: Pairing Heap**
- Pros: Good decrease-key performance, simpler than Fibonacci heap
- Cons: Requires new implementation or dependency
- Implementation: Implement pairing heap or use crate like `pairing-heap`

**Option B: Binary Heap with Stale Entries**
- Pros: Uses standard library, simple "lazy delete" approach
- Cons: May have more stale entries, slightly more complex logic
- Implementation: Modify BlockHeap to use `BinaryHeap` and track "stale" entries

**Option C: Hybrid Approach**
- Start with Option B (simpler), fall back to Option A if needed
- Benchmark both approaches

**Recommendation**: Start with Option B (binary heap with stale entries) as it's simpler and can provide significant improvements. Upgrade to pairing heap if needed.

**Implementation Steps**:
1. Create new `FastBlockHeap` or modify existing `BlockHeap`
2. Implement using `BinaryHeap` with stale entry tracking
3. Add tests to ensure correctness
4. Benchmark vs. current implementation
5. Replace or add as alternative based on benchmark results

### Task 2.2: Benchmark Heap Implementations

**File**: `rust/bmssp-core/benches/block_heap_bench.rs` (new file)

**Benchmarks**:
- Push operations
- Pop operations
- Decrease-key operations
- Block extraction
- Mixed workloads (simulating BMSSP usage)

**Comparison**: BTreeSet+HashMap vs. BinaryHeap+Stale vs. PairingHeap

## Phase 3: Allocation Reduction

### Task 3.1: Buffer Reuse

**Files**: `rust/bmssp-core/src/bmssp.rs`

**Current**: Allocate distance and predecessor arrays per call

**Optimization**: Provide reusable buffer interface

**Approach**:
- Create `BmsspState` struct that holds reusable buffers
- Allow caller to reuse state across multiple calls
- Default to allocating per-call for API compatibility
- Add `bmssp_sssp_with_state()` function for performance-critical paths

**Implementation**:
```rust
pub struct BmsspState<T> {
    distances: Vec<T>,
    predecessors: Vec<usize>,
    // ... other reusable buffers
}

pub fn bmssp_sssp_with_state<T>(
    state: &mut BmsspState<T>,
    graph: &CsrGraph,
    weights: &[T],
    source: usize,
    enabled: Option<&[bool]>,
) -> Result<&[T]>
```

**Considerations**:
- Maintain backward compatibility (existing API still works)
- State must be sized for largest graph used
- Clear/reset state between calls
- Thread safety (state not thread-safe by default)

### Task 3.2: Pre-allocation Strategy

**Areas for Pre-allocation**:
- BlockHeap capacity hints
- Temporary vectors for block processing
- Pivot finding temporary structures

**Implementation**:
- Add capacity parameters where helpful
- Pre-size vectors based on graph characteristics
- Reuse scratch space within single SSSP call

### Task 3.3: Zero-Copy Optimizations (Advanced, Optional)

**Considerations**:
- Minimize data copying between Python and Rust
- Use views/slices where possible
- Consider pinned memory for large arrays

**Note**: This may be more relevant for Python bindings layer (Milestone 5)

## Phase 4: Feature Flags and Specialized Paths

### Task 4.1: Add Feature Flags

**File**: `rust/bmssp-core/Cargo.toml`

**Flags to Add**:
- `f64-only`: Compile only f64 support (smaller binary, potentially faster)
- `f32-only`: Compile only f32 support
- `no-predecessors`: Specialized build without predecessor tracking code
- `fast-math`: Enable fast-math optimizations (with appropriate warnings)

**Implementation**:
- Use `#[cfg(feature = "...")]` attributes
- Default features: `["f32", "f64"]` (backward compatible)
- Optional features: `["f64-only", "f32-only", "no-predecessors", "fast-math"]`

**Considerations**:
- Maintain default behavior (backward compatible)
- Document feature flags
- Test with different feature combinations

### Task 4.2: Specialized Fast Paths

**File**: `rust/bmssp-core/src/bmssp.rs`

**Fast Paths to Add**:
- No-predecessors path (skip predecessor updates)
- Small graph path (use simpler algorithm for very small graphs)
- No-enabled-mask path (skip mask checks when all edges enabled)

**Implementation**:
- Add internal functions: `bmssp_sssp_no_pred()`, `bmssp_sssp_no_mask()`, etc.
- Route to appropriate function based on parameters
- Use `#[inline]` hints where beneficial
- Benchmark to ensure improvements

**Note**: Some of these may be handled by compiler optimizations; verify benefits with benchmarks.

## Phase 5: Algorithm Optimizations

### Task 5.1: Block Size Tuning

**File**: `rust/bmssp-core/src/params.rs`

**Current**: Parameters based on log(n) formulas

**Optimization**: Tune parameters based on benchmarking

**Approach**:
- Benchmark different parameter values
- Create lookup table for common graph sizes
- Use formulas for general case, tables for common cases
- Consider graph density in parameter selection

**Considerations**:
- Parameters affect correctness (must maintain correctness)
- Tuning is empirical (benchmark-driven)
- Document any changes

### Task 5.2: Early Termination Optimizations (Optional)

**Considerations**:
- Stop early if all targets reached (multi-sink scenarios)
- Skip processing if no improvements possible
- Batch edge relaxations

**Note**: These optimizations are more complex and may not provide significant benefits. Evaluate based on profiling.

## Phase 6: Benchmarking and Validation

### Task 6.1: Comprehensive Benchmark Suite

**Files**: 
- `rust/bmssp-core/benches/dijkstra_vs_bmssp.rs` (enhance)
- `python/benchmarks/bench_sssp.py` (enhance)

**Benchmarks**:
- Single SSSP calls (various graph sizes)
- Repeated SSSP calls (scenario simulation)
- Different graph topologies (grid, random, pipeline-like)
- Comparison: BMSSP vs. Dijkstra vs. SciPy (Python)
- Memory usage profiling

**Metrics**:
- Time (mean, median, p95, p99)
- Memory (peak, per-call)
- Scaling curves (time vs. graph size, time vs. edge count)
- Speedup ratios (BMSSP / Dijkstra)

### Task 6.2: Regression Testing

**Ensuring Correctness**:
- All existing tests must still pass
- Add performance regression tests (ensure no major slowdowns)
- Verify correctness on performance-critical paths
- Test with different feature flag combinations

### Task 6.3: Performance Documentation

**File**: `docs/performance.md` (update)

**Content**:
- Performance characteristics
- Benchmark results
- When to use BMSSP vs. alternatives
- Performance tuning tips
- Feature flag recommendations

## Success Criteria

Milestone 4 is complete when:

1. ✅ BlockHeap optimization shows measurable improvement (or verified current is optimal)
2. ✅ Allocation reduction improves repeated-call performance
3. ✅ Feature flags allow compilation-time optimizations
4. ✅ Benchmarks show BMSSP competitive or superior to Dijkstra on target workloads
5. ✅ All tests pass (correctness maintained)
6. ✅ Performance improvements documented
7. ✅ No performance regressions on small graphs
8. ✅ Clear performance wins on large sparse graphs with repeated calls

## Files to Modify/Create

**Rust Core**:
- `rust/bmssp-core/src/block_heap.rs` - Optimize or add alternative implementation
- `rust/bmssp-core/src/bmssp.rs` - Add state reuse, fast paths
- `rust/bmssp-core/src/params.rs` - Tune parameters
- `rust/bmssp-core/Cargo.toml` - Add feature flags
- `rust/bmssp-core/benches/block_heap_bench.rs` - New benchmark file
- `rust/bmssp-core/benches/dijkstra_vs_bmssp.rs` - Enhance benchmarks

**Python Benchmarks**:
- `python/benchmarks/bench_sssp.py` - Enhance benchmarks

**Documentation**:
- `docs/performance.md` - Update with results and guidance

**Tests**:
- Add performance regression tests (optional)
- Test with different feature flag combinations

## Implementation Strategy

### Phased Approach

1. **Phase 1 (Profiling)**: Understand current performance
2. **Phase 2 (Heap)**: Biggest win, implement first
3. **Phase 3 (Allocations)**: Important for repeated calls
4. **Phase 4 (Features)**: Compile-time optimizations
5. **Phase 5 (Algorithm)**: Fine-tuning
6. **Phase 6 (Validation)**: Ensure correctness and document

### Measurement-Driven

- Profile before optimizing
- Measure after each optimization
- Verify improvements with benchmarks
- Don't optimize without data showing benefit

### Correctness First

- All optimizations must maintain correctness
- Test thoroughly after each change
- Keep performance tests separate from correctness tests
- Document any correctness assumptions

## Dependencies

- Milestone 2 complete (correct BMSSP implementation)
- Milestone 3 complete (scenarios for benchmarking)
- Benchmark infrastructure in place
- Profiling tools available

## Next Steps After Milestone 4

Once Milestone 4 is complete, the project will have:
- Optimized performance for target use cases
- Documented performance characteristics
- Feature flags for different deployment scenarios
- Comprehensive benchmarks

This sets up well for:
- **Milestone 5**: Packaging and distribution (have performant, feature-complete package ready for release)

## Notes

- Performance optimization is iterative; don't expect to complete all tasks in one pass
- Some optimizations may not provide expected benefits; be ready to revert
- Focus on target use case (large sparse graphs, repeated calls) rather than optimizing for all cases
- Maintain code readability and maintainability while optimizing
