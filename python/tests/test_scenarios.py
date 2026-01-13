"""Tests for scenario utilities."""

import numpy as np
import pytest
from bmssp.scenario import EdgeAttributes, weight_model, apply_outage
from bmssp import Graph, sssp, reconstruct_path


def test_weight_model_single():
    """Test weight model with single attributes."""
    flow = np.array([0.5, 1.0, 0.0], dtype=np.float32)
    attrs = EdgeAttributes(base_cost=1.0, capacity=1.0, risk=1.0)
    weights = weight_model(flow, attrs, alpha=1.0)
    
    assert len(weights) == len(flow)
    assert weights[0] == 1.0 * (1.0 + 1.0 * 0.5**2)  # 1.25
    assert weights[1] == 1.0 * (1.0 + 1.0 * 1.0**2)  # 2.0
    assert weights[2] == 1.0  # No flow


def test_apply_outage_mask():
    """Test applying outage with mask."""
    weights = np.array([1.0, 2.0, 3.0], dtype=np.float32)
    edge_mask = np.array([True, False, True], dtype=bool)
    updated_weights, enabled = apply_outage(weights, edge_mask=edge_mask)
    
    assert np.isinf(updated_weights[0])
    assert updated_weights[1] == 2.0
    assert np.isinf(updated_weights[2])
    assert enabled is not None
    assert not enabled[0]
    assert enabled[1]
    assert not enabled[2]


def test_apply_outage_ids():
    """Test applying outage with edge IDs."""
    weights = np.array([1.0, 2.0, 3.0], dtype=np.float32)
    edge_ids = np.array([0, 2], dtype=np.int64)
    updated_weights, enabled = apply_outage(weights, edge_ids=edge_ids)
    
    assert np.isinf(updated_weights[0])
    assert updated_weights[1] == 2.0
    assert np.isinf(updated_weights[2])
    assert enabled is not None
    assert not enabled[0]
    assert enabled[1]
    assert not enabled[2]


def test_scenario_outage_reachability():
    """Test that outage changes reachability."""
    n = 4
    edges = np.array([
        [0, 1], [0, 2],
        [1, 3],
        [2, 3],
    ], dtype=np.int64)
    weights = np.array([1.0, 1.0, 1.0, 1.0], dtype=np.float32)
    
    graph, _ = Graph.from_edges(n, edges, weights=weights)
    
    # Before outage: can reach 3 via either path
    result_before = sssp(graph, weights, source=0)
    assert result_before.dist[3] == 2.0
    
    # Apply outage to edge 0->1
    edge_mask = np.array([True, False, False, False], dtype=bool)
    _, enabled = apply_outage(weights, edge_mask=edge_mask)
    
    # After outage: must use path 0->2->3
    # Use only the enabled mask, not the updated_weights with inf
    result_after = sssp(graph, weights, source=0, enabled=enabled)
    assert result_after.dist[3] == 2.0  # Still reachable via other path
    
    # Apply outage to both paths to 3
    # To block both paths (0->1->3 and 0->2->3), disable edges 2 and 3 (1->3 and 2->3)
    edge_mask = np.array([False, False, True, True], dtype=bool)
    _, enabled = apply_outage(weights, edge_mask=edge_mask)
    result_final = sssp(graph, weights, source=0, enabled=enabled)
    assert np.isinf(result_final.dist[3])  # No longer reachable


