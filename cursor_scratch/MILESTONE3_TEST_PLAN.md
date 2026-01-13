# Milestone 3 Test Plan

This document describes the comprehensive test suite for Milestone 3, which implements scenario-based features including dynamic weight updates, outage simulation, and path reconstruction for grid and pipeline network optimization.

## Overview

The test suite verifies that scenario utilities (`weight_model`, `apply_outage`) and their integration with the SSSP API work correctly. Tests focus on:

1. **Weight Model Tests**: Dynamic weight computation from flow and edge attributes
2. **Outage Tests**: Edge disabling via enabled masks
3. **Path Reconstruction Tests**: Path reconstruction in scenario contexts
4. **Integration Tests**: Complete scenario workflows combining multiple features
5. **Dynamic Weight Tests**: Multiple weight update scenarios

Tests are located in: `python/tests/test_scenarios.py`

## Test Categories

### 1. Weight Model Tests

Tests for the `weight_model()` function that computes effective weights from flow and edge attributes.

#### 1.1 Basic Weight Model Tests

**`test_weight_model_single()`**
- Tests weight model with single `EdgeAttributes` for all edges
- Verifies formula: `w = base_cost * risk * (1 + alpha * (flow/capacity)^2)`
- Tests zero flow, partial flow, and full capacity flow

**`test_weight_model_per_edge_attributes()`**
- Tests weight model with array of `EdgeAttributes` (one per edge)
- Verifies each edge uses its own attributes correctly
- Tests different combinations of base_cost, capacity, and risk

**Run:**
```bash
cd python
pytest tests/test_scenarios.py::test_weight_model_single -v
pytest tests/test_scenarios.py::test_weight_model_per_edge_attributes -v
```

#### 1.2 Weight Model Flow Scenarios

**`test_weight_model_various_flows()`**
- Tests weight model with various flow levels:
  - Zero flow (no congestion)
  - Low flow (minimal congestion)
  - High flow (near capacity)
  - Overflow (exceeds capacity)
- Verifies congestion increases with flow

**`test_weight_model_edge_cases()`**
- Tests edge cases:
  - Zero capacity (should handle gracefully)
  - Very small capacity
- Verifies division by zero is avoided

**Run:**
```bash
cd python
pytest tests/test_scenarios.py::test_weight_model_various_flows -v
pytest tests/test_scenarios.py::test_weight_model_edge_cases -v
```

### 2. Outage Tests

Tests for the `apply_outage()` function and outage scenarios using enabled masks.

#### 2.1 Basic Outage Tests

**`test_apply_outage_mask()`**
- Tests applying outage using boolean mask
- Verifies disabled edges have infinite weight and enabled mask is correct
- Verifies non-disabled edges remain unchanged

**`test_apply_outage_ids()`**
- Tests applying outage using edge ID array
- Verifies edge IDs are correctly mapped to disabled edges
- Verifies enabled mask matches disabled edges

**Run:**
```bash
cd python
pytest tests/test_scenarios.py::test_apply_outage_mask -v
pytest tests/test_scenarios.py::test_apply_outage_ids -v
```

#### 2.2 Outage Scenario Tests

**`test_scenario_outage_reachability()`**
- Tests that outages change reachability correctly
- Before outage: vertex reachable via multiple paths
- After single edge outage: vertex still reachable via alternative path
- After blocking all paths: vertex becomes unreachable

**`test_outage_reachability()`**
- Tests various reachability scenarios:
  - Single edge outage (vertex still reachable)
  - All edges from source disabled (vertices become unreachable)
  - All edges to vertex disabled (vertex becomes unreachable)

**`test_outage_disconnected_components()`**
- Tests outages that create disconnected graph components
- Disables bridge edge connecting two components
- Verifies vertices in disconnected component become unreachable

**`test_outage_all_edges_to_vertex()`**
- Tests disabling all incoming edges to a vertex
- Verifies vertex becomes unreachable
- Verifies dependent vertices also become unreachable

**Run:**
```bash
cd python
pytest tests/test_scenarios.py::test_scenario_outage_reachability -v
pytest tests/test_scenarios.py::test_outage_reachability -v
pytest tests/test_scenarios.py::test_outage_disconnected_components -v
pytest tests/test_scenarios.py::test_outage_all_edges_to_vertex -v
```

#### 2.3 Outage Path Changes

**`test_outage_path_changes()`**
- Tests that paths change after outages
- Computes initial path
- Applies outage that blocks initial path
- Verifies new path uses alternative route
- Verifies path doesn't use disabled edges

**Run:**
```bash
cd python
pytest tests/test_scenarios.py::test_outage_path_changes -v
```

### 3. Path Reconstruction Tests

Tests for path reconstruction in scenario contexts, including after outages and weight updates.

**`test_path_reconstruction_after_outage()`**
- Tests path reconstruction after outage changes route
- Before outage: path uses one route
- After outage: path uses alternative route
- Verifies paths are correctly reconstructed in both cases

