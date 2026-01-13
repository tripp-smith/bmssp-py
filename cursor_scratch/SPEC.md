## Objective

Ship a Python package, `bmssp`, that exposes a clean, Pythonic shortest-path API while delegating the heavy work to a Rust backend implementing BMSSP (the “sorting barrier” directed SSSP algorithm for non-negative weights).

Primary target use case: grid, pipeline, power distribution networks modeled as directed graphs where edge weights reflect cost, risk, congestion, or capacity-driven penalties, supporting:

* Fast re-routing after outages (edge removals, capacity drops, topology changes)
* Cost-efficient distribution planning under varying load (weights change frequently)
* “Tight loop” integration into near-real-time operations (fast repeated SSSP calls)

Non-goals for v1:

* Negative weights
* All-pairs shortest paths
* Full min-cost flow / OPF (we focus on the SSSP primitive that those systems often call repeatedly)

## Scope and constraints

Algorithmic scope

* Directed graph, non-negative edge weights
* Single-source shortest paths (SSSP)
* Output: distances, and optionally predecessor pointers for path reconstruction
* Determinism: exact shortest paths

Performance scope

* Primary win comes from a faster SSSP routine under repeated calls on large sparse graphs
* Real value is expected when:

  * Graph is large (hundreds of thousands to millions of edges)
  * You call SSSP many times (different sources, scenarios, outages, weight updates)

Engineering scope

* Rust core is the authoritative implementation
* Python is a thin, stable API layer with good ergonomics and domain examples

## Reference implementations to learn from

You have multiple public codebases you can consult to avoid “blank sheet” risk:

* Go BMSSP implementations (two independent repos)
* Rust BMSSP implementations (at least one crate with published source and some repo variants)
* A TypeScript implementation that can be used as a readability reference

Build plan assumes:

* Use one Go implementation as a behavioral reference (test oracle against Dijkstra)
* Use the Rust crate source as a structural reference for the major components (block heap, pivot selection, recursion), but write and own your codebase

## High-level architecture

### Repository layout

Monorepo recommended:

* `rust/`

  * `bmssp-core/` (pure Rust library crate)
  * `bmssp-py/` (pyo3 bindings crate; depends on `bmssp-core`)
* `python/`

  * `bmssp/` (Python package wrapper: user-facing API, utilities, examples)
  * `tests/`
  * `examples/`
  * `benchmarks/`
* `docs/`

  * API docs, design notes, algorithm notes, performance tips

### Data flow

Python user code supplies one of:

* CSR arrays (preferred for performance and memory)
* Edge list (convenience; converted once to CSR)
* A “Graph” object that owns CSR and supports cheap copies/views

Rust core performs:

* BMSSP SSSP computation given graph + source + weights array
* Returns distances (+ predecessors when requested)

Python layer performs:

* Domain modeling for the grid/pipeline example (capacity/risk/load to weights)
* Scenario simulation (outages, varying load) by updating weight arrays and/or disabling edges

## User-facing Python API spec

### Core types

`bmssp.Graph`

* Immutable graph topology, backed by CSR
* Stores:

  * `n: int`
  * `indptr: np.ndarray[int32|int64]`
  * `indices: np.ndarray[int32|int64]`
  * `edge_ids: np.ndarray[int32|int64]` (optional; useful for mapping domain edges to CSR positions)

Constructors:

* `Graph.from_edges(n, edges: np.ndarray[(u,v)], *, sort=True, dedupe="min")`
* `Graph.from_csr(indptr, indices, *, n=None)`
* `Graph.grid(rows, cols, directed=True, diagonals=False)` (example convenience, not mandatory for v1)

`bmssp.Weights`

* Holds weights aligned with CSR edge order
* Backed by `np.ndarray[float32|float64]`
* Can be updated in-place by domain code

### Algorithms

`bmssp.sssp(graph: Graph, weights: np.ndarray, source: int, *, return_predecessors=False) -> SSSPResult`

`SSSPResult`

* `dist: np.ndarray[float32|float64]`
* `pred: np.ndarray[int32] | None`
* `pred_edge: np.ndarray[int64] | None` (optional; useful for reconstructing edge-level paths)

Helpers:

* `bmssp.reconstruct_path(pred, target) -> list[int]`
* `bmssp.multi_sink_costs(dist, sinks) -> np.ndarray` (thin helper)

### Scenario utilities for the target use case

`bmssp.scenario.EdgeAttributes` (pure Python dataclass)

* `base_cost`
* `capacity`
* `risk`
* optional: `region`, `line_type`, `is_switchable`, etc.

`bmssp.scenario.weight_model(flow, attrs) -> weights`