def test_weight_model_per_edge_attributes():
    """Test weight_model with array of attributes."""
    flow = np.array([0.5, 1.0, 0.0, 2.0], dtype=np.float32)
    attrs_list = [
        EdgeAttributes(base_cost=1.0, capacity=1.0, risk=1.0),
        EdgeAttributes(base_cost=2.0, capacity=1.0, risk=1.0),
        EdgeAttributes(base_cost=1.0, capacity=2.0, risk=1.0),
        EdgeAttributes(base_cost=1.0, capacity=1.0, risk=2.0),
    ]
    weights = weight_model(flow, attrs_list, alpha=1.0)
    
    assert len(weights) == len(flow)
    # Edge 0: base_cost=1.0, capacity=1.0, flow=0.5 -> weight = 1.0 * (1 + 0.5^2) = 1.25
    assert abs(weights[0] - 1.25) < 1e-6
    # Edge 1: base_cost=2.0, capacity=1.0, flow=1.0 -> weight = 2.0 * (1 + 1.0^2) = 4.0
    assert abs(weights[1] - 4.0) < 1e-6
    # Edge 2: base_cost=1.0, capacity=2.0, flow=0.0 -> weight = 1.0 * (1 + 0) = 1.0
    assert abs(weights[2] - 1.0) < 1e-6
    # Edge 3: base_cost=1.0, capacity=1.0, risk=2.0, flow=2.0 -> weight = 1.0 * 2.0 * (1 + 2.0^2) = 10.0
    assert abs(weights[3] - 10.0) < 1e-6


def test_weight_model_various_flows():
    """Test weight_model with various flow scenarios."""
    # Zero flow
    flow_zero = np.array([0.0, 0.0], dtype=np.float32)
    attrs = EdgeAttributes(base_cost=1.0, capacity=10.0, risk=1.0)
    weights_zero = weight_model(flow_zero, attrs, alpha=1.0)
    assert np.allclose(weights_zero, 1.0)
    
    # Low flow
    flow_low = np.array([2.0, 3.0], dtype=np.float32)
    weights_low = weight_model(flow_low, attrs, alpha=1.0)
    assert weights_low[0] > 1.0
    assert weights_low[1] > 1.0
    
    # High flow (near capacity)
    flow_high = np.array([9.0, 9.5], dtype=np.float32)
    weights_high = weight_model(flow_high, attrs, alpha=1.0)
    assert weights_high[0] > weights_low[0]
    assert weights_high[1] > weights_low[1]
    
    # Overflow (exceeds capacity)
    flow_overflow = np.array([15.0, 20.0], dtype=np.float32)
    weights_overflow = weight_model(flow_overflow, attrs, alpha=1.0)
    assert weights_overflow[0] > weights_high[0]
    assert weights_overflow[1] > weights_high[1]


def test_weight_model_edge_cases():
    """Test weight_model edge cases."""
    # Zero capacity (should handle gracefully)
    flow = np.array([1.0], dtype=np.float32)
    attrs_zero_cap = EdgeAttributes(base_cost=1.0, capacity=0.0, risk=1.0)
    weights = weight_model(flow, attrs_zero_cap, alpha=1.0)
    # Should use capacity > 0 check, so congestion term = 1.0
    assert weights[0] == 1.0
    
    # Very small capacity
    attrs_small_cap = EdgeAttributes(base_cost=1.0, capacity=0.001, risk=1.0)
    weights_small = weight_model(flow, attrs_small_cap, alpha=1.0)
    assert weights_small[0] > 1.0  # Should have congestion


def test_dynamic_weights_multiple_updates():
    """Test multiple weight updates in sequence."""
    n = 4
    edges = np.array([[0, 1], [1, 2], [2, 3], [0, 3]], dtype=np.int64)
    weights1 = np.array([1.0, 1.0, 1.0, 10.0], dtype=np.float32)
    
    graph, weights1_sorted = Graph.from_edges(n, edges, weights=weights1)
    
    # Initial SSSP - use sorted weights
    # After sorting: [0->1: 1.0, 0->3: 10.0, 1->2: 1.0, 2->3: 1.0]
    # Path 0->1->2->3 uses edges 0, 2, 3: 1.0 + 1.0 + 1.0 = 3.0
    result1 = sssp(graph, weights1_sorted, source=0, return_predecessors=True)
    assert result1.dist[3] == 3.0  # Path 0->1->2->3
    
    # Update weights - make direct edge cheaper (edge 1: 0->3)
    weights2 = weights1_sorted.copy()
    weights2[1] = 2.0  # Edge 0->3 becomes cheaper
    result2 = sssp(graph, weights2, source=0, return_predecessors=True)
    assert result2.dist[3] == 2.0  # Path 0->3
    
    # Update again - make middle edges very expensive (edges 2 and 3)
    weights3 = weights2.copy()
    weights3[2] = 100.0  # Edge 1->2
    weights3[3] = 100.0  # Edge 2->3
    result3 = sssp(graph, weights3, source=0, return_predecessors=True)
    assert result3.dist[3] == 2.0  # Still path 0->3 (direct)
    
    # Graph topology unchanged
    assert graph.num_vertices() == n
    assert graph.num_edges() == len(edges)


