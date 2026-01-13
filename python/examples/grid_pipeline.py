"""Example: Grid and pipeline network optimization.

This example demonstrates:
1. Building a grid-like distribution network
2. Computing initial shortest paths
3. Applying load flows and updating weights
4. Handling outages
5. Recomputing paths and analyzing results
"""

import numpy as np
import time
from typing import Optional
from bmssp import Graph, sssp, reconstruct_path
from bmssp.scenario import EdgeAttributes, weight_model, apply_outage


def _find_edge_index(graph: Graph, u: int, v: int) -> Optional[int]:
    """Find the CSR edge index for edge (u, v).
    
    Args:
        graph: Graph object
        u: Source vertex
        v: Destination vertex
    
    Returns:
        Edge index if found, None otherwise
    """
    start = graph.indptr[u]
    end = graph.indptr[u + 1]
    edge_range = graph.indices[start:end]
    
    # Find position of v in the edge range
    matches = np.where(edge_range == v)[0]
    if len(matches) > 0:
        return start + matches[0]  # Return first match
    return None


def build_grid_network(rows: int, cols: int, seed: Optional[int] = None) -> tuple[Graph, np.ndarray, list[EdgeAttributes]]:
    """Build a grid-like distribution network.
    
    Args:
        rows: Number of rows
        cols: Number of columns
        seed: Optional random seed for reproducibility
    
    Returns:
        Tuple of (graph, initial_weights, edge_attributes)
        Note: edge_attributes are in CSR edge order (after sorting and deduplication)
    """
    if seed is not None:
        np.random.seed(seed)
    
    n = rows * cols
    edges = []
    edge_attrs_map = {}  # Map (u, v) -> EdgeAttributes (for deduplication)
    
    # Horizontal edges
    for i in range(rows):
        for j in range(cols - 1):
            u = i * cols + j
            v = u + 1
            edges.append([u, v])
            # For deduplication, we'll use the first attribute (matches "min" behavior)
            if (u, v) not in edge_attrs_map:
                edge_attrs_map[(u, v)] = EdgeAttributes(
                    base_cost=1.0 + np.random.random() * 0.5,
                    capacity=10.0 + np.random.random() * 5.0,
                    risk=1.0 + np.random.random() * 0.3,
                )
    
    # Vertical edges
    for i in range(rows - 1):
        for j in range(cols):
            u = i * cols + j
            v = u + cols
            edges.append([u, v])
            if (u, v) not in edge_attrs_map:
                edge_attrs_map[(u, v)] = EdgeAttributes(
                    base_cost=1.0 + np.random.random() * 0.5,
                    capacity=10.0 + np.random.random() * 5.0,
                    risk=1.0 + np.random.random() * 0.3,
                )
    
    # Tie lines (some diagonal connections)
    num_tie_lines = rows + cols
    for _ in range(num_tie_lines):
        u = np.random.randint(0, n)
        v = np.random.randint(0, n)
        if u != v:
            edges.append([u, v])
            if (u, v) not in edge_attrs_map:
                edge_attrs_map[(u, v)] = EdgeAttributes(
                    base_cost=2.0 + np.random.random(),
                    capacity=5.0 + np.random.random() * 3.0,
                    risk=1.5 + np.random.random() * 0.5,
                )
    
    edges_array = np.array(edges, dtype=np.int64)
    
    # Create initial weights from attributes (before sorting)
    # Use weights for deduplication - same (u, v) edges will use minimum weight
    initial_weights_array = np.array([
        edge_attrs_map[(u, v)].base_cost * edge_attrs_map[(u, v)].risk 
        for u, v in edges_array
    ], dtype=np.float32)
    
    # Build graph - this sorts edges and deduplicates
    graph, sorted_weights = Graph.from_edges(n, edges_array, weights=initial_weights_array, sort=True, dedupe="min")
    
    # Recreate attrs_list in CSR order
    # Reconstruct sorted edges from CSR and look up attributes
    attrs_list_sorted = []
    for u in range(n):
        for idx in range(graph.indptr[u], graph.indptr[u + 1]):
            v = graph.indices[idx]
            # Look up attribute for this edge
            if (u, v) in edge_attrs_map:
                attrs_list_sorted.append(edge_attrs_map[(u, v)])
            else:
                # Should not happen, but provide default
                attrs_list_sorted.append(EdgeAttributes(1.0, 10.0, 1.0))
    
    return graph, sorted_weights, attrs_list_sorted


