"""Graph representation using Compressed Sparse Row (CSR) format."""

import numpy as np
from typing import Optional, Tuple


class Graph:
    """Immutable directed graph backed by CSR format.
    
    The graph topology is immutable, but weights can be updated
    for different scenarios without rebuilding the graph structure.
    """
    
    def __init__(
        self,
        n: int,
        indptr: np.ndarray,
        indices: np.ndarray,
        edge_ids: Optional[np.ndarray] = None,
    ):
        """Create a graph from CSR arrays.
        
        Args:
            n: Number of vertices
            indptr: Index pointer array (length n+1)
            indices: Column indices array (length m, number of edges)
            edge_ids: Optional edge ID mapping (length m)
        """
        self.n = n
        self.indptr = np.asarray(indptr, dtype=np.int64)
        self.indices = np.asarray(indices, dtype=np.int64)
        self.edge_ids = edge_ids if edge_ids is None else np.asarray(edge_ids, dtype=np.int64)
        
        # Validate
        if self.indptr.shape[0] != n + 1:
            raise ValueError(f"indptr length {self.indptr.shape[0]} != n+1 ({n+1})")
        if not np.all(self.indptr[:-1] <= self.indptr[1:]):
            raise ValueError("indptr is not monotonic")
        if self.indptr[-1] != len(self.indices):
            raise ValueError(f"indptr[{n}] = {self.indptr[-1]} != indices.length = {len(self.indices)}")
        if np.any(self.indices < 0) or np.any(self.indices >= n):
            raise ValueError(f"indices out of range [0, {n})")
        if self.edge_ids is not None and len(self.edge_ids) != len(self.indices):
            raise ValueError(f"edge_ids length {len(self.edge_ids)} != indices length {len(self.indices)}")
    
    @classmethod
    def from_csr(
        cls,
        indptr: np.ndarray,
        indices: np.ndarray,
        n: Optional[int] = None,
        edge_ids: Optional[np.ndarray] = None,
    ) -> "Graph":
        """Create a graph from CSR arrays.
        
        Args:
            indptr: Index pointer array
            indices: Column indices array
            n: Number of vertices (default: indptr.length - 1)
            edge_ids: Optional edge ID mapping
        """
        if n is None:
            n = len(indptr) - 1
        return cls(n, indptr, indices, edge_ids)
    
    @classmethod
    def from_edges(
        cls,
        n: int,
        edges: np.ndarray,
        weights: Optional[np.ndarray] = None,
        sort: bool = True,
        dedupe: str = "min",
    ):
        """Create a graph from an edge list.
        
        Args:
            n: Number of vertices
            edges: Edge array of shape (m, 2) with (u, v) pairs
            weights: Optional weights for deduplication (if dedupe="min")
            sort: Whether to sort edges by source vertex (required for CSR)
            dedupe: How to handle duplicates: "min" (keep minimum weight), "first", "last"
        """
        edges = np.asarray(edges)
        if edges.shape[1] != 2:
            raise ValueError(f"edges must have shape (m, 2), got {edges.shape}")
        
        m = edges.shape[0]
        u = edges[:, 0].astype(np.int64)
        v = edges[:, 1].astype(np.int64)
        
        # Validate vertex indices
        if np.any(u < 0) or np.any(u >= n) or np.any(v < 0) or np.any(v >= n):
            raise ValueError(f"Vertex indices must be in [0, {n})")
        
        # Handle duplicates if needed
        if dedupe != "first":
            # Create edge keys for deduplication
            edge_keys = u * n + v  # Unique key for each (u, v) pair
            
            if dedupe == "min":
                if weights is None:
                    weights = np.ones(m, dtype=np.float32)
                else:
                    weights = np.asarray(weights, dtype=np.float32)
            
            # Sort by key, then by weight (ascending)
            sort_idx = np.lexsort((weights, edge_keys))
            u_sorted = u[sort_idx]
            v_sorted = v[sort_idx]
            weights_sorted = weights[sort_idx]
            keys_sorted = edge_keys[sort_idx]
            
            # Keep first occurrence (minimum weight) for each unique key
            _, unique_idx = np.unique(keys_sorted, return_index=True)
            u = u_sorted[unique_idx]
            v = v_sorted[unique_idx]
            weights = weights_sorted[unique_idx]
        elif dedupe == "last":
            # Keep last occurrence - sort in reverse
            reverse_sort_idx = np.lexsort((edge_keys,))[::-1]
            u_rev = u[reverse_sort_idx]
            v_rev = v[reverse_sort_idx]
            keys_rev = edge_keys[reverse_sort_idx]
            _, unique_idx = np.unique(keys_rev, return_index=True)
            u = u_rev[unique_idx]
            v = v_rev[unique_idx]
            if weights is not None:
                weights_rev = weights[reverse_sort_idx]
                weights = weights_rev[unique_idx]
            else:
                weights = np.ones(len(u), dtype=np.float32)
        else:  # dedupe == "first"
            if weights is None:
                weights = np.ones(m, dtype=np.float32)
            else:
                weights = np.asarray(weights, dtype=np.float32)
        
        # Sort by source vertex (required for CSR)
        if sort:
            sort_idx = np.argsort(u)
            u = u[sort_idx]
            v = v[sort_idx]
            weights = weights[sort_idx]
        
        # Build CSR arrays
        indptr = np.zeros(n + 1, dtype=np.int64)
        indices = v.astype(np.int64)
        
        # Count edges per vertex
        if len(u) > 0:
            np.add.at(indptr, u + 1, 1)
        indptr = np.cumsum(indptr)
        
        return cls(n, indptr, indices), weights
    
    def num_vertices(self) -> int:
        """Number of vertices."""
        return self.n
    
    def num_edges(self) -> int:
        """Number of edges."""
        return len(self.indices)