def test_outage_path_changes():
    """Test that paths change after outages."""
    n = 5
    edges = np.array([
        [0, 1], [0, 2],
        [1, 3], [2, 3],
        [3, 4],
    ], dtype=np.int64)
    weights = np.array([1.0, 1.0, 1.0, 1.0, 1.0], dtype=np.float32)
    
    graph, _ = Graph.from_edges(n, edges, weights=weights)
    
    # Initial path to 4: should be 0->1->3->4 or 0->2->3->4 (both cost 3.0)
    result_before = sssp(graph, weights, source=0, return_predecessors=True)
    assert result_before.dist[4] == 3.0
    path_before = reconstruct_path(result_before.pred, 4)
    assert len(path_before) == 4
    
    # Apply outage to edge 1->3 (edge index 2)
    _, enabled = apply_outage(weights, edge_ids=np.array([2]))
    result_after = sssp(graph, weights, source=0, enabled=enabled, return_predecessors=True)
    assert result_after.dist[4] == 3.0  # Still reachable via 0->2->3->4
    path_after = reconstruct_path(result_after.pred, 4)
    
    # Path should have changed (no longer goes through edge 1->3)
    # Verify path doesn't contain the sequence [1, 3]
    has_1_to_3 = False
    for i in range(len(path_after) - 1):
        if path_after[i] == 1 and path_after[i + 1] == 3:
            has_1_to_3 = True
            break
    assert not has_1_to_3, "Path should not use edge 1->3 after outage"


def test_outage_reachability():
    """Test reachability changes with various outage scenarios."""
    n = 4
    edges = np.array([
        [0, 1], [0, 2],
        [1, 3], [2, 3],
    ], dtype=np.int64)
    weights = np.array([1.0, 1.0, 1.0, 1.0], dtype=np.float32)
    
    graph, _ = Graph.from_edges(n, edges, weights=weights)
    
    # Single edge outage - vertex still reachable
    _, enabled1 = apply_outage(weights, edge_ids=np.array([0]))
    result1 = sssp(graph, weights, source=0, enabled=enabled1)
    assert result1.dist[3] < np.inf  # Still reachable via 0->2->3
    
    # All edges from source disabled
    _, enabled2 = apply_outage(weights, edge_ids=np.array([0, 1]))
    result2 = sssp(graph, weights, source=0, enabled=enabled2)
    assert np.isinf(result2.dist[1])
    assert np.isinf(result2.dist[2])
    assert np.isinf(result2.dist[3])
    
    # All edges to vertex 3 disabled
    _, enabled3 = apply_outage(weights, edge_ids=np.array([2, 3]))
    result3 = sssp(graph, weights, source=0, enabled=enabled3)
    assert np.isinf(result3.dist[3])


def test_outage_disconnected_components():
    """Test outages that create disconnected components."""
    n = 6
    edges = np.array([
        [0, 1], [1, 2],
        [3, 4], [4, 5],
        [2, 3],  # Bridge edge
    ], dtype=np.int64)
    weights = np.array([1.0, 1.0, 1.0, 1.0, 1.0], dtype=np.float32)
    
    graph, weights_sorted = Graph.from_edges(n, edges, weights=weights)
    
    # Before outage: all vertices reachable from 0
    result_before = sssp(graph, weights_sorted, source=0)
    assert result_before.dist[5] < np.inf
    
    # Find bridge edge index in sorted order (2->3)
    # After sorting: [0->1: 0, 1->2: 1, 2->3: 2 (bridge), 3->4: 3, 4->5: 4]
    bridge_edge_idx = None
    for u in range(n):
        for idx in range(graph.indptr[u], graph.indptr[u + 1]):
            if u == 2 and graph.indices[idx] == 3:
                bridge_edge_idx = idx
                break
        if bridge_edge_idx is not None:
            break
    
    assert bridge_edge_idx is not None, "Bridge edge 2->3 not found"
    
    # Disable bridge edge
    _, enabled = apply_outage(weights_sorted, edge_ids=np.array([bridge_edge_idx]))
    result_after = sssp(graph, weights_sorted, source=0, enabled=enabled)
    
    # Vertices 3, 4, 5 should be unreachable
    assert np.isinf(result_after.dist[3])
    assert np.isinf(result_after.dist[4])
    assert np.isinf(result_after.dist[5])
    # Vertices 1, 2 should still be reachable
    assert result_after.dist[1] < np.inf
    assert result_after.dist[2] < np.inf


