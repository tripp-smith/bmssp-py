# Milestone 3: Outages + Dynamic Weights Implementation Plan

## Overview

Milestone 3 focuses on completing the scenario-based features: outage simulation via enabled masks and dynamic weight updates. While much of the infrastructure is already in place from Milestones 1 and 2, this milestone ensures everything works end-to-end and provides a complete, polished example.

## Current State Assessment

### Already Implemented ✅

**Rust Core:**
- ✅ Enabled mask support in `bmssp_sssp()` and `bmssp_sssp_with_preds()`
- ✅ Enabled mask support in `dijkstra_sssp()` and `dijkstra_sssp_with_preds()`
- ✅ Predecessor tracking in BMSSP and Dijkstra
- ✅ Edge validation for enabled masks

**Python Bindings:**
- ✅ `enabled` parameter in `sssp_f32_csr()` and `sssp_f64_csr()`
- ✅ Predecessor arrays returned when requested
- ✅ Input validation for enabled masks

**Python Package:**
- ✅ `scenario.py` with `EdgeAttributes`, `weight_model()`, `apply_outage()`
- ✅ `reconstruct_path()` function implemented
- ✅ `grid_pipeline.py` example skeleton exists
- ✅ Tests for scenario utilities exist (`test_scenarios.py`)

### Gaps to Address ⚠️

1. **Example Completion**: `grid_pipeline.py` needs to be complete and demonstrate all features
2. **End-to-End Verification**: Ensure all components work together seamlessly
3. **Test Coverage**: Add comprehensive tests for dynamic weight scenarios
4. **Documentation**: Update docs to reflect complete scenario usage

## Phase 1: Example Completion and Enhancement

### Task 1.1: Complete grid_pipeline.py Example

**File**: `python/examples/grid_pipeline.py`

**Current State**: Skeleton exists but may need completion/enhancement

**Requirements** (from SPEC.md):
1. Build a directed graph for:
   - Grid-like distribution network
   - Pipeline network with compressor-like edges and bypasses
2. Edge attributes: `base_cost`, `risk`, `capacity`
3. Scenario loop:
   - Compute initial SSSP from supply node
   - Apply load flows to edges (simple synthetic flow model)
   - Update weights from congestion model
   - Recompute SSSP
   - Apply outage mask to subset of edges
   - Recompute SSSP
4. Outputs:
   - Cost-to-serve list of sink nodes
   - Reconstructed path(s) for at least one sink (before/after outage)
   - Basic timing for repeated calls

**Implementation Steps**:
- Verify existing functions work correctly
- Complete any missing functionality
- Add proper error handling
- Add timing measurements
- Add clear output formatting
- Ensure example is self-contained and runnable

### Task 1.2: Add Pipeline Network Example (Optional Enhancement)

**File**: `python/examples/grid_pipeline.py` (extend existing or create separate function)

**Purpose**: Demonstrate pipeline-specific features (compressor-like edges, bypasses)

**Implementation**:
- Create pipeline network builder function
- Model compressor edges (higher capacity, different cost structure)
- Add bypass links
- Demonstrate re-routing after outages

## Phase 2: End-to-End Verification

### Task 2.1: Verify Enabled Mask Flow

**Verification Points**:
1. Python API accepts enabled mask correctly
2. Enabled mask propagates through Python → Rust bindings → BMSSP/Dijkstra
3. Disabled edges are correctly skipped
4. Results match expected behavior (unreachable vertices, path changes)

**Test Cases**:
- Single edge outage
- Multiple edge outages
- All edges from a vertex disabled
- All edges to a vertex disabled
- Outage that creates disconnected components

### Task 2.2: Verify Dynamic Weight Updates

**Verification Points**:
1. Weight arrays can be updated in-place
2. Graph topology remains immutable
3. Multiple SSSP calls with different weights produce correct results
4. Scenario utilities (`weight_model`, `apply_outage`) integrate correctly

**Test Cases**:
- Update weights without topology change
- Weight updates that change optimal paths
- Extreme weight changes (very large/small weights)
- Weight updates combined with enabled masks

### Task 2.3: Verify Path Reconstruction

**Verification Points**:
1. Predecessors are correctly returned
2. `reconstruct_path()` works for all reachable vertices
3. Path costs match computed distances
4. Path reconstruction works after outages (when paths change)

**Test Cases**:
- Simple paths (linear chains)
- Complex paths (multiple options)
- Path reconstruction after outage changes route
- Multiple sinks from same source

## Phase 3: Test Coverage Expansion

### Task 3.1: Enhance Scenario Tests

**File**: `python/tests/test_scenarios.py`

**Current Tests**: Basic tests exist, but need expansion

**New Test Cases**:
- `test_dynamic_weights_multiple_updates()` - Multiple weight updates
- `test_outage_path_changes()` - Verify paths change after outages
- `test_weight_model_various_flows()` - Different flow scenarios
- `test_scenario_loop_end_to_end()` - Full scenario loop
- `test_outage_reachability()` - Reachability changes
- `test_weight_model_edge_cases()` - Zero flow, overflow capacity, etc.

