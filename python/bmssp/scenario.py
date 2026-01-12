"""Scenario utilities for grid and pipeline network optimization."""

import numpy as np
from dataclasses import dataclass
from typing import Optional


@dataclass
class EdgeAttributes:
    """Attributes for graph edges in grid/pipeline networks."""
    
    base_cost: float
    """Base cost for using this edge."""
    
    capacity: float
    """Capacity of the edge."""
    
    risk: float
    """Risk factor (multiplies cost)."""
    
    region: Optional[str] = None
    """Optional region identifier."""
    
    line_type: Optional[str] = None
    """Optional line type (e.g., 'transmission', 'distribution')."""
    
    is_switchable: bool = False
    """Whether this edge can be switched on/off."""


def weight_model(
    flow: np.ndarray,
    attrs: EdgeAttributes,
    alpha: float = 1.0,
) -> np.ndarray:
    """Compute effective weights from flow and edge attributes.
    
    Default model: w = base_cost * risk * (1 + alpha * (flow/capacity)^2)
    
    Args:
        flow: Current flow per edge (length = number of edges)
        attrs: Edge attributes (single EdgeAttributes for all edges, or array)
        alpha: Congestion factor (default: 1.0)
    
    Returns:
        Weight array (length = number of edges)
    """
    flow = np.asarray(flow)
    
    # Handle single attributes or array of attributes
    if isinstance(attrs, EdgeAttributes):
        base_cost = attrs.base_cost
        capacity = attrs.capacity
        risk = attrs.risk
        
        # Compute congestion term
        congestion = 1.0 + alpha * np.square(np.divide(flow, capacity, out=np.zeros_like(flow), where=capacity > 0))
        
        # Compute weights
        weights = base_cost * risk * congestion
    else:
        # Array of attributes - compute per edge
        weights = np.zeros(len(flow))
        for i, attr in enumerate(attrs):
            if i < len(flow):
                congestion = 1.0 + alpha * (flow[i] / attr.capacity) ** 2 if attr.capacity > 0 else 1.0
                weights[i] = attr.base_cost * attr.risk * congestion
    
    return weights


def apply_outage(
    weights: np.ndarray,
    edge_mask: Optional[np.ndarray] = None,
    edge_ids: Optional[np.ndarray] = None,
    penalty: float = np.inf,
) -> tuple[np.ndarray, Optional[np.ndarray]]:
    """Apply outage to edges.
    
    Args:
        weights: Current edge weights
        edge_mask: Boolean mask of edges to disable (length = number of edges)
        edge_ids: Alternative: indices of edges to disable
        penalty: Penalty weight for disabled edges (default: inf)
    
    Returns:
        Tuple of (updated_weights, enabled_mask)
        - If using edge_mask: returns (weights with penalty, enabled mask)
        - If using edge_ids: returns (weights with penalty, enabled mask)
        - enabled_mask is None if no outages applied
    """
    weights = np.asarray(weights).copy()
    n = len(weights)
    
    if edge_mask is not None:
        edge_mask = np.asarray(edge_mask, dtype=bool)
        if len(edge_mask) != n:
            raise ValueError(f"edge_mask length {len(edge_mask)} != weights length {n}")
        weights[edge_mask] = penalty
        enabled = ~edge_mask
        return weights, enabled
    elif edge_ids is not None:
        edge_ids = np.asarray(edge_ids, dtype=np.int64)
        enabled = np.ones(n, dtype=bool)
        enabled[edge_ids] = False
        weights[edge_ids] = penalty
        return weights, enabled
    else:
        return weights, None
