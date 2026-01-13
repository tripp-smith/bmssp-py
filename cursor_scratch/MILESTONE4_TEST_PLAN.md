# Milestone 4 Test Plan

This document describes the comprehensive regression test suite for Milestone 4, which implements performance optimizations including FastBlockHeap, state reuse APIs, and feature flags. This test plan includes all tests from Milestones 2 and 3 as regression tests, plus new tests specific to Milestone 4 features.

## Overview

The test suite verifies that performance optimizations maintain correctness. Tests are organized into:

1. **Regression Tests**: All tests from Milestone 2 (algorithm correctness) and Milestone 3 (scenarios)
2. **FastBlockHeap Tests**: New heap implementation correctness
3. **State Reuse Tests**: BmsspState and state-based API tests
4. **Integration Tests**: Ensuring optimizations work with existing features
5. **Performance Tests**: Benchmarking (separate from correctness tests)

## Test Categories

### 1. Regression Tests from Milestone 2

All tests from Milestone 2 must continue to pass to ensure algorithm correctness is maintained after switching from BlockHeap to FastBlockHeap.

#### 1.1 Rust Unit Tests (Regression)

**Location**: `rust/bmssp-core/tests/bmssp_tests.rs`

All existing tests must pass:
- `test_bmssp_simple`: Single edge graph
- `test_bmssp_with_preds`: Predecessor tracking
- `test_bmssp_chain`: Linear chain graph
- `test_bmssp_grid_2x2`: 2x2 grid graph
- `test_bmssp_disconnected`: Graph with unreachable vertices
- `test_bmssp_enabled_mask`: Enabled mask functionality
- `test_bmssp_cycle`: Graph with cycles
- `test_bmssp_multiple_paths`: Multiple paths to same vertex
- `test_bmssp_predecessor_path_reconstruction`: Path reconstruction correctness
- `test_bmssp_predecessor_source`: Source vertex predecessor handling
- `test_bmssp_predecessor_unreachable`: Unreachable vertex handling

**Run:**
```bash
cd rust/bmssp-core
cargo test bmssp_tests
```

**Expected:** All tests pass (same results as Milestone 2).

#### 1.2 Rust Parity Tests (Regression)

**Location**: `rust/bmssp-core/tests/golden_tests.rs`

All parity tests comparing BMSSP vs Dijkstra must still pass:
- `test_golden_small`: Small random graph
- `test_golden_grid`: 4x4 grid graph
- `test_parity_random_graphs`: Multiple random graphs
- `test_parity_multiple_sources`: Multiple source vertices
- `test_parity_grid_graphs`: Multiple grid sizes
- `test_parity_pipeline_like`: Chain graph with sparse bypass links
- `test_parity_with_enabled_mask`: Enabled mask patterns
- `test_parity_predecessors`: Predecessor correctness
- `test_parity_sparse_graph`: Very sparse graph
- `test_parity_dense_graph`: Moderately dense graph
- `test_parity_weight_variations`: Different weight distributions

**Run:**
```bash
cd rust/bmssp-core
cargo test golden_tests
```

**Expected:** All tests pass. BMSSP (now using FastBlockHeap) must still match Dijkstra within floating point tolerance.

#### 1.3 Python Integration Tests (Regression)

**Location**: `python/tests/test_bmssp_parity.py`

All Python parity tests must still pass:
- `test_parity_simple`: Simple graph correctness
- `test_parity_with_predecessors`: Predecessor reconstruction
- `test_parity_grid`: Grid graph
- `test_parity_vs_scipy`: Comparison with SciPy
- `test_parity_large_graphs`: Larger graphs
- `test_parity_enabled_mask`: Enabled mask functionality
- `test_parity_f64`: f64 precision testing
- `test_parity_multiple_sources`: Multiple source vertices
- `test_parity_path_reconstruction`: Path reconstruction
- `test_parity_grid_large`: Larger grid graph

**Run:**
```bash
cd python
pytest tests/test_bmssp_parity.py -v
```

**Expected:** All tests pass (same results as Milestone 2).

### 2. Regression Tests from Milestone 3

All scenario tests must continue to pass to ensure performance optimizations don't break scenario functionality.

#### 2.1 Scenario Tests (Regression)

**Location**: `python/tests/test_scenarios.py`