def compute_flow(graph: Graph, result, sinks: np.ndarray, source: int, flow_per_sink: float = 1.0) -> np.ndarray:
    """Compute flow on each edge by tracing paths from source to sinks.
    
    This is a simplified model - in practice, you'd use a full power flow solver.
    
    Args:
        graph: Graph object
        result: SSSPResult with predecessors
        sinks: Array of sink vertex indices
        source: Source vertex index
        flow_per_sink: Flow amount per sink (default: 1.0)
    
    Returns:
        Flow array (length = number of edges) aligned with CSR edge order
    """
    if result.pred is None:
        raise ValueError("Predecessors are required for flow computation")
    
    num_edges = graph.num_edges()
    flow = np.zeros(num_edges, dtype=np.float32)
    
    # For each sink, trace back path and add flow
    for sink in sinks:
        if result.dist[sink] >= np.inf:
            continue  # Skip unreachable sinks
        
        path = reconstruct_path(result.pred, sink)
        if len(path) < 2:
            continue  # Need at least source and sink
        
        # Trace path and accumulate flow on edges
        for i in range(len(path) - 1):
            u = path[i]
            v = path[i + 1]
            
            # Find CSR edge index for (u, v)
            edge_idx = _find_edge_index(graph, u, v)
            if edge_idx is not None:
                flow[edge_idx] += flow_per_sink
    
    return flow