**`test_path_cost_matches_distance()`**
- Tests that reconstructed path costs match computed distances
- Reconstructs path from predecessors
- Manually computes path cost by summing edge weights
- Verifies path cost equals computed distance (within tolerance)

**`test_multiple_sinks_path_reconstruction()`**
- Tests path reconstruction for multiple sinks from same source
- Reconstructs paths to multiple target vertices
- Verifies each path is correct and independent
- Verifies paths differ when appropriate

**Run:**
```bash
cd python
pytest tests/test_scenarios.py::test_path_reconstruction_after_outage -v
pytest tests/test_scenarios.py::test_path_cost_matches_distance -v
pytest tests/test_scenarios.py::test_multiple_sinks_path_reconstruction -v
```

### 4. Dynamic Weight Tests

Tests for multiple weight updates and weight change scenarios.

**`test_dynamic_weights_multiple_updates()`**
- Tests multiple weight updates in sequence
- Initial SSSP computation
- Update weights (make different path optimal)
- Update weights again (change optimal path)
- Verifies graph topology remains unchanged
- Verifies paths change as expected with weight changes

**`test_dynamic_weights_with_outages()`**
- Tests weight updates combined with enabled masks
- Updates weights and applies outage simultaneously
- Verifies both effects are correctly applied
- Verifies computation completes without error

**Run:**
```bash
cd python
pytest tests/test_scenarios.py::test_dynamic_weights_multiple_updates -v
pytest tests/test_scenarios.py::test_dynamic_weights_with_outages -v
```

### 5. Integration Tests

Tests for complete scenario workflows combining multiple features.

**`test_scenario_loop_end_to_end()`**
- Tests full scenario loop:
  1. Build graph
  2. Compute initial SSSP
  3. Update weights (simulate congestion)
  4. Recompute SSSP
  5. Apply outage
  6. Recompute SSSP
- Verifies all steps complete without error
- Verifies scenario workflow is correct

**`test_multiple_scenarios_sequence()`**
- Tests multiple scenarios in sequence
- Runs multiple scenarios with different weight configurations
- Verifies scenarios are independent
- Verifies costs vary appropriately
- Verifies all scenarios remain reachable

**Run:**
```bash
cd python
pytest tests/test_scenarios.py::test_scenario_loop_end_to_end -v
pytest tests/test_scenarios.py::test_multiple_scenarios_sequence -v
```

## Running Tests

### All Scenario Tests

```bash
cd python
pytest tests/test_scenarios.py -v
```

### Specific Test Categories

```bash
# Weight model tests only
pytest tests/test_scenarios.py -k "weight_model" -v

# Outage tests only
pytest tests/test_scenarios.py -k "outage" -v

# Path reconstruction tests only
pytest tests/test_scenarios.py -k "path_reconstruction" -v

# Integration tests only
pytest tests/test_scenarios.py -k "scenario_loop or multiple_scenarios" -v
```

### Individual Tests

```bash
pytest tests/test_scenarios.py::test_weight_model_single -v
pytest tests/test_scenarios.py::test_apply_outage_mask -v
# ... etc
```

### With Coverage

```bash
cd python
pytest tests/test_scenarios.py --cov=bmssp.scenario --cov-report=html
```

## Test Execution Order

Recommended order for running tests:

1. **Basic utility tests**: `test_weight_model_single`, `test_apply_outage_mask`, `test_apply_outage_ids`
2. **Weight model tests**: All weight_model tests
3. **Basic outage tests**: Outage application and reachability
4. **Path reconstruction tests**: Verify paths are correct
5. **Dynamic weight tests**: Multiple update scenarios
6. **Integration tests**: Complete workflows

This order ensures basic functionality works before testing complex scenarios.

## Success Criteria

Milestone 3 tests pass when:

1. ✅ All weight model tests pass (formula correctness, edge cases)
2. ✅ All outage tests pass (mask/ID application, reachability changes)
3. ✅ All path reconstruction tests pass (paths match distances, correct after changes)
4. ✅ All dynamic weight tests pass (multiple updates work correctly)
5. ✅ All integration tests pass (complete workflows work end-to-end)
6. ✅ Enabled masks correctly disable edges in SSSP computation
7. ✅ Weight updates don't require graph topology changes
8. ✅ Path reconstruction works correctly after outages and weight changes

## Test Coverage Analysis

### Covered Areas

**Weight Model:**
- ✅ Single attributes for all edges
- ✅ Per-edge attributes
- ✅ Various flow levels (zero, low, high, overflow)
- ✅ Edge cases (zero capacity, small capacity)
- ✅ Congestion formula correctness

**Outage Simulation:**
- ✅ Boolean mask application
- ✅ Edge ID array application
- ✅ Single edge outages
- ✅ Multiple edge outages
- ✅ All edges from source disabled
- ✅ All edges to vertex disabled
- ✅ Disconnected components
- ✅ Path changes after outages
- ✅ Reachability changes

