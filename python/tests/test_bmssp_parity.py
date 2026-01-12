"""Tests comparing BMSSP against Dijkstra baseline."""

import numpy as np
import pytest
from bmssp import Graph, sssp


def test_parity_simple():
    """Test that BMSSP (currently Dijkstra) matches itself."""
    n = 5
    edges = np.array([
        [0, 1], [0, 2],
        [1, 3],
        [2, 3], [2, 4],
        [3, 4],
    ], dtype=np.int64)
    weights = np.array([1.0, 2.0, 1.0, 1.0, 3.0, 1.0], dtype=np.float32)
    
    graph, _ = Graph.from_edges(n, edges, weights=weights)
    result = sssp(graph, weights, source=0)
    
    # Distances should be correct
    assert result.dist[0] == 0.0
    assert result.dist[1] == 1.0
    assert result.dist[3] == 2.0
    assert result.dist[4] == 3.0


def test_parity_with_predecessors():
    """Test predecessor reconstruction."""
    n = 4
    edges = np.array([
        [0, 1], [0, 2],
        [1, 3],
        [2, 3],
    ], dtype=np.int64)
    weights = np.array([1.0, 2.0, 1.0, 1.0], dtype=np.float32)
    
    graph, _ = Graph.from_edges(n, edges, weights=weights)
    result = sssp(graph, weights, source=0, return_predecessors=True)
    
    assert result.pred is not None
    from bmssp import reconstruct_path
    
    path = reconstruct_path(result.pred, 3)
    assert len(path) > 0
    assert path[0] == 0  # Starts at source
    assert path[-1] == 3  # Ends at target


def test_parity_grid():
    """Test on a grid graph."""
    # 2x2 grid
    n = 4
    edges = np.array([
        [0, 1], [0, 2],
        [1, 3],
        [2, 3],
    ], dtype=np.int64)
    weights = np.array([1.0, 1.0, 1.0, 1.0], dtype=np.float32)
    
    graph, _ = Graph.from_edges(n, edges, weights=weights)
    result = sssp(graph, weights, source=0)
    
    assert result.dist[0] == 0.0
    assert result.dist[1] == 1.0
    assert result.dist[2] == 1.0
    assert result.dist[3] == 2.0