* Given current flow per edge, compute effective weights
* Default model: `w = base_cost * risk * (1 + alpha*(flow/capacity)^2)`
* Supports swapping risk model or congestion model

`bmssp.scenario.apply_outage(weights, edge_mask|edge_ids, penalty=np.inf)`

* Either:

  * remove edges via mask (preferred: pass an “enabled mask” to Rust)
  * set weights to a very large value (works, but can distort numeric range)

Recommended v1 approach for outages:

* Support an `enabled: Optional[np.ndarray[bool]]` mask in Rust (fast and avoids giant weights)
* That mask is scenario-specific and can change per run without rebuilding CSR

So v1 `sssp` becomes:
`sssp(graph, weights, source, enabled=None, return_predecessors=False)`

## Rust core design spec

### Crates

`bmssp-core` (no Python dependencies)

* Implements:

  * CSR graph representation
  * BMSSP algorithm (recursive)
  * Dijkstra fallback (for debugging and benchmarking)
  * Validation utilities

`bmssp-py` (bindings)

* Exposes:

  * `sssp_csr(indptr, indices, weights, source, enabled=None, return_pred=False)`
  * Minimal validation; keep overhead low

### Graph representation

CSR layout:

* `indptr: Vec<usize>` length `n+1`
* `indices: Vec<usize>` length `m`
* `weights` passed per call as slice `&[f32]` or `&[f64]`
* Optional `enabled` mask slice `&[u8]` or `&[bool]` length `m`

Keep topology immutable for best caching and stable edge ordering.

### Numeric types

Support `f32` and `f64` via generics:

* v1 can start with `f32` only (simpler), then extend to `f64`
* Provide Python dispatch based on numpy dtype

Infinity handling:

* Use `T::INFINITY` for unreachable
* Avoid NaNs; reject NaN weights

### Algorithm components (BMSSP structure)

You will implement the BMSSP-style recursion with these internal pieces:

1. Pivot finding subroutine

* Bounded relaxations to build a candidate set `W`
* Predecessor forest tracking for tree sizes
* Selection of pivot set `P` (sources that have large enough trees)

2. Base case

* Bounded Dijkstra-like expansion, limited by `k` and bounded by `b` (cap)
* Uses an ordered structure keyed by tentative distance

3. Block heap / frontier

* A structure that can “pull” a block of up to `m` vertices with smallest keys
* Exposes `b_next` as the next key distance (or cap)
* Maintains decrease-key behavior

4. Main recursion

* Parameters computed from `n`:

  * `t`, `k`, `l` (based on log factors)
* Recursively processes blocks; relax edges; repopulates frontier

Engineering choices:

* Use adjacency via CSR; relaxing edges from a vertex is iterating over `indices[indptr[u]..indptr[u+1]]`
* For `BlockHeap`, use a structure that supports:

  * ordered extraction by distance
  * decrease-key
* Rust options:

  * `BTreeSet` + map for decrease-key (simple, matches many reference implementations)
  * A pairing heap or binary heap plus “stale entries” trick (faster; no delete)
* Recommendation:

  * Start with the simpler ordered-set approach to validate correctness
  * Add an optimized heap implementation once test parity is strong

### Enabled mask for outages

Edge relax should check:

* if `enabled` is present and `enabled[eid] == false`: skip

This supports fast outage simulation without changing CSR.

### Predecessors

For path reconstruction, store:

* `pred_vertex[v] = u`
* `pred_edge[v] = eid` (edge id within CSR)

Update pred when a relaxation improves a distance.

### Safety and validation

In Rust:

* Validate:

  * `weights.len() == m`
  * `enabled` length matches `m` if present
  * `source < n`
  * all weights finite and `>= 0`
* Expose a “checked” API for Python
* Keep an “unchecked” internal function for benchmarks and controlled use

## Python bindings (pyo3 + maturin)

### Binding functions

Expose one function per dtype:

* `sssp_f32_csr(...)`
* `sssp_f64_csr(...)`

Python-level `sssp(...)` dispatches by dtype.

Data passing

* Accept NumPy arrays without copies:

  * `indptr`: int32 or int64
  * `indices`: int32 or int64
  * `weights`: float32 or float64
  * `enabled`: optional bool or uint8

Return

* `dist` as NumPy array
* `pred` arrays if requested

Packaging

* Build wheels for:

  * manylinux x86_64 and aarch64
  * macOS x86_64 and arm64
  * Windows x86_64
* Use `maturin` in CI for reproducible builds

## Testing plan

Correctness tests must be the gate.

### Unit tests (Rust)

Graph invariants

* CSR validity checks

