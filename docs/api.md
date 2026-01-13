# API Reference

## Core Types

### `bmssp.Graph`

Immutable directed graph backed by CSR (Compressed Sparse Row) format.

#### Constructors

##### `Graph.from_csr(indptr, indices, n=None, edge_ids=None)`

Create a graph from CSR arrays.

**Parameters:**
- `indptr` (np.ndarray[int32|int64]): Index pointer array (length n+1)
- `indices` (np.ndarray[int32|int64]): Column indices array (length m)
- `n` (int, optional): Number of vertices (default: len(indptr) - 1)
- `edge_ids` (np.ndarray, optional): Optional edge ID mapping

**Returns:** `Graph` instance

##### `Graph.from_edges(n, edges, weights=None, sort=True, dedupe="min")`

Create a graph from an edge list.

**Parameters:**
- `n` (int): Number of vertices
- `edges` (np.ndarray): Edge array of shape (m, 2) with (u, v) pairs
- `weights` (np.ndarray, optional): Edge weights for deduplication
- `sort` (bool): Whether to sort edges (required for CSR, default: True)
- `dedupe` (str): How to handle duplicates: "min", "first", "last" (default: "min")

**Returns:** `(Graph, weights_array)` tuple

#### Methods

##### `num_vertices() -> int`

Return the number of vertices.

##### `num_edges() -> int`

Return the number of edges.

## Algorithms

### `bmssp.sssp(graph, weights, source, enabled=None, return_predecessors=False) -> SSSPResult`

Compute single-source shortest paths.

**Parameters:**
- `graph` (Graph): Graph object
- `weights` (np.ndarray[float32|float64]): Edge weights (length = number of edges)
- `source` (int): Source vertex index
- `enabled` (np.ndarray[bool], optional): Boolean mask for enabled edges (None = all enabled)
- `return_predecessors` (bool): Whether to return predecessor arrays (default: False)

**Returns:** `SSSPResult` with distances and optionally predecessors

**Examples:**

Basic usage:
```python
result = sssp(graph, weights, source=0)
print(result.dist)  # Distances from source to each vertex
```

With predecessors for path reconstruction:
```python
result = sssp(graph, weights, source=0, return_predecessors=True)
path = reconstruct_path(result.pred, target=5)
```

With enabled mask for outage simulation:
```python
# Disable specific edges
enabled = np.ones(graph.num_edges(), dtype=bool)
enabled[0] = False  # Disable edge 0
enabled[5] = False  # Disable edge 5

result = sssp(graph, weights, source=0, enabled=enabled)
# Disabled edges are skipped during computation
```

Using `apply_outage` helper:
```python
from bmssp.scenario import apply_outage

# Apply outage to edges by ID
weights_after, enabled = apply_outage(weights, edge_ids=np.array([0, 5]))
result = sssp(graph, weights_after, source=0, enabled=enabled)
```

### `bmssp.SSSPResult`

Result of SSSP computation.

**Attributes:**
- `dist` (np.ndarray): Distances from source to each vertex (infinity if unreachable)
- `pred` (np.ndarray[int32] | None): Predecessor vertex array (None if not requested)
- `pred_edge` (np.ndarray[int64] | None): Predecessor edge array (currently None)

## Helper Functions

### `bmssp.reconstruct_path(pred, target) -> list[int]`

Reconstruct path from source to target using predecessor array.

**Parameters:**
- `pred` (np.ndarray[int32]): Predecessor array from SSSPResult
- `target` (int): Target vertex index

**Returns:** List of vertex indices from source to target (inclusive), empty if unreachable

**Examples:**

Basic path reconstruction:
```python
result = sssp(graph, weights, source=0, return_predecessors=True)
path = reconstruct_path(result.pred, target=5)
# Returns: [0, 2, 4, 5] (example path)
```

Handling unreachable vertices:
```python
result = sssp(graph, weights, source=0, return_predecessors=True)
if result.dist[target] < np.inf:
    path = reconstruct_path(result.pred, target)
    print(f"Path: {path}")
else:
    print("Target is unreachable")
```