### Task 3.2: Add Integration Tests

**File**: `python/tests/test_scenarios.py` (new section) or `test_integration.py` (new file)

**Purpose**: Test complete workflows combining multiple features

**Test Cases**:
- Full scenario: build graph → compute paths → update weights → recompute → apply outage → recompute
- Multiple scenarios in sequence
- Large graph scenario performance
- Memory usage (ensure no leaks with repeated calls)

### Task 3.3: Rust-Level Scenario Tests (Optional)

**File**: `rust/bmssp-core/tests/scenario_tests.rs` (new file, optional)

**Purpose**: Low-level tests for enabled mask and predecessor edge cases

**Test Cases**:
- Enabled mask with all edges disabled
- Enabled mask boundary conditions
- Predecessor updates during edge relaxation
- Predecessor correctness with cycles

## Phase 4: Documentation Updates

### Task 4.1: Update Tutorial

**File**: `docs/tutorial.md`

**Current State**: Tutorial exists but may need updates

**Updates Needed**:
- Verify all code examples work
- Add complete example showing full scenario loop
- Add section on outage simulation
- Add section on dynamic weight updates
- Include timing examples
- Add troubleshooting tips

### Task 4.2: Update API Documentation

**File**: `docs/api.md`

**Updates Needed**:
- Verify all API descriptions are accurate
- Add examples for enabled mask usage
- Add examples for dynamic scenarios
- Document `reconstruct_path()` usage
- Document scenario utilities thoroughly

### Task 4.3: Create Scenario Guide (Optional)

**File**: `docs/scenarios.md` (new file, optional)

**Purpose**: Comprehensive guide for scenario-based workflows

**Contents**:
- Overview of scenario modeling
- Building networks for scenarios
- Dynamic weight updates
- Outage simulation
- Performance considerations
- Best practices
- Common patterns

## Phase 5: Example Enhancements and Polish

### Task 5.1: Add Example Output Formatting

**File**: `python/examples/grid_pipeline.py`

**Enhancements**:
- Pretty-print results
- Visualize path changes (text-based)
- Summary statistics
- Comparison tables (before/after scenarios)

### Task 5.2: Add Example Command-Line Interface (Optional)

**File**: `python/examples/grid_pipeline.py`

**Enhancements**:
- Make example runnable from command line
- Accept parameters (graph size, number of scenarios, etc.)
- Output options (verbose, summary, JSON, etc.)

### Task 5.3: Add Performance Timing

**File**: `python/examples/grid_pipeline.py`

**Enhancements**:
- Measure time for each scenario step
- Compare times: initial vs. recomputed SSSP
- Show timing benefits of immutable graph topology
- Optional: compare with rebuilding graph vs. using enabled masks

## Success Criteria

Milestone 3 is complete when:

1. ✅ `grid_pipeline.py` example is complete and demonstrates all required features
2. ✅ Example runs successfully and produces expected output
3. ✅ All scenario features work end-to-end (enabled masks, dynamic weights, path reconstruction)
4. ✅ Comprehensive test coverage for scenario features
5. ✅ Documentation is updated and accurate
6. ✅ All tests pass (existing + new scenario tests)
7. ✅ Example demonstrates timing benefits of the design

## Files to Modify/Create

**Python Examples**:
- `python/examples/grid_pipeline.py` - Complete/enhance example

**Python Tests**:
- `python/tests/test_scenarios.py` - Expand test coverage
- `python/tests/test_integration.py` - New file (optional)

**Rust Tests** (Optional):
- `rust/bmssp-core/tests/scenario_tests.rs` - New file (optional)

**Documentation**:
- `docs/tutorial.md` - Update with complete examples
- `docs/api.md` - Update API docs
- `docs/scenarios.md` - New file (optional)

## Implementation Notes

### Design Principles

1. **Immutable Graph Topology**: Graph structure never changes; only weights and enabled masks change
2. **Fast Re-computation**: Multiple SSSP calls should be fast because graph topology is reused
3. **Clean API**: Scenario utilities should be intuitive and easy to use
4. **Performance Focus**: Highlight the performance benefits in the example

### Testing Strategy

- Test individual components (enabled mask, weight updates, path reconstruction)
- Test integration (components working together)
- Test edge cases (extreme scenarios, boundary conditions)
- Test performance (ensure no regressions)

### Documentation Strategy

- Start with simple examples
- Build up to complex scenarios
- Include real-world use cases
- Show performance benefits
- Provide troubleshooting guidance

## Dependencies

- Milestone 2 must be complete (BMSSP correctness verified)
- All existing tests must pass
- Example should work with both BMSSP and Dijkstra backends (if applicable)

## Next Steps After Milestone 3

Once Milestone 3 is complete, the project will have:
- Complete functionality for the target use case
- Working examples demonstrating the value proposition
- Comprehensive test coverage
- Good documentation

This sets up well for:
- **Milestone 4**: Performance optimization (can benchmark real scenarios)
- **Milestone 5**: Packaging and distribution (have complete, tested package)