All scenario tests must pass:
- Weight model tests: `test_weight_model_single`, `test_weight_model_per_edge_attributes`, `test_weight_model_various_flows`, `test_weight_model_edge_cases`
- Outage tests: `test_apply_outage_mask`, `test_apply_outage_ids`, `test_scenario_outage_reachability`, `test_outage_reachability`, `test_outage_disconnected_components`, `test_outage_all_edges_to_vertex`, `test_outage_path_changes`
- Path reconstruction tests: `test_path_reconstruction_after_outage`, `test_path_cost_matches_distance`, `test_multiple_sinks_path_reconstruction`
- Dynamic weight tests: `test_dynamic_weights_multiple_updates`, `test_dynamic_weights_with_outages`
- Integration tests: `test_scenario_loop_end_to_end`, `test_multiple_scenarios_sequence`

**Run:**
```bash
cd python
pytest tests/test_scenarios.py -v
```

**Expected:** All tests pass (same results as Milestone 3).

### 3. FastBlockHeap Tests (New)

Tests for the new FastBlockHeap implementation to ensure it works correctly.

#### 3.1 FastBlockHeap Unit Tests

**Location**: `rust/bmssp-core/src/block_heap.rs` (in `fast_block_heap_tests` module)

**Existing Tests:**
- `test_fast_push_pop`: Basic push and pop operations
- `test_fast_decrease_key`: Decrease-key operations
- `test_fast_is_empty`: Empty heap detection
- `test_fast_min_distance`: Minimum distance retrieval

**Additional Tests to Add:**
- `test_fast_block_extraction`: Extract blocks of various sizes
- `test_fast_stale_entries`: Verify stale entry handling works correctly
- `test_fast_large_heap`: Test with larger heaps (100+ vertices)
- `test_fast_multiple_decrease_keys`: Multiple decrease-key operations on same vertex
- `test_fast_ordering`: Verify block extraction maintains distance ordering

**Run:**
```bash
cd rust/bmssp-core
cargo test fast_block_heap_tests
```

**Expected:** All tests pass.

#### 3.2 FastBlockHeap vs BlockHeap Parity Tests (New)

**Location**: `rust/bmssp-core/tests/` (new file: `fast_block_heap_parity_tests.rs` or add to existing test file)

Tests to ensure FastBlockHeap produces identical results to BlockHeap:

- `test_heap_parity_simple`: Compare results on simple graph
- `test_heap_parity_random_graphs`: Compare on random graphs
- `test_heap_parity_complex`: Compare on complex graphs with many decrease-key operations
- `test_heap_parity_block_sizes`: Test with different block sizes
- `test_heap_parity_edge_cases`: Edge cases (empty heap, single vertex, etc.)

**Implementation:** Run same BMSSP computation twice - once with BlockHeap (if still accessible) or compare against known correct results from Milestone 2.

**Run:**
```bash
cd rust/bmssp-core
cargo test fast_block_heap_parity  # or similar
```

**Expected:** FastBlockHeap produces identical results to BlockHeap (or matches expected results).

### 4. BmsspState Tests (New)

Tests for the new state reuse API.

#### 4.1 BmsspState Unit Tests

**Location**: `rust/bmssp-core/src/bmssp.rs` (in `tests` module)

**Existing Tests:**
- `test_bmssp_state`: Basic state-based SSSP
- `test_bmssp_state_with_preds`: State-based SSSP with predecessors
- `test_bmssp_state_reuse`: State reuse across multiple calls

**Additional Tests to Add:**
- `test_bmssp_state_reset`: Verify reset() works correctly
- `test_bmssp_state_resize`: Test state resizing for larger graphs
- `test_bmssp_state_multiple_sources`: Multiple sources with same state
- `test_bmssp_state_with_enabled_mask`: State API with enabled masks
- `test_bmssp_state_lifetime`: Verify lifetime constraints work correctly
- `test_bmssp_state_vs_regular_api`: Compare state API results with regular API

**Run:**
```bash
cd rust/bmssp-core
cargo test bmssp --lib
```

**Expected:** All tests pass.

#### 4.2 State API Parity Tests (New)

**Location**: `rust/bmssp-core/tests/` (new file or add to existing)

Tests to ensure state-based API produces identical results to regular API:

- `test_state_api_parity_simple`: Compare state API vs regular API on simple graphs
- `test_state_api_parity_random`: Compare on random graphs
- `test_state_api_parity_with_preds`: Compare with predecessor tracking
- `test_state_api_parity_with_enabled_mask`: Compare with enabled masks
- `test_state_api_parity_large_graphs`: Compare on larger graphs

**Run:**
```bash
cd rust/bmssp-core
cargo test state_api_parity  # or similar
```

**Expected:** State-based API produces identical results to regular API.

### 5. Integration Tests (New)

Tests ensuring new features work with existing functionality.

#### 5.1 State API with Scenarios (New)

**Location**: `python/tests/test_scenarios.py` (add new tests)

Tests combining state reuse with scenario features:

- `test_state_api_with_weight_updates`: Use state API with dynamic weight updates
- `test_state_api_with_outages`: Use state API with outage simulation
- `test_state_api_scenario_loop`: Complete scenario loop using state API
- `test_state_api_performance`: Verify state reuse improves performance (optional, more of a benchmark)

**Run:**
```bash
cd python
pytest tests/test_scenarios.py -k "state_api" -v
```

**Expected:** All tests pass.

#### 5.2 FastBlockHeap Integration (Regression)

Ensure FastBlockHeap works correctly in all existing scenarios:

- Run all Milestone 2 parity tests (FastBlockHeap is used internally)
- Run all Milestone 3 scenario tests (FastBlockHeap is used internally)
- Verify no performance regressions in correctness (tests should complete in reasonable time)

**Expected:** All existing tests pass, no new failures.

### 6. Feature Flag Tests (New)

Tests for feature flags (though limited due to generic design).

#### 6.1 Feature Flag Compilation Tests

**Location**: Manual testing or CI configuration

Test that code compiles with different feature flag combinations:

- Default features (`f32`, `f64`)
- `f32-only` feature (if implemented)
- `f64-only` feature (if implemented)
- `no-predecessors` feature (if implemented)
- `fast-math` feature (if implemented)

**Run:**
```bash
cd rust/bmssp-core
cargo build --features "f32-only"
cargo build --features "f64-only"
# etc.
```

**Expected:** Code compiles with all feature flag combinations (some may be placeholders).

### 7. Performance Tests (Benchmarks)

Performance benchmarks (separate from correctness tests).

#### 7.1 BlockHeap Benchmark

**Location**: `rust/bmssp-core/benches/block_heap_bench.rs`

Benchmarks comparing heap implementations:
- `bench_push_operations`: Push performance
- `bench_decrease_key_operations`: Decrease-key performance
- `bench_pop_block_operations`: Block extraction performance
- `bench_mixed_workload`: Mixed operations performance

**Run:**
```bash
cd rust/bmssp-core
cargo bench --bench block_heap_bench
```

**Expected:** Benchmarks run successfully. FastBlockHeap should show performance improvements over BlockHeap.

#### 7.2 State API Performance

**Location**: `rust/bmssp-core/benches/` or `python/benchmarks/`

Benchmarks comparing state API vs regular API:
- Repeated SSSP calls with state reuse
- Repeated SSSP calls without state reuse
- Measure allocation differences

**Expected:** State API shows improved performance for repeated calls.

## Running All Tests

### Rust Tests

```bash
cd rust/bmssp-core

# All unit tests
cargo test --lib

# All integration tests
cargo test --test '*'

# All tests
cargo test
```

### Python Tests

```bash
cd python

# All tests
pytest tests/ -v

# Scenario tests only
pytest tests/test_scenarios.py -v

# Parity tests only
pytest tests/test_bmssp_parity.py -v
```

### Full Regression Test Suite

From project root:

```bash
# Rust tests
cd rust/bmssp-core && cargo test && cd ../..

# Python tests (requires built bindings)
cd python && pytest tests/ -v && cd ..
```

## Test Execution Order

Recommended order for comprehensive testing:

1. **FastBlockHeap unit tests**: Verify new heap implementation works
2. **BmsspState unit tests**: Verify state API works
3. **FastBlockHeap parity tests**: Verify FastBlockHeap matches BlockHeap behavior
4. **State API parity tests**: Verify state API matches regular API
5. **Milestone 2 regression tests**: Verify algorithm correctness maintained
6. **Milestone 3 regression tests**: Verify scenario functionality maintained
7. **Integration tests**: Verify new features work with existing features
8. **Performance benchmarks**: Measure performance improvements (optional for correctness)

## Success Criteria

