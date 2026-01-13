# Tutorial: Grid and Pipeline Network Optimization

This tutorial demonstrates how to use BMSSP for optimizing grid and pipeline networks with dynamic weights and outages.

## Setup

```python
import numpy as np
import time
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

## Complete Scenario Loop Example

Here's a complete example showing the full workflow:

```python
import numpy as np
import time
from bmssp import Graph, sssp, reconstruct_path
from bmssp.scenario import EdgeAttributes, weight_model, apply_outage

# Build a simple network
n = 5
edges = np.array([
    [0, 1], [1, 2], [2, 3], [3, 4],
    [0, 2], [1, 3], [2, 4],  # Alternative paths
], dtype=np.int64)

# Create edge attributes
attrs_list = [
    EdgeAttributes(base_cost=1.0, capacity=10.0, risk=1.0) 
    for _ in range(len(edges))
]
initial_weights = np.array([attr.base_cost * attr.risk for attr in attrs_list], dtype=np.float32)

graph, sorted_weights, sorted_attrs = Graph.from_edges(n, edges, weights=initial_weights)

source = 0
sink = 4

# Step 1: Initial SSSP
start = time.time()
result1 = sssp(graph, sorted_weights, source=source, return_predecessors=True)
time1 = time.time() - start
print(f"Initial cost: {result1.dist[sink]:.2f}, Time: {time1*1000:.2f}ms")
path1 = reconstruct_path(result1.pred, sink)

# Step 2: Simulate flow and update weights
# In practice, you'd compute flow from a power flow solver
# For demo, use simple flow based on paths
flow = np.ones(graph.num_edges(), dtype=np.float32) * 2.0
updated_weights = weight_model(flow, sorted_attrs, alpha=1.0)

# Step 3: Recompute with updated weights
start = time.time()
result2 = sssp(graph, updated_weights, source=source, return_predecessors=True)
time2 = time.time() - start
print(f"After congestion: {result2.dist[sink]:.2f}, Time: {time2*1000:.2f}ms")
path2 = reconstruct_path(result2.pred, sink)

if path1 != path2:
    print("Path changed due to congestion!")

# Step 4: Apply outage
outage_ids = np.array([0])  # Disable first edge
weights_after_outage, enabled = apply_outage(updated_weights, edge_ids=outage_ids)

# Step 5: Recompute after outage
start = time.time()
result3 = sssp(graph, weights_after_outage, source=source, enabled=enabled, return_predecessors=True)
time3 = time.time() - start

if result3.dist[sink] < np.inf:
    print(f"After outage: {result3.dist[sink]:.2f}, Time: {time3*1000:.2f}ms")
    path3 = reconstruct_path(result3.pred, sink)
    if path3 != path2:
        print("Path changed due to outage!")
else:
    print("Sink unreachable after outage")

# Performance note
print(f"\nTiming ratio: {time2/time1:.2f}x, {time3/time1:.2f}x (recomputations are fast!)")
```

## Dynamic Weight Updates

BMSSP is optimized for scenarios where edge weights change frequently but the graph topology remains fixed. This is common in grid and pipeline networks where:

- **Congestion models** update weights based on current flow
- **Load variations** change effective costs
- **Risk assessments** adjust edge weights dynamically

### Example: Multiple Weight Updates

```python
# Build graph once
graph, initial_weights, _ = Graph.from_edges(n, edges, weights=base_weights)

# Update weights multiple times (fast - no graph rebuild)
scenarios = []
for iteration in range(100):
    # Update weights (e.g., from congestion model)
    new_weights = compute_updated_weights(initial_weights, iteration)
    
    # Fast recomputation
    result = sssp(graph, new_weights, source=0)
    scenarios.append(result.dist[sink])

# Graph topology never changes - only weights update
# This is much faster than rebuilding the graph for each scenario
```

### Weight Model Details

The `weight_model()` function computes effective weights from flow and attributes:

```
weight = base_cost * risk * (1 + alpha * (flow/capacity)^2)
```

- `base_cost`: Base cost per edge
- `risk`: Risk multiplier
- `flow`: Current flow on edge
- `capacity`: Edge capacity
- `alpha`: Congestion factor (default: 1.0)

For per-edge attributes, pass a list:

```python
attrs_list = [
    EdgeAttributes(base_cost=1.0, capacity=10.0, risk=1.0),
    EdgeAttributes(base_cost=2.0, capacity=5.0, risk=1.5),
    # ... one per edge
]
weights = weight_model(flow, attrs_list, alpha=1.0)
```

## Outage Simulation

Outages are simulated using the `enabled` mask parameter. This approach is fast because it doesn't require rebuilding the graph topology.

### Basic Outage

```python
# Apply outage to specific edges by ID
outage_edge_ids = np.array([0, 5, 10], dtype=np.int64)
weights_after, enabled = apply_outage(weights, edge_ids=outage_edge_ids)