Algorithm invariants

* Distances are non-decreasing under relaxation
* Unreachable stays INF
* Pred path sums match dist within tolerance

Golden tests vs Dijkstra

* Random directed graphs with non-negative weights
* Grid graphs
* “Pipeline-like” graphs (mostly chain, with sparse bypass links)
* Compare:

  * `dist_bmssp == dist_dijkstra` (within float tolerance)
  * Pred reconstruct path cost matches dist (when pred requested)

Property tests (optional but valuable)

* Use `proptest` to generate graphs; assert parity vs Dijkstra

### Python tests

* Parity vs `scipy.sparse.csgraph.dijkstra` when SciPy is available
* Parity vs a pure Python Dijkstra for small graphs (no SciPy dependency)
* Scenario tests:

  * Outage simulation changes reachability/cost as expected
  * Varying load scenario increases cost on congested corridors and can change chosen route

## Benchmark plan

Two layers: Rust microbench and Python end-to-end.

Rust (criterion)

* `bmssp` vs `dijkstra`
* Graph families:

  * Random sparse directed (E ~ 5V)
  * Grid directed (E ~ 2V) with some back edges
  * Pipeline chain with bypasses
* Track:

  * time
  * peak memory (rough proxy: allocations; optional tooling)
  * scaling curves

Python (pytest-benchmark)

* Repeat SSSP across 100–1000 scenarios:

  * Change weights only
  * Change enabled mask (outages)
* Compare:

  * `bmssp` binding
  * NetworkX Dijkstra (baseline, slow)
  * SciPy Dijkstra (baseline, fast but different strengths)

## Deliverable: the grid and pipeline network optimization example

Provide a first-class example in `examples/grid_pipeline.py` and in docs.

Example requirements

* Build a directed graph for:

  * A grid-like distribution network, plus “tie lines” for reconfiguration
  * A pipeline network with compressor-like edges and bypasses
* Edge attributes:

  * `base_cost`, `risk`, `capacity`
* Scenario loop:

  1. Compute initial SSSP from a supply node
  2. Apply load flows to edges (simple synthetic flow model is fine for demo)
  3. Update weights from congestion model
  4. Recompute SSSP
  5. Apply outage mask to a subset of edges
  6. Recompute SSSP
* Outputs:

  * Cost-to-serve a list of sink nodes
  * Reconstructed path(s) for at least one sink (before/after outage)
  * Basic timing for repeated calls (to highlight the “real-time” angle)

## Milestones

Milestone 1: Skeleton package

* `bmssp-core` CSR + Dijkstra baseline
* `bmssp-py` bindings that call Dijkstra
* Python `Graph` + `sssp` API working end-to-end

Milestone 2: BMSSP correctness

* Implement BMSSP in Rust using the reference structure (pivot selection, base case, block frontier, recursion)
* Extensive parity tests vs Dijkstra

Milestone 3: Outages + dynamic weights

* `enabled` mask supported in Rust and Python
* Example scenario implements:

  * varying load weights
  * outage reconfiguration
* Predecessors returned and paths reconstructed

Milestone 4: Performance hardening

* Replace ordered-set decrease-key structures if needed
* Reduce allocations (reuse buffers, pre-allocate)
* Add feature flags for `f32` vs `f64`, optional predecessors

Milestone 5: Packaging and docs

* Wheels built in CI for major platforms
* Published docs with:

  * API reference
  * “Grid and pipeline optimization” tutorial
  * Performance guidance and caveats

## Key design decisions to lock early

1. Graph topology immutable, weights mutable

* Best match for repeated scenario evaluation
* Avoid rebuilding CSR on every scenario

2. Outages via enabled mask

* Faster than rebuilding topology
* Cleaner than “infinite weight” hacks

3. Return predecessors optionally

* Most operational systems need the route, not just the cost
* Make it optional to keep hot path fast

4. Start with correctness-first BMSSP

* Use Dijkstra parity as the safety net
* Optimize once parity is stable

## Acceptance criteria

A “done” v1 satisfies:

* `pip install bmssp` works with prebuilt wheels on macOS, Linux, Windows
* `bmssp.sssp()` returns exact distances matching Dijkstra on test suites
* The provided example demonstrates:

  * varying load weight updates
  * outage reconfiguration via enabled mask
  * route reconstruction for at least one sink
* Performance shows clear improvement in repeated SSSP calls on at least one realistic sparse network size, without sacrificing correctness

If you want, I can convert this into a tracked implementation checklist (file-by-file, function-by-function), with a minimal first sprint that gets the Python API and Dijkstra baseline in place, then swaps in BMSSP behind the same interface.
