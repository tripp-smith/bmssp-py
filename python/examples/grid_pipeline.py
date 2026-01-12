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
from bmssp import Graph, sssp, reconstruct_path
from bmssp.scenario import EdgeAttributes, weight_model, apply_outage


def build_grid_network(rows: int, cols: int) -> tuple[Graph, np.ndarray, list[EdgeAttributes]]:
    """Build a grid-like distribution network.
    
    Args:
        rows: Number of rows
        cols: Number of columns
    
    Returns:
        Tuple of (graph, initial_weights, edge_attributes)
    """
    n = rows * cols
    edges = []
    attrs_list = []
    
    # Horizontal edges
    for i in range(rows):
        for j in range(cols - 1):
            u = i * cols + j
            v = u + 1
            edges.append([u, v])
            attrs_list.append(EdgeAttributes(
                base_cost=1.0 + np.random.random() * 0.5,
                capacity=10.0 + np.random.random() * 5.0,
                risk=1.0 + np.random.random() * 0.3,
            ))
    
    # Vertical edges
    for i in range(rows - 1):
        for j in range(cols):
            u = i * cols + j
            v = u + cols
            edges.append([u, v])
            attrs_list.append(EdgeAttributes(
                base_cost=1.0 + np.random.random() * 0.5,
                capacity=10.0 + np.random.random() * 5.0,
                risk=1.0 + np.random.random() * 0.3,
            ))
    
    # Tie lines (some diagonal connections)
    num_tie_lines = rows + cols
    for _ in range(num_tie_lines):
        u = np.random.randint(0, n)
        v = np.random.randint(0, n)
        if u != v:
            edges.append([u, v])
            attrs_list.append(EdgeAttributes(
                base_cost=2.0 + np.random.random(),
                capacity=5.0 + np.random.random() * 3.0,
                risk=1.5 + np.random.random() * 0.5,
            ))
    
    edges_array = np.array(edges, dtype=np.int64)
    graph, _ = Graph.from_edges(n, edges_array)
    
    # Initial weights (no flow)
    initial_weights = np.array([attr.base_cost * attr.risk for attr in attrs_list], dtype=np.float32)
    
    return graph, initial_weights, attrs_list


def compute_flow(graph: Graph, result, sinks: np.ndarray) -> np.ndarray:
    """Simple flow model: compute flow on each edge.
    
    This is a simplified model - in practice, you'd use a full power flow solver.
    """
    # For demo: simple flow proportional to paths used
    num_edges = graph.num_edges()
    flow = np.zeros(num_edges, dtype=np.float32)
    
    # For each sink, trace back path and add flow
    for sink in sinks:
        if result.pred is not None and result.pred[sink] >= 0:
            path = reconstruct_path(result.pred, sink)
            # Add unit flow to edges on path (simplified)
            # In real implementation, would trace actual edges
            pass
    
    # For demo: random flow
    flow = np.random.random(num_edges) * 5.0
    return flow


def main():
    """Run the grid/pipeline optimization example."""
    print("Grid and Pipeline Network Optimization Example")
    print("=" * 50)
    
    # Build network
    print("\n1. Building grid network (4x4)...")
    graph, initial_weights, attrs_list = build_grid_network(4, 4)
    print(f"   Graph: {graph.num_vertices()} vertices, {graph.num_edges()} edges")
    
    # Define source and sinks
    source = 0
    sinks = np.array([graph.num_vertices() - 1])
    
    # Initial SSSP
    print("\n2. Computing initial shortest paths...")
    start_time = time.time()
    result1 = sssp(graph, initial_weights, source=source, return_predecessors=True)
    elapsed1 = time.time() - start_time
    print(f"   Time: {elapsed1*1000:.2f} ms")
    print(f"   Cost to sink {sinks[0]}: {result1.dist[sinks[0]]:.2f}")
    
    if result1.pred is not None:
        path1 = reconstruct_path(result1.pred, sinks[0])
        print(f"   Path length: {len(path1)} vertices")
    
    # Apply load flows
    print("\n3. Applying load flows...")
    flow = compute_flow(graph, result1, sinks)
    
    # Update weights from congestion model
    print("4. Updating weights from congestion model...")
    # Use first attribute as template (simplified - in practice would use per-edge attributes)
    attrs_template = attrs_list[0] if attrs_list else EdgeAttributes(1.0, 10.0, 1.0)
    updated_weights = weight_model(flow, attrs_template, alpha=1.0)
    
    # Recompute SSSP
    print("5. Recomputing SSSP with updated weights...")
    start_time = time.time()
    result2 = sssp(graph, updated_weights, source=source, return_predecessors=True)
    elapsed2 = time.time() - start_time
    print(f"   Time: {elapsed2*1000:.2f} ms")
    print(f"   Cost to sink {sinks[0]}: {result2.dist[sinks[0]]:.2f}")
    
    if result2.pred is not None:
        path2 = reconstruct_path(result2.pred, sinks[0])
        print(f"   Path length: {len(path2)} vertices")
        if path1 != path2:
            print("   Path changed due to congestion!")
    
    # Apply outage
    print("\n6. Applying outage (disabling 10% of edges)...")
    num_outages = max(1, graph.num_edges() // 10)
    outage_edge_ids = np.random.choice(graph.num_edges(), size=num_outages, replace=False)
    weights_after_outage, enabled = apply_outage(updated_weights, edge_ids=outage_edge_ids)
    print(f"   Disabled {num_outages} edges")
    
    # Recompute SSSP after outage
    print("7. Recomputing SSSP after outage...")
    start_time = time.time()
    result3 = sssp(graph, weights_after_outage, source=source, enabled=enabled, return_predecessors=True)
    elapsed3 = time.time() - start_time
    print(f"   Time: {elapsed3*1000:.2f} ms")
    print(f"   Cost to sink {sinks[0]}: {result3.dist[sinks[0]]:.2f}")
    
    if result3.pred is not None and result3.dist[sinks[0]] < np.inf:
        path3 = reconstruct_path(result3.pred, sinks[0])
        print(f"   Path length: {len(path3)} vertices")
    else:
        print("   Sink is unreachable after outage")
    
    # Summary
    print("\n" + "=" * 50)
    print("Summary:")
    print(f"  Initial cost: {result1.dist[sinks[0]]:.2f}")
    print(f"  After congestion: {result2.dist[sinks[0]]:.2f}")
    if result3.dist[sinks[0]] < np.inf:
        print(f"  After outage: {result3.dist[sinks[0]]:.2f}")
    else:
        print("  After outage: unreachable")
    print(f"\n  Timing: {elapsed1*1000:.2f}ms / {elapsed2*1000:.2f}ms / {elapsed3*1000:.2f}ms")


if __name__ == "__main__":
    main()
