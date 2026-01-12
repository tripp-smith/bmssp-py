"""Tests for Graph class."""

import numpy as np
import pytest
from bmssp import Graph


def test_graph_from_csr():
    """Test creating graph from CSR arrays."""
    indptr = np.array([0, 1, 2], dtype=np.int64)
    indices = np.array([1, 0], dtype=np.int64)
    graph = Graph.from_csr(indptr, indices)
    assert graph.num_vertices() == 2
    assert graph.num_edges() == 2


def test_graph_from_edges():
    """Test creating graph from edge list."""
    n = 3
    edges = np.array([[0, 1], [1, 2], [0, 2]], dtype=np.int64)
    graph, weights = Graph.from_edges(n, edges)
    assert graph.num_vertices() == 3
    assert graph.num_edges() == 3
    assert len(weights) == 3


def test_graph_from_edges_with_weights():
    """Test creating graph from edge list with weights."""
    n = 3
    edges = np.array([[0, 1], [1, 2], [0, 2]], dtype=np.int64)
    weights = np.array([1.0, 2.0, 1.5], dtype=np.float32)
    graph, result_weights = Graph.from_edges(n, edges, weights=weights)
    assert graph.num_vertices() == 3
    assert graph.num_edges() == 3
    np.testing.assert_array_equal(result_weights, weights)


def test_graph_validation():
    """Test graph validation."""
    # Invalid indptr length
    with pytest.raises(ValueError):
        Graph.from_csr(np.array([0, 1]), np.array([1]), n=2)
    
    # Invalid indices
    with pytest.raises(ValueError):
        Graph.from_csr(np.array([0, 1, 1]), np.array([5]), n=2)


def test_graph_dedupe_min():
    """Test duplicate edge handling with min weight."""
    n = 2
    edges = np.array([[0, 1], [0, 1], [0, 1]], dtype=np.int64)
    weights = np.array([3.0, 1.0, 2.0], dtype=np.float32)
    graph, result_weights = Graph.from_edges(n, edges, weights=weights, dedupe="min")
    assert graph.num_edges() == 1
    assert result_weights[0] == 1.0  # Minimum weight kept
