# Tutorial: Grid and Pipeline Network Optimization

This tutorial demonstrates how to use BMSSP for optimizing grid and pipeline networks with dynamic weights and outages.

## Setup

```python
import numpy as np
from bmssp import Graph, sssp, reconstruct_path
from bmssp.scenario import EdgeAttributes, weight_model, apply_outage
```

## Step 1: Build a Grid Network

```python
# Create a 4x4 grid network
n = 16  # 4x4 = 16 vertices
edges = []

# Horizontal edges
for i in range(4):
    for j in range(3):
        u = i * 4 + j
        v = u + 1
        edges.append([u, v])

# Vertical edges
for i in range(3):
    for j in range(4):
        u = i * 4 + j
        v = u + 4
        edges.append([u, v])

edges = np.array(edges, dtype=np.int64)

# Create graph
graph, initial_weights = Graph.from_edges(n, edges)
```

## Step 2: Define Edge Attributes

```python
# Define attributes for each edge
attrs = EdgeAttributes(
    base_cost=1.0,
    capacity=10.0,
    risk=1.0
)

# In practice, you'd have different attributes per edge
```

## Step 3: Compute Initial Shortest Paths

```python
source = 0
sink = 15  # Bottom-right corner

result1 = sssp(graph, initial_weights, source=source, return_predecessors=True)
print(f"Initial cost to sink: {result1.dist[sink]:.2f}")

path1 = reconstruct_path(result1.pred, sink)
print(f"Initial path: {path1}")
```

## Step 4: Apply Load Flows and Update Weights

```python
# Simulate flow on edges (simplified - in practice use power flow solver)
flow = np.random.random(graph.num_edges()) * 5.0

# Compute updated weights from congestion model
updated_weights = weight_model(flow, attrs, alpha=1.0)

# Recompute shortest paths
result2 = sssp(graph, updated_weights, source=source, return_predecessors=True)
print(f"Cost after congestion: {result2.dist[sink]:.2f}")

path2 = reconstruct_path(result2.pred, sink)
if path1 != path2:
    print("Path changed due to congestion!")
```

## Step 5: Handle Outages

```python
# Apply outage to 10% of edges
num_outages = max(1, graph.num_edges() // 10)
outage_edge_ids = np.random.choice(graph.num_edges(), size=num_outages, replace=False)

weights_after_outage, enabled = apply_outage(updated_weights, edge_ids=outage_edge_ids)

# Recompute paths with outage
result3 = sssp(graph, weights_after_outage, source=source, enabled=enabled, return_predecessors=True)

if result3.dist[sink] < np.inf:
    print(f"Cost after outage: {result3.dist[sink]:.2f}")
    path3 = reconstruct_path(result3.pred, sink)
    print(f"Path after outage: {path3}")
else:
    print("Sink is unreachable after outage")
```

## Step 6: Scenario Analysis

```python
# Run multiple scenarios
scenarios = []
for scenario_id in range(10):
    # Vary flow
    flow = np.random.random(graph.num_edges()) * 5.0
    weights = weight_model(flow, attrs)
    
    # Apply random outages
    num_outages = np.random.randint(0, graph.num_edges() // 5)
    outage_ids = np.random.choice(graph.num_edges(), size=num_outages, replace=False)
    weights, enabled = apply_outage(weights, edge_ids=outage_ids)
    
    # Compute cost
    result = sssp(graph, weights, source=source, enabled=enabled)
    cost = result.dist[sink]
    
    scenarios.append({
        'scenario': scenario_id,
        'cost': cost,
        'reachable': cost < np.inf
    })

# Analyze scenarios
reachable_scenarios = [s for s in scenarios if s['reachable']]
print(f"Reachable scenarios: {len(reachable_scenarios)}/{len(scenarios)}")
if reachable_scenarios:
    avg_cost = np.mean([s['cost'] for s in reachable_scenarios])
    print(f"Average cost: {avg_cost:.2f}")
```

## Complete Example

See `python/examples/grid_pipeline.py` for a complete working example.