def test_outage_all_edges_to_vertex():
    """Test disabling all incoming edges to a vertex."""
    n = 4
    edges = np.array([
        [0, 2], [1, 2],
        [2, 3],
    ], dtype=np.int64)
    weights = np.array([1.0, 1.0, 1.0], dtype=np.float32)
    
    graph, _ = Graph.from_edges(n, edges, weights=weights)
    
    # Disable all edges to vertex 2 (edges 0 and 1)
    _, enabled = apply_outage(weights, edge_ids=np.array([0, 1]))
    result = sssp(graph, weights, source=0, enabled=enabled)
    
    # Vertex 2 should be unreachable (no incoming edges)
    assert np.isinf(result.dist[2])
    # Vertex 3 should also be unreachable (depends on 2)
    assert np.isinf(result.dist[3])


def test_path_reconstruction_after_outage():
    """Test path reconstruction after outage changes route."""
    n = 5
    edges = np.array([
        [0, 1], [0, 2],
        [1, 3], [2, 3],
        [3, 4],
    ], dtype=np.int64)
    weights = np.array([1.0, 1.0, 10.0, 1.0, 1.0], dtype=np.float32)
    
    graph, _ = Graph.from_edges(n, edges, weights=weights)
    
    # Before outage: should prefer 0->2->3->4 (cost 3.0) over 0->1->3->4 (cost 12.0)
    result_before = sssp(graph, weights, source=0, return_predecessors=True)
    path_before = reconstruct_path(result_before.pred, 4)
    assert path_before[1] == 2  # Goes through vertex 2
    
    # Apply outage to edge 0->2 (edge index 1)
    _, enabled = apply_outage(weights, edge_ids=np.array([1]))
    result_after = sssp(graph, weights, source=0, enabled=enabled, return_predecessors=True)
    
    if result_after.dist[4] < np.inf:
        path_after = reconstruct_path(result_after.pred, 4)
        assert path_after[1] == 1  # Now goes through vertex 1
        assert path_after != path_before


def test_path_cost_matches_distance():
    """Test that path costs match computed distances."""
    n = 6
    edges = np.array([
        [0, 1], [1, 2], [2, 3],
        [0, 4], [4, 5], [5, 3],
        [0, 3],  # Direct edge
    ], dtype=np.int64)
    weights = np.array([1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 5.0], dtype=np.float32)
    
    graph, _ = Graph.from_edges(n, edges, weights=weights)
    
    result = sssp(graph, weights, source=0, return_predecessors=True)
    
    # Check path cost for vertex 3
    path = reconstruct_path(result.pred, 3)
    assert len(path) > 0
    
    # Compute path cost manually
    path_cost = 0.0
    for i in range(len(path) - 1):
        u, v = path[i], path[i + 1]
        # Find edge weight
        edge_found = False
        for e_idx in range(graph.indptr[u], graph.indptr[u + 1]):
            if graph.indices[e_idx] == v:
                path_cost += weights[e_idx]
                edge_found = True
                break
        assert edge_found, f"Edge ({u}, {v}) not found in graph"
    
    # Path cost should match distance (within floating point tolerance)
    assert abs(path_cost - result.dist[3]) < 1e-5