Milestone 4 tests pass when:

1. ✅ All Milestone 2 regression tests pass (algorithm correctness maintained)
2. ✅ All Milestone 3 regression tests pass (scenario functionality maintained)
3. ✅ All FastBlockHeap tests pass (new heap implementation works correctly)
4. ✅ FastBlockHeap parity tests pass (produces same results as BlockHeap)
5. ✅ All BmsspState tests pass (state API works correctly)
6. ✅ State API parity tests pass (produces same results as regular API)
7. ✅ Integration tests pass (new features work with existing features)
8. ✅ No performance regressions in correctness (tests complete in reasonable time)
9. ✅ Feature flags compile correctly (if applicable)

## Breaking Changes Analysis

### API Changes

**No breaking changes to existing APIs:**
- `bmssp_sssp()` - Still works the same way
- `bmssp_sssp_with_preds()` - Still works the same way
- All Python APIs - Unchanged

**New APIs added (non-breaking):**
- `BmsspState<T>` - New type
- `bmssp_sssp_with_state()` - New function
- `bmssp_sssp_with_preds_and_state()` - New function
- `FastBlockHeap<T>` - New type (also exported)

**Internal changes (should be transparent):**
- BMSSP algorithm now uses `FastBlockHeap` internally instead of `BlockHeap`
- `BlockHeap` is still available as a public type (backward compatibility)

### Regression Test Coverage

All existing tests should continue to work without modification because:
- Public APIs are unchanged
- Internal heap implementation change should be transparent (same results)
- New APIs are additions, not replacements

## Test Coverage Summary

### New Functionality Tested

- ✅ FastBlockHeap implementation correctness
- ✅ FastBlockHeap vs BlockHeap parity
- ✅ BmsspState creation and management
- ✅ State-based API correctness
- ✅ State reuse across multiple calls
- ✅ State API with scenarios (weight updates, outages)
- ✅ State API parity with regular API

### Regression Coverage

- ✅ All Milestone 2 algorithm correctness tests
- ✅ All Milestone 2 parity tests (BMSSP vs Dijkstra)
- ✅ All Milestone 3 scenario tests
- ✅ All Milestone 3 integration tests
- ✅ Python API tests
- ✅ Enabled mask functionality
- ✅ Predecessor tracking
- ✅ Path reconstruction

## Known Limitations and Notes

- **Feature flags**: Most feature flags are placeholders due to generic code design
- **Pairing heap**: Not implemented (deferred), only FastBlockHeap (BinaryHeap-based) is available
- **Performance benchmarks**: Separate from correctness tests, run with `cargo bench`
- **State API**: Requires pre-allocating state for largest graph size
- **BlockHeap**: Still available but no longer used internally by BMSSP

## Troubleshooting

### Tests Fail After FastBlockHeap Integration

**Problem:** Tests that previously passed now fail

**Solution:**
- Verify FastBlockHeap produces same results as BlockHeap
- Check that FastBlockHeap parity tests pass
- Ensure FastBlockHeap handles all edge cases correctly
- Verify stale entry handling works correctly

### State API Tests Fail

**Problem:** State-based API produces different results

**Solution:**
- Verify state is reset correctly between calls
- Check state resizing logic for larger graphs
- Verify lifetime constraints are handled correctly
- Compare state API results with regular API results

### Performance Regression in Tests

**Problem:** Tests take longer to run (correctness, not benchmarks)

**Solution:**
- Check if FastBlockHeap has performance issues
- Verify no infinite loops or excessive allocations
- Profile test execution to find bottlenecks
- Consider if regression is acceptable (correctness more important than performance in tests)

## Test Maintenance

When adding new tests:

1. Follow existing test patterns for consistency
2. Add regression tests for any changes to core algorithms
3. Add parity tests when implementing alternative implementations
4. Update this document with new test descriptions
5. Ensure tests are independent (no shared state)
6. Use descriptive test names
7. Group related tests appropriately

## Related Documentation

- **Milestone 2 Test Plan**: `cursor_scratch/MILESTONE2_TEST_PLAN.md`
- **Milestone 3 Test Plan**: `cursor_scratch/MILESTONE3_TEST_PLAN.md`
- **Milestone 4 Plan**: `cursor_scratch/MILESTONE4_PLAN.md`
- **Performance Guide**: `docs/performance.md`
- **API Reference**: `docs/api.md`
