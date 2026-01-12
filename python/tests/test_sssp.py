"""Tests for SSSP functions."""

import numpy as np
import pytest
from bmssp import Graph, sssp, reconstruct_path, multi_sink_costs


@pytest.fixture
def simple_graph():
    """Create a simple 3-vertex chain graph."""
    n = 3
    edges = np.array([[0, 1], [1, 2]], dtype=np.int64)
    graph, weights = Graph.from_edges(n, edges, weights=np.array([1.0, 2.0], dtype=np.float32))
    return graph, weights


def test_sssp_simple(simple_graph):
    """Test SSSP on a simple chain graph."""
    graph, weights = simple_graph
    result = sssp(graph, weights, source=0)
    
    assert len(result.dist) == 3
    assert result.dist[0] == 0.0
    assert result.dist[1] == 1.0
    assert result.dist[2] == 3.0
    assert result.pred is None


def test_sssp_disconnected():
    """Test SSSP on disconnected graph."""
    n = 3
    edges = np.array([[0, 1]], dtype=np.int64)
    graph, weights = Graph.from_edges(n, edges, weights=np.array([1.0], dtype=np.float32))
    result = sssp(graph, weights, source=0)
    
    assert result.dist[0] == 0.0
    assert result.dist[1] == 1.0
    assert np.isinf(result.dist[2])


def test_sssp_invalid_source(simple_graph):
    """Test SSSP with invalid source vertex."""
    graph, weights = simple_graph
    with pytest.raises(ValueError):
        sssp(graph, weights, source=10)


def test_sssp_invalid_weights(simple_graph):
    """Test SSSP with invalid weights length."""
    graph, _ = simple_graph
    weights = np.array([1.0, 2.0, 3.0], dtype=np.float32)  # Wrong length
    with pytest.raises(ValueError):
        sssp(graph, weights, source=0)


def test_multi_sink_costs(simple_graph):
    """Test multi_sink_costs helper."""
    graph, weights = simple_graph
    result = sssp(graph, weights, source=0)
    sinks = np.array([1, 2])
    costs = multi_sink_costs(result.dist, sinks)
    
    assert len(costs) == 2
    assert costs[0] == 1.0
    assert costs[1] == 3.0
