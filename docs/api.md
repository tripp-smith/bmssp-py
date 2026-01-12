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
- `enabled` (np.ndarray[bool], optional): Boolean mask for enabled edges
- `return_predecessors` (bool): Whether to return predecessor arrays (default: False)

**Returns:** `SSSPResult` with distances and optionally predecessors

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

**Returns:** List of vertex indices from source to target (empty if unreachable)

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
- `flow` (np.ndarray): Current flow per edge
- `attrs` (EdgeAttributes or list): Edge attributes
- `alpha` (float): Congestion factor (default: 1.0)

**Returns:** Weight array

### `bmssp.scenario.apply_outage(weights, edge_mask=None, edge_ids=None, penalty=inf) -> tuple`

Apply outage to edges.

**Parameters:**
- `weights` (np.ndarray): Current edge weights
- `edge_mask` (np.ndarray[bool], optional): Boolean mask of edges to disable
- `edge_ids` (np.ndarray, optional): Indices of edges to disable
- `penalty` (float): Penalty weight for disabled edges (default: inf)

**Returns:** Tuple of (updated_weights, enabled_mask)
