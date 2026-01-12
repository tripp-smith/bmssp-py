"""Single-source shortest path algorithms."""

import numpy as np
from dataclasses import dataclass
from typing import Optional, List, TYPE_CHECKING

if TYPE_CHECKING:
    from .graph import Graph

try:
    import _bmssp
except ImportError:
    _bmssp = None  # Will be available after building with maturin


@dataclass
class SSSPResult:
    """Result of single-source shortest path computation."""
    
    dist: np.ndarray
    """Distances from source to each vertex (infinity if unreachable)."""
    
    pred: Optional[np.ndarray] = None
    """Predecessor vertex array (None if not requested)."""
    
    pred_edge: Optional[np.ndarray] = None
    """Predecessor edge array (None if not requested)."""


def sssp(
    graph: "Graph",
    weights: np.ndarray,
    source: int,
    enabled: Optional[np.ndarray] = None,
    return_predecessors: bool = False,
) -> SSSPResult:
    """Compute single-source shortest paths.
    
    Args:
        graph: Graph object
        weights: Edge weights array (length = number of edges)
        source: Source vertex index
        enabled: Optional boolean mask for enabled edges (None = all enabled)
        return_predecessors: Whether to return predecessor arrays
    
    Returns:
        SSSPResult with distances and optionally predecessors
    """
    if _bmssp is None:
        raise RuntimeError("_bmssp module not available. Build with 'maturin develop'")
    
    # Validate inputs
    weights = np.asarray(weights, dtype=np.float32)
    if len(weights) != graph.num_edges():
        raise ValueError(
            f"Weights length {len(weights)} != graph edges {graph.num_edges()}"
        )
    
    if source < 0 or source >= graph.num_vertices():
        raise ValueError(
            f"Source {source} out of range [0, {graph.num_vertices()})"
        )
    
    if enabled is not None:
        enabled = np.asarray(enabled, dtype=np.uint8)
        if len(enabled) != graph.num_edges():
            raise ValueError(
                f"Enabled mask length {len(enabled)} != graph edges {graph.num_edges()}"
            )
    
    # Dispatch based on dtype
    weights_dtype = weights.dtype
    if weights_dtype == np.float32:
        result = _bmssp.sssp_f32_csr(
            graph.indptr,
            graph.indices,
            weights,
            source,
            enabled,
            return_predecessors,
        )
    elif weights_dtype == np.float64:
        result = _bmssp.sssp_f64_csr(
            graph.indptr,
            graph.indices,
            weights,
            source,
            enabled,
            return_predecessors,
        )
    else:
        # Convert to float32
        weights = weights.astype(np.float32)
        result = _bmssp.sssp_f32_csr(
            graph.indptr,
            graph.indices,
            weights,
            source,
            enabled,
            return_predecessors,
        )
    
    # Parse result
    if return_predecessors:
        dist = result["dist"]
        pred = result["pred"]
        # Convert -1 to None or keep as is for path reconstruction
        return SSSPResult(dist=dist, pred=pred, pred_edge=None)
    else:
        return SSSPResult(dist=result, pred=None, pred_edge=None)


def reconstruct_path(pred: np.ndarray, target: int) -> List[int]:
    """Reconstruct path from source to target using predecessor array.
    
    Args:
        pred: Predecessor array (from SSSPResult.pred), where -1 indicates unreachable
        target: Target vertex index
    
    Returns:
        List of vertex indices from source to target (inclusive), empty if unreachable
    """
    if pred[target] < 0:
        return []  # Unreachable
    
    path = []
    current = target
    visited = set()  # Prevent cycles
    
    while current >= 0 and current not in visited:
        visited.add(current)
        path.append(current)
        if current >= len(pred):
            break
        next_pred = pred[current]
        if next_pred < 0 or next_pred == current:  # Source or unreachable
            break
        current = next_pred
    
    path.reverse()
    return path


def multi_sink_costs(dist: np.ndarray, sinks: np.ndarray) -> np.ndarray:
    """Extract distances to multiple sink vertices.
    
    Args:
        dist: Distance array from sssp()
        sinks: Array of sink vertex indices
    
    Returns:
        Array of distances to each sink
    """
    sinks = np.asarray(sinks)
    return dist[sinks]