Reconstructing paths after outages:
```python
result_before = sssp(graph, weights, source=0, return_predecessors=True)
path_before = reconstruct_path(result_before.pred, target)

# Apply outage
_, enabled = apply_outage(weights, edge_ids=np.array([0]))
result_after = sssp(graph, weights, source=0, enabled=enabled, return_predecessors=True)
path_after = reconstruct_path(result_after.pred, target)

if path_before != path_after:
    print("Path changed after outage")
```

### `bmssp.multi_sink_costs(dist, sinks) -> np.ndarray`

Extract distances to multiple sink vertices.

**Parameters:**
- `dist` (np.ndarray): Distance array from SSSPResult.dist
- `sinks` (np.ndarray): Array of sink vertex indices

**Returns:** Array of distances to each sink

## Scenario Utilities

### `bmssp.scenario.EdgeAttributes`

Dataclass for edge attributes in grid/pipeline networks.

**Attributes:**
- `base_cost` (float): Base cost for using this edge
- `capacity` (float): Capacity of the edge
- `risk` (float): Risk factor (multiplies cost)
- `region` (str, optional): Optional region identifier
- `line_type` (str, optional): Optional line type
- `is_switchable` (bool): Whether edge can be switched on/off

### `bmssp.scenario.weight_model(flow, attrs, alpha=1.0) -> np.ndarray`

Compute effective weights from flow and edge attributes.

Default model: `w = base_cost * risk * (1 + alpha * (flow/capacity)^2)`

**Parameters:**
- `flow` (np.ndarray): Current flow per edge (length = number of edges)
- `attrs` (EdgeAttributes or list[EdgeAttributes]): Edge attributes (single or per-edge)
- `alpha` (float): Congestion factor (default: 1.0)

**Returns:** Weight array (length = number of edges)

**Examples:**

Single attribute for all edges:
```python
attrs = EdgeAttributes(base_cost=1.0, capacity=10.0, risk=1.0)
flow = np.array([2.0, 5.0, 0.0], dtype=np.float32)
weights = weight_model(flow, attrs, alpha=1.0)
# weights[0] = 1.0 * (1 + 1.0 * (2.0/10.0)^2) = 1.04
# weights[1] = 1.0 * (1 + 1.0 * (5.0/10.0)^2) = 1.25
# weights[2] = 1.0 * (1 + 0) = 1.0
```

Per-edge attributes:
```python
attrs_list = [
    EdgeAttributes(base_cost=1.0, capacity=10.0, risk=1.0),
    EdgeAttributes(base_cost=2.0, capacity=5.0, risk=1.5),
    EdgeAttributes(base_cost=1.0, capacity=20.0, risk=1.0),
]
flow = np.array([5.0, 3.0, 10.0], dtype=np.float32)
weights = weight_model(flow, attrs_list, alpha=1.0)
# Each edge uses its own attributes
```

Integration with scenario loop:
```python
# Initial weights
initial_weights = np.array([attr.base_cost * attr.risk for attr in attrs_list])

# Compute flow (simplified - in practice use power flow solver)
flow = compute_flow(graph, result, sinks)

# Update weights based on congestion
updated_weights = weight_model(flow, attrs_list, alpha=1.0)

# Recompute paths with updated weights
result = sssp(graph, updated_weights, source=0)
```

### `bmssp.scenario.apply_outage(weights, edge_mask=None, edge_ids=None, penalty=inf) -> tuple`

Apply outage to edges. Creates an enabled mask for use with `sssp()`.

**Parameters:**
- `weights` (np.ndarray): Current edge weights (length = number of edges)
- `edge_mask` (np.ndarray[bool], optional): Boolean mask of edges to disable (True = disable)
- `edge_ids` (np.ndarray, optional): Indices of edges to disable
- `penalty` (float): Penalty weight for disabled edges (default: inf)

**Returns:** Tuple of (updated_weights, enabled_mask)
- `updated_weights`: Weights with disabled edges set to penalty
- `enabled_mask`: Boolean mask (False = disabled, True = enabled) for use with `sssp(enabled=...)`

**Note:** Either `edge_mask` or `edge_ids` must be provided, not both.