def format_path(path: list[int], max_show: int = 10) -> str:
    """Format a path for display.
    
    Args:
        path: List of vertex indices
        max_show: Maximum number of vertices to show (default: 10)
    
    Returns:
        Formatted path string
    """
    if len(path) == 0:
        return "[]"
    if len(path) <= max_show:
        return " -> ".join(str(v) for v in path)
    else:
        start = " -> ".join(str(v) for v in path[:max_show//2])
        end = " -> ".join(str(v) for v in path[-max_show//2:])
        return f"{start} -> ... -> {end} ({len(path)} vertices)"


def main():
    """Run the grid/pipeline optimization example."""
    print("Grid and Pipeline Network Optimization Example")
    print("=" * 60)
    
    # Build network
    print("\n1. Building grid network (4x4)...")
    try:
        graph, initial_weights, attrs_list = build_grid_network(4, 4, seed=42)
        print(f"   Graph: {graph.num_vertices()} vertices, {graph.num_edges()} edges")
    except Exception as e:
        print(f"   Error building network: {e}")
        return
    
    # Define source and sinks
    source = 0
    sinks = np.array([graph.num_vertices() - 1])
    
    # Validate inputs
    if source < 0 or source >= graph.num_vertices():
        print(f"   Error: Invalid source vertex {source}")
        return
    
    if np.any(sinks < 0) or np.any(sinks >= graph.num_vertices()):
        print(f"   Error: Invalid sink vertices")
        return
    
    # Initial SSSP
    print("\n2. Computing initial shortest paths...")
    try:
        start_time = time.time()
        result1 = sssp(graph, initial_weights, source=source, return_predecessors=True)
        elapsed1 = time.time() - start_time
        print(f"   Time: {elapsed1*1000:.3f} ms")
        
        sink_cost = result1.dist[sinks[0]]
        if sink_cost >= np.inf:
            print(f"   Warning: Sink {sinks[0]} is unreachable from source {source}")
            return
        
        print(f"   Cost to sink {sinks[0]}: {sink_cost:.3f}")
        
        if result1.pred is not None:
            path1 = reconstruct_path(result1.pred, sinks[0])
            print(f"   Path: {format_path(path1)}")
            print(f"   Path length: {len(path1)} vertices")
    except Exception as e:
        print(f"   Error computing initial paths: {e}")
        return
    
    # Apply load flows
    print("\n3. Applying load flows...")
    try:
        flow = compute_flow(graph, result1, sinks, source, flow_per_sink=1.0)
        total_flow = np.sum(flow)
        max_flow = np.max(flow)
        print(f"   Total flow: {total_flow:.2f}")
        print(f"   Max edge flow: {max_flow:.2f}")
    except Exception as e:
        print(f"   Error computing flows: {e}")
        return
    
    # Update weights from congestion model
    print("\n4. Updating weights from congestion model...")
    try:
        # Use per-edge attributes (attrs_list is already in CSR order)
        updated_weights = weight_model(flow, attrs_list, alpha=1.0)
        print(f"   Updated {len(updated_weights)} edge weights")
    except Exception as e:
        print(f"   Error updating weights: {e}")
        return
    
    # Recompute SSSP
    print("\n5. Recomputing SSSP with updated weights...")
    try:
        start_time = time.time()
        result2 = sssp(graph, updated_weights, source=source, return_predecessors=True)
        elapsed2 = time.time() - start_time
        print(f"   Time: {elapsed2*1000:.3f} ms")
        
        sink_cost2 = result2.dist[sinks[0]]
        print(f"   Cost to sink {sinks[0]}: {sink_cost2:.3f}")
        
        if result2.pred is not None:
            path2 = reconstruct_path(result2.pred, sinks[0])
            print(f"   Path: {format_path(path2)}")
            print(f"   Path length: {len(path2)} vertices")
            
            if path1 != path2:
                print("   ✓ Path changed due to congestion!")
                print(f"     Old path cost: {sink_cost:.3f}")
                print(f"     New path cost: {sink_cost2:.3f}")
            else:
                print("   Path unchanged")
    except Exception as e:
        print(f"   Error recomputing paths: {e}")
        return
    
    # Apply outage
    print("\n6. Applying outage (disabling 10% of edges)...")
    try:
        num_outages = max(1, graph.num_edges() // 10)
        outage_edge_ids = np.random.choice(graph.num_edges(), size=num_outages, replace=False)
        weights_after_outage, enabled = apply_outage(updated_weights, edge_ids=outage_edge_ids)
        print(f"   Disabled {num_outages} edges (out of {graph.num_edges()} total)")
    except Exception as e:
        print(f"   Error applying outage: {e}")
        return
    
    # Recompute SSSP after outage
    print("\n7. Recomputing SSSP after outage...")
    try:
        start_time = time.time()
        result3 = sssp(graph, weights_after_outage, source=source, enabled=enabled, return_predecessors=True)
        elapsed3 = time.time() - start_time
        print(f"   Time: {elapsed3*1000:.3f} ms")
        
        sink_cost3 = result3.dist[sinks[0]]
        if sink_cost3 >= np.inf:
            print(f"   Sink {sinks[0]} is unreachable after outage")
        else:
            print(f"   Cost to sink {sinks[0]}: {sink_cost3:.3f}")
            
            if result3.pred is not None:
                path3 = reconstruct_path(result3.pred, sinks[0])
                print(f"   Path: {format_path(path3)}")
                print(f"   Path length: {len(path3)} vertices")
                
                if path3 != path2:
                    print("   ✓ Path changed due to outage!")
                else:
                    print("   Path unchanged (outage did not affect route)")
    except Exception as e:
        print(f"   Error recomputing paths after outage: {e}")
        return
    
    # Summary
    print("\n" + "=" * 60)
    print("Summary:")
    print("-" * 60)
    print(f"  Scenario              Cost        Time (ms)    Path Length")
    print("-" * 60)
    print(f"  Initial              {sink_cost:8.3f}     {elapsed1*1000:8.3f}      {len(path1):3d}")
    print(f"  After congestion     {sink_cost2:8.3f}     {elapsed2*1000:8.3f}      {len(path2):3d}")
    if sink_cost3 < np.inf:
        print(f"  After outage         {sink_cost3:8.3f}     {elapsed3*1000:8.3f}      {len(path3):3d}")
    else:
        print(f"  After outage         unreachable  {elapsed3*1000:8.3f}      -")
    print("-" * 60)
    
    # Timing analysis
    print("\nPerformance Notes:")
    print(f"  • Graph topology is immutable - no rebuilding needed")
    print(f"  • Recomputations are fast: {elapsed2/elapsed1:.2f}x and {elapsed3/elapsed1:.2f}x of initial time")
    print(f"  • Enabled mask allows fast outage simulation without graph changes")


if __name__ == "__main__":
    main()