result = sssp(graph, weights_after, source=0, enabled=enabled)
```

### Outage with Mask

```python
# Apply outage using boolean mask
edge_mask = np.zeros(graph.num_edges(), dtype=bool)
edge_mask[0] = True  # Disable edge 0
edge_mask[5] = True  # Disable edge 5

weights_after, enabled = apply_outage(weights, edge_mask=edge_mask)
result = sssp(graph, weights_after, source=0, enabled=enabled)
```

### Multiple Outage Scenarios

```python
# Test multiple outage scenarios efficiently
outage_scenarios = [
    np.array([0]),      # Single edge outage
    np.array([0, 1]),   # Two edge outage
    np.array([5, 10, 15]),  # Three edge outage
]

for outage_ids in outage_scenarios:
    _, enabled = apply_outage(weights, edge_ids=outage_ids)
    result = sssp(graph, weights, source=0, enabled=enabled)
    print(f"Outage {outage_ids}: cost = {result.dist[sink]:.2f}")
```

### Handling Unreachable Vertices

```python
result = sssp(graph, weights, source=0, enabled=enabled)

# Check if vertex is reachable
if result.dist[sink] < np.inf:
    path = reconstruct_path(result.pred, sink)
    print(f"Path: {path}")
else:
    print("Sink is unreachable after outage")
```

## Performance Considerations

BMSSP is designed for scenarios with many repeated SSSP calls. Key performance features:

1. **Immutable Graph Topology**: Build the graph once, reuse for many scenarios
2. **Fast Re-computation**: Subsequent calls are fast because topology is cached
3. **Enabled Mask Efficiency**: Outage simulation via masks is faster than rebuilding graphs

### Timing Example

```python
import time

# Build graph once
graph, weights, _ = Graph.from_edges(n, edges, weights=base_weights)

# Initial computation
start = time.time()
result1 = sssp(graph, weights, source=0)
time_initial = time.time() - start

# Multiple recomputations with different weights
times_recompute = []
for i in range(10):
    new_weights = update_weights(weights, i)
    start = time.time()
    result = sssp(graph, new_weights, source=0)
    times_recompute.append(time.time() - start)

avg_recompute = np.mean(times_recompute)
print(f"Initial: {time_initial*1000:.2f}ms")
print(f"Average recompute: {avg_recompute*1000:.2f}ms")
print(f"Ratio: {avg_recompute/time_initial:.2f}x")
```

### Best Practices

1. **Build graph once**: Create the graph outside your scenario loop
2. **Update weights in-place**: Modify weight arrays rather than creating new graphs
3. **Use enabled masks for outages**: Don't rebuild graphs for outages
4. **Batch scenarios**: Process multiple scenarios with the same graph topology

## Troubleshooting

### Common Issues

**Problem**: "Weights length X != graph edges Y"
- **Solution**: Ensure weight array length matches the number of edges in the graph. Use `graph.num_edges()` to get the correct length.

**Problem**: "Source out of range"
- **Solution**: Source vertex must be in `[0, graph.num_vertices())`. Check your source index.

**Problem**: Sink is always unreachable
- **Solution**: 
  - Verify the graph is connected from source to sink
  - Check if enabled mask is disabling critical edges
  - Ensure weights are finite (not infinite)

**Problem**: Path reconstruction returns empty list
- **Solution**: Check if the target vertex is reachable: `result.dist[target] < np.inf`. Also ensure `return_predecessors=True` was used.

**Problem**: Weights don't match edge attributes
- **Solution**: When using `Graph.from_edges()`, edges are sorted. Ensure your attributes list matches the sorted edge order, or use per-edge attributes with `weight_model()`.

### Debugging Tips

1. **Verify graph structure**:
   ```python
   print(f"Vertices: {graph.num_vertices()}, Edges: {graph.num_edges()}")
   ```

2. **Check reachability**:
   ```python
   result = sssp(graph, weights, source=0)
   reachable = result.dist < np.inf
   print(f"Reachable vertices: {np.sum(reachable)}/{graph.num_vertices()}")
   ```

3. **Verify enabled mask**:
   ```python
   _, enabled = apply_outage(weights, edge_ids=outage_ids)
   print(f"Enabled edges: {np.sum(enabled)}/{len(enabled)}")
   ```

4. **Check path validity**:
   ```python
   path = reconstruct_path(result.pred, target)
   if len(path) > 0:
       print(f"Path length: {len(path)}, Cost: {result.dist[target]:.2f}")
   else:
       print("Path is empty - vertex unreachable")
   ```
