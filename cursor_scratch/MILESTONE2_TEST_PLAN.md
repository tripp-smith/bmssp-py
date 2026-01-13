# Milestone 2 Test Plan

This document describes the comprehensive test suite for Milestone 2, which implements the BMSSP algorithm with correctness verification against Dijkstra as the oracle.

## Overview

The test suite verifies that the BMSSP implementation produces correct results matching Dijkstra's algorithm within floating point tolerance. Tests are organized into three categories:

1. **Unit Tests** (Rust): Test individual algorithm components
2. **Parity Tests** (Rust): Compare BMSSP against Dijkstra on diverse graphs
3. **Integration Tests** (Python): End-to-end API tests and comparison with SciPy

## Test Categories

### 1. Rust Unit Tests

Location: `rust/bmssp-core/tests/`

#### 1.1 BMSSP Algorithm Tests (`bmssp_tests.rs`)

Tests the BMSSP algorithm implementation directly:

- `test_bmssp_simple`: Single edge graph
- `test_bmssp_with_preds`: Predecessor tracking
- `test_bmssp_chain`: Linear chain graph
- `test_bmssp_grid_2x2`: 2x2 grid graph
- `test_bmssp_disconnected`: Graph with unreachable vertices
- `test_bmssp_enabled_mask`: Enabled mask functionality
- `test_bmssp_cycle`: Graph with cycles
- `test_bmssp_multiple_paths`: Multiple paths to same vertex
- `test_bmssp_predecessor_path_reconstruction`: Verify path reconstruction correctness
- `test_bmssp_predecessor_source`: Source vertex predecessor handling
- `test_bmssp_predecessor_unreachable`: Unreachable vertex handling

**Run:**
```bash
cd rust/bmssp-core
cargo test bmssp_tests
```

**Expected:** All tests pass.

#### 1.2 Graph Tests (`graph_tests.rs`)

Tests CSR graph representation (existing tests).

**Run:**
```bash
cd rust/bmssp-core
cargo test graph_tests
```

#### 1.3 Dijkstra Tests (`dijkstra_tests.rs`)

Tests Dijkstra implementation used as oracle (existing tests).

**Run:**
```bash
cd rust/bmssp-core
cargo test dijkstra_tests
```

### 2. Rust Parity Tests

Location: `rust/bmssp-core/tests/golden_tests.rs`

These tests compare BMSSP results against Dijkstra on various graph types:

- `test_golden_small`: Small random graph (10 vertices, 20 edges)
- `test_golden_grid`: 4x4 grid graph
- `test_parity_random_graphs`: Multiple random graphs (10-200 vertices)
- `test_parity_multiple_sources`: Multiple source vertices per graph
- `test_parity_grid_graphs`: Multiple grid sizes (2x2 to 5x5)
- `test_parity_pipeline_like`: Chain graph with sparse bypass links
- `test_parity_with_enabled_mask`: Enabled mask patterns
- `test_parity_predecessors`: Predecessor correctness verification
- `test_parity_sparse_graph`: Very sparse graph (E ≈ 2V)
- `test_parity_dense_graph`: Moderately dense graph (E ≈ 5V)
- `test_parity_weight_variations`: Different weight distributions

**Run:**
```bash
cd rust/bmssp-core
cargo test golden_tests
```

**Expected:** All tests pass. Distances match within floating point tolerance (relative tolerance ~1e-5).

**Success Criteria:**
- BMSSP distances match Dijkstra distances exactly (within tolerance)
- Infinity handling is correct (unreachable vertices)
- Predecessor paths reconstruct to correct distances

### 3. Python Integration Tests

Location: `python/tests/`

#### 3.1 BMSSP Parity Tests (`test_bmssp_parity.py`)

Python-level tests comparing BMSSP against expected results:

- `test_parity_simple`: Simple graph correctness
- `test_parity_with_predecessors`: Predecessor reconstruction
- `test_parity_grid`: Grid graph
- `test_parity_vs_scipy`: Comparison with SciPy's Dijkstra (requires SciPy)
- `test_parity_large_graphs`: Larger graphs (50-200 vertices)
- `test_parity_enabled_mask`: Enabled mask functionality
- `test_parity_f64`: f64 precision testing
- `test_parity_multiple_sources`: Multiple source vertices
- `test_parity_path_reconstruction`: Path reconstruction correctness
- `test_parity_grid_large`: Larger grid graph (5x5)

