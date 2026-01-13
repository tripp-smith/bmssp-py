"""Tests comparing BMSSP (via Dijkstra baseline) against SciPy."""

import numpy as np
import pytest

try:
    from scipy.sparse import csr_matrix
    from scipy.sparse.csgraph import dijkstra
    SCIPY_AVAILABLE = True
except ImportError:
    SCIPY_AVAILABLE = False

from bmssp import Graph, sssp


@pytest.mark.skipif(not SCIPY_AVAILABLE, reason="SciPy not available")
def test_parity_small_graph():
    """Compare against SciPy on a small graph."""
    n = 4
    # Create a small directed graph
    edges = np.array([
        [0, 1], [0, 2],
        [1, 3],
        [2, 3],
    ], dtype=np.int64)
    weights = np.array([1.0, 2.0, 1.0, 1.0], dtype=np.float32)
    
    graph, _ = Graph.from_edges(n, edges, weights=weights)
    result = sssp(graph, weights, source=0)
    
    # Compare with SciPy
    # Build CSR matrix for SciPy
    row = edges[:, 0]
    col = edges[:, 1]
    data = weights
    csr = csr_matrix((data, (row, col)), shape=(n, n))
    dist_scipy = dijkstra(csgraph=csr, directed=True, indices=0, return_predecessors=False)
    
    # Compare distances (within floating point tolerance)
    np.testing.assert_allclose(result.dist, dist_scipy, rtol=1e-6, atol=1e-6)


@pytest.mark.skipif(not SCIPY_AVAILABLE, reason="SciPy not available")
def test_parity_grid_graph():
    """Compare against SciPy on a small grid graph."""
    n = 4  # 2x2 grid
    # 0 -> 1
    # |    |
    # v    v
    # 2 -> 3
    edges = np.array([
        [0, 1], [0, 2],
        [1, 3],
        [2, 3],
    ], dtype=np.int64)
    weights = np.array([1.0, 1.0, 1.0, 1.0], dtype=np.float32)
    
    graph, _ = Graph.from_edges(n, edges, weights=weights)
    result = sssp(graph, weights, source=0)
    
    # Compare with SciPy
    row = edges[:, 0]
    col = edges[:, 1]
    data = weights
    csr = csr_matrix((data, (row, col)), shape=(n, n))
    dist_scipy = dijkstra(csgraph=csr, directed=True, indices=0, return_predecessors=False)
    
    np.testing.assert_allclose(result.dist, dist_scipy, rtol=1e-6, atol=1e-6)