**Path Reconstruction:**
- ✅ Path reconstruction after outages
- ✅ Path cost matches distance
- ✅ Multiple sinks from same source
- ✅ Path correctness verification

**Dynamic Weights:**
- ✅ Multiple weight updates in sequence
- ✅ Weight updates with outages
- ✅ Graph topology immutability

**Integration:**
- ✅ Complete scenario loops
- ✅ Multiple scenarios in sequence

### Test Gaps and Recommendations

While the current test suite is comprehensive, the following areas could benefit from additional tests:

#### 1. Error Handling Tests

**Missing tests for invalid inputs:**

- `apply_outage()` error handling:
  - Invalid edge IDs (out of range, negative)
  - Wrong length edge_mask
  - Both edge_mask and edge_ids provided (should error)
  - Empty edge_ids array (edge case)
  - Empty edge_mask (all False - all edges disabled)

- `weight_model()` error handling:
  - Mismatched flow/attributes lengths (when using list)
  - Empty flow array
  - Negative flows (behavior should be documented/tested)
  - Mismatched array lengths

**Recommendation:** Add error handling tests to verify proper validation and error messages.

#### 2. Edge Case Tests

**Additional edge cases to consider:**

- Single vertex graph (no edges)
- Graph with no edges
- Source equals sink
- Empty outage (no edges disabled - should work but worth testing)
- All edges disabled (graph becomes isolated source)

**Recommendation:** Add edge case tests for robustness.

#### 3. Performance/Stress Tests

**Missing performance-related tests:**

- Large graph scenarios (hundreds of vertices)
- Many repeated weight updates (performance regression)
- Many outage scenarios in sequence (memory leaks)

**Note:** Performance benchmarks are separate from correctness tests, but basic performance tests could be valuable.

#### 4. Type/Dtype Tests

**Missing dtype-specific tests:**

- f32 vs f64 precision in weight_model
- Different integer types for edge IDs
- Mixed precision scenarios

**Recommendation:** Add dtype tests if precision issues are a concern.

## Troubleshooting

### Tests Fail with ImportError

**Problem:** Cannot import `bmssp` or `bmssp.scenario`

**Solution:**
- Build Python bindings: `cd python && maturin develop`
- Activate virtual environment if using one
- Verify installation: `python -c "import bmssp; import bmssp.scenario"`

### Tests Fail with AssertionErrors

**Problem:** Test assertions fail (incorrect results)

**Solution:**
- Check that scenario utilities are implemented correctly
- Verify weight_model formula matches expected behavior
- Verify apply_outage creates correct enabled masks
- Check that SSSP computation uses enabled masks correctly
- Verify path reconstruction logic

### Floating Point Precision Issues

**Problem:** Floating point comparisons fail

**Solution:**
- Use relative tolerance for comparisons (e.g., `abs(x - y) < 1e-6`)
- Handle infinity comparisons separately
- Consider both f32 and f64 precisions

### Outage Tests Fail (Paths Don't Change)

**Problem:** Outages don't affect paths as expected

**Solution:**
- Verify enabled mask is passed to `sssp()` function
- Check that edge IDs are correctly mapped
- Verify graph structure matches expectations
- Ensure outages actually block intended paths

### Weight Model Tests Fail (Incorrect Weights)

**Problem:** Computed weights don't match expected values

**Solution:**
- Verify weight model formula: `w = base_cost * risk * (1 + alpha * (flow/capacity)^2)`
- Check attribute values (base_cost, capacity, risk)
- Verify flow values are correct
- Check alpha parameter
- Handle division by zero (capacity = 0)

## Test Maintenance

When adding new tests:

1. Follow existing test patterns for consistency
2. Use descriptive test names: `test_<feature>_<scenario>()`
3. Add docstrings explaining what the test verifies
4. Use appropriate assertions with clear failure messages
5. Group related tests (use pytest markers if needed)
6. Update this document with new test descriptions
7. Ensure tests are independent (no shared state)

## Related Documentation

- **API Reference**: `docs/api.md` - Documents scenario utility APIs
- **Tutorial**: `docs/tutorial.md` - Includes scenario usage examples
- **Example**: `python/examples/grid_pipeline.py` - Complete scenario example
- **Milestone Plan**: `cursor_scratch/MILESTONE3_PLAN.md` - Implementation plan

## Comparison with Milestone 2 Tests

Milestone 2 focused on algorithm correctness (BMSSP vs Dijkstra parity). Milestone 3 focuses on:

- **Scenario utilities** (weight_model, apply_outage)
- **Dynamic weight updates** (multiple updates without graph rebuild)
- **Outage simulation** (enabled masks)
- **Path reconstruction in scenarios** (after outages and weight changes)
- **Integration workflows** (complete scenario loops)

Milestone 3 tests complement Milestone 2 tests by verifying scenario-specific functionality rather than algorithm correctness (which is already verified in Milestone 2).
