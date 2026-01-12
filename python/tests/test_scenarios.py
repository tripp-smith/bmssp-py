"""Tests for scenario utilities."""

import numpy as np
import pytest
from bmssp.scenario import EdgeAttributes, weight_model, apply_outage
from bmssp import Graph, sssp


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
    updated_weights, enabled = apply_outage(weights, edge_mask=edge_mask)
    
    # After outage: must use path 0->2->3
    result_after = sssp(graph, updated_weights, source=0, enabled=enabled)
    assert result_after.dist[3] == 2.0  # Still reachable via other path
    
    # Apply outage to both paths to 3
    edge_mask = np.array([True, False, True, False], dtype=bool)
    updated_weights, enabled = apply_outage(weights, edge_mask=edge_mask)
    result_final = sssp(graph, updated_weights, source=0, enabled=enabled)
    assert np.isinf(result_final.dist[3])  # No longer reachable