**Examples:**

Using edge IDs:
```python
weights = np.array([1.0, 2.0, 3.0, 4.0], dtype=np.float32)
outage_ids = np.array([0, 2], dtype=np.int64)

weights_after, enabled = apply_outage(weights, edge_ids=outage_ids)
# weights_after[0] = inf, weights_after[2] = inf
# enabled[0] = False, enabled[2] = False, enabled[1] = True, enabled[3] = True

result = sssp(graph, weights_after, source=0, enabled=enabled)
```

Using edge mask:
```python
weights = np.array([1.0, 2.0, 3.0, 4.0], dtype=np.float32)
edge_mask = np.array([True, False, True, False], dtype=bool)

weights_after, enabled = apply_outage(weights, edge_mask=edge_mask)
result = sssp(graph, weights_after, source=0, enabled=enabled)
```

Multiple outage scenarios:
```python
# Test multiple outage scenarios
outage_scenarios = [
    np.array([0]),
    np.array([0, 1]),
    np.array([5, 10, 15]),
]

for outage_ids in outage_scenarios:
    _, enabled = apply_outage(weights, edge_ids=outage_ids)
    result = sssp(graph, weights, source=0, enabled=enabled)
    print(f"Outage {outage_ids}: cost = {result.dist[sink]:.2f}")
```

**Edge Cases and Limitations:**

- If all edges are disabled, all vertices except the source become unreachable
- Disabling all incoming edges to a vertex makes that vertex unreachable
- The enabled mask must be passed to `sssp()` for outages to take effect (setting weights to inf alone is not sufficient)
- Edge IDs must be in range `[0, len(weights))`
- Edge mask length must equal `len(weights)`

## Scenario Integration Examples

### Complete Scenario Workflow

Here's a complete example integrating all scenario utilities:

```python
import numpy as np
from bmssp import Graph, sssp, reconstruct_path
from bmssp.scenario import EdgeAttributes, weight_model, apply_outage

# Build graph
n = 5
edges = np.array([[0, 1], [1, 2], [2, 3], [3, 4], [0, 2], [2, 4]], dtype=np.int64)
attrs_list = [EdgeAttributes(base_cost=1.0, capacity=10.0, risk=1.0) for _ in range(len(edges))]
initial_weights = np.array([attr.base_cost * attr.risk for attr in attrs_list], dtype=np.float32)

graph, sorted_weights, sorted_attrs = Graph.from_edges(n, edges, weights=initial_weights)

# Step 1: Initial SSSP
result1 = sssp(graph, sorted_weights, source=0, return_predecessors=True)
path1 = reconstruct_path(result1.pred, target=4)

# Step 2: Update weights based on flow (congestion model)
flow = np.ones(graph.num_edges(), dtype=np.float32) * 5.0
updated_weights = weight_model(flow, sorted_attrs, alpha=1.0)

# Step 3: Recompute with updated weights
result2 = sssp(graph, updated_weights, source=0, return_predecessors=True)
path2 = reconstruct_path(result2.pred, target=4)

# Step 4: Apply outage
weights_after_outage, enabled = apply_outage(updated_weights, edge_ids=np.array([0]))

# Step 5: Recompute after outage
result3 = sssp(graph, weights_after_outage, source=0, enabled=enabled, return_predecessors=True)
if result3.dist[4] < np.inf:
    path3 = reconstruct_path(result3.pred, target=4)
```

### Performance Pattern: Build Once, Reuse Many Times

```python
# Build graph once (outside loop)
graph, initial_weights, attrs_list = build_network()

# Process many scenarios
for scenario_id in range(100):
    # Update weights (fast - no graph rebuild)
    flow = compute_flow_for_scenario(scenario_id)
    weights = weight_model(flow, attrs_list, alpha=1.0)
    
    # Apply outages (fast - uses enabled mask)
    _, enabled = apply_outage(weights, edge_ids=get_outage_edges(scenario_id))
    
    # Fast recomputation (graph topology unchanged)
    result = sssp(graph, weights, source=0, enabled=enabled)
    
    process_result(result, scenario_id)
```