**Run:**
```bash
cd python
pytest tests/test_bmssp_parity.py -v
```

**Expected:** All tests pass. Distances match expected values or SciPy results.

#### 3.2 Other Python Tests

- `test_graph.py`: Graph construction tests
- `test_sssp.py`: SSSP API tests
- `test_scenarios.py`: Scenario utility tests
- `test_dijkstra_parity.py`: Python-level Dijkstra parity tests

**Run:**
```bash
cd python
pytest tests/ -v
```

## Running All Tests

### Rust Tests

```bash
cd rust/bmssp-core
cargo test --lib          # Unit tests
cargo test --test '*'     # All integration tests
```

### Python Tests

```bash
cd python
pytest tests/ -v
```

### Full Test Suite

From project root:

```bash
# Rust tests
cd rust/bmssp-core && cargo test && cd ../..

# Python tests (requires built bindings)
cd python && pytest tests/ -v && cd ..
```

## Test Execution Order

Recommended order for running tests:

1. **Graph tests**: Verify graph structure is correct
2. **Dijkstra tests**: Verify oracle implementation is correct
3. **BMSSP unit tests**: Verify BMSSP implementation works on simple cases
4. **Rust parity tests**: Verify BMSSP matches Dijkstra on diverse graphs
5. **Python integration tests**: Verify end-to-end API correctness

## Floating Point Tolerance

Tests use relative tolerance for floating point comparisons:

- **Rust**: Relative tolerance ~1e-5 (100 * Float::epsilon())
- **Python**: Relative tolerance 1e-5 to 1e-6

For infinity comparisons:
- Both algorithms should agree on which vertices are unreachable
- Unreachable vertices should have `dist[v] == infinity`

## Success Criteria

Milestone 2 tests pass when:

1. ✅ All Rust unit tests pass
2. ✅ All Rust parity tests pass (BMSSP matches Dijkstra)
3. ✅ All Python unit tests pass
4. ✅ All Python parity tests pass
5. ✅ Predecessor tracking produces valid paths
6. ✅ Enabled mask functionality works correctly
7. ✅ Both f32 and f64 precisions work correctly

## Known Limitations

- Tests require Rust toolchain to be installed
- Python tests require `maturin develop` to be run first to build bindings
- SciPy comparison tests require SciPy to be installed (optional)
- Very large graphs (>1000 vertices) are not extensively tested in unit tests (benchmarks cover these)

## Troubleshooting

### Tests Fail with Compilation Errors

- Ensure Rust toolchain is installed: `rustc --version`
- Run `cargo clean` and rebuild: `cargo test`

### Python Tests Fail with ImportError

- Build Python bindings: `cd python && maturin develop`
- Activate virtual environment if using one

### Parity Tests Fail (BMSSP doesn't match Dijkstra)

- Check that BMSSP implementation is correct
- Verify edge relaxation logic
- Check enabled mask handling
- Ensure predecessor updates happen correctly

### Floating Point Precision Issues

- Use relative tolerance, not absolute
- Handle infinity comparisons separately
- Consider both f32 and f64 precisions

## Test Coverage

### Graph Types Covered

- Small graphs (2-10 vertices)
- Medium graphs (10-100 vertices)
- Large graphs (100-200 vertices)
- Sparse graphs (E ≈ 2V)
- Dense graphs (E ≈ 5V)
- Grid graphs (2x2 to 5x5)
- Chain/pipeline graphs
- Random graphs
- Graphs with cycles
- Disconnected graphs

### Features Tested

- Basic SSSP computation
- Predecessor tracking
- Path reconstruction
- Enabled mask (outage simulation)
- Multiple source vertices
- f32 and f64 precision
- Infinity handling (unreachable vertices)
- Edge cases (single vertex, single edge, etc.)

## Performance Notes

These tests focus on **correctness**, not performance. Performance benchmarks are in:
- `rust/bmssp-core/benches/` - Rust-level benchmarks
- `python/benchmarks/` - Python-level benchmarks

Run benchmarks separately:
```bash
cd rust/bmssp-core
cargo bench

cd python
pytest benchmarks/ --benchmark-only
```

## Maintenance

When adding new tests:

1. Add Rust unit tests to `bmssp_tests.rs` for new algorithm features
2. Add Rust parity tests to `golden_tests.rs` for new graph types
3. Add Python tests to `test_bmssp_parity.py` for new API features
4. Update this document with new test descriptions
5. Ensure tests follow existing patterns for consistency