def test_multiple_sinks_path_reconstruction():
    """Test path reconstruction for multiple sinks from same source."""
    n = 5
    edges = np.array([
        [0, 1], [1, 2],
        [0, 3], [3, 4],
    ], dtype=np.int64)
    weights = np.array([1.0, 1.0, 1.0, 1.0], dtype=np.float32)
    
    graph, _ = Graph.from_edges(n, edges, weights=weights)
    
    result = sssp(graph, weights, source=0, return_predecessors=True)
    
    # Reconstruct paths to multiple sinks
    sink2_path = reconstruct_path(result.pred, 2)
    sink4_path = reconstruct_path(result.pred, 4)
    
    assert len(sink2_path) == 3  # 0->1->2
    assert sink2_path[0] == 0
    assert sink2_path[-1] == 2
    
    assert len(sink4_path) == 3  # 0->3->4
    assert sink4_path[0] == 0
    assert sink4_path[-1] == 4
    
    # Paths should be different
    assert sink2_path != sink4_path


def test_scenario_loop_end_to_end():
    """Test full scenario loop: build → compute → update weights → recompute → apply outage → recompute."""
    n = 4
    edges = np.array([
        [0, 1], [1, 2], [2, 3],
        [0, 2], [1, 3],
    ], dtype=np.int64)
    weights = np.array([1.0, 1.0, 1.0, 3.0, 3.0], dtype=np.float32)
    
    graph, _ = Graph.from_edges(n, edges, weights=weights)
    
    # Step 1: Initial SSSP
    result1 = sssp(graph, weights, source=0, return_predecessors=True)
    assert result1.dist[3] < np.inf
    
    # Step 2: Update weights (simulate congestion)
    weights2 = weights.copy()
    weights2[0] = 5.0  # Make 0->1 expensive
    result2 = sssp(graph, weights2, source=0, return_predecessors=True)
    
    # Step 3: Apply outage
    _, enabled = apply_outage(weights2, edge_ids=np.array([1]))  # Disable 1->2
    result3 = sssp(graph, weights2, source=0, enabled=enabled, return_predecessors=True)
    
    # All steps should complete without error
    assert result1.dist[3] < np.inf
    assert result2.dist[3] < np.inf
    # Result 3 might be unreachable depending on outage
    # Just verify it computed without error


def test_multiple_scenarios_sequence():
    """Test multiple scenarios in sequence."""
    n = 4
    edges = np.array([
        [0, 1], [1, 2], [2, 3],
        [0, 3],
    ], dtype=np.int64)
    base_weights = np.array([1.0, 1.0, 1.0, 5.0], dtype=np.float32)
    
    graph, _ = Graph.from_edges(n, edges, weights=base_weights)
    
    scenarios = []
    for scenario_id in range(3):
        # Vary weights
        weights = base_weights.copy()
        weights[scenario_id] = 10.0
        
        # Compute SSSP
        result = sssp(graph, weights, source=0)
        scenarios.append({
            'scenario': scenario_id,
            'cost': result.dist[3],
            'reachable': result.dist[3] < np.inf
        })
    
    # All scenarios should be reachable
    assert all(s['reachable'] for s in scenarios)
    # Costs should vary
    assert len(set(s['cost'] for s in scenarios)) > 1


def test_dynamic_weights_with_outages():
    """Test weight updates combined with enabled masks."""
    n = 4
    edges = np.array([
        [0, 1], [1, 2], [2, 3],
        [0, 2], [1, 3],
    ], dtype=np.int64)
    weights = np.array([1.0, 1.0, 1.0, 3.0, 3.0], dtype=np.float32)
    
    graph, _ = Graph.from_edges(n, edges, weights=weights)
    
    # Initial computation
    result1 = sssp(graph, weights, source=0)
    cost1 = result1.dist[3]
    
    # Update weights AND apply outage
    weights2 = weights.copy()
    weights2[0] = 5.0  # Increase weight
    _, enabled = apply_outage(weights2, edge_ids=np.array([1]))  # Disable edge
    
    result2 = sssp(graph, weights2, source=0, enabled=enabled)
    cost2 = result2.dist[3]
    
    # Should compute without error
    # Cost might be different or infinity
    assert cost2 >= cost1 or np.isinf(cost2)
