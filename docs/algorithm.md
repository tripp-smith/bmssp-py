# BMSSP Algorithm

## Overview

BMSSP (Blocked Multi-Source Shortest Path) is an algorithm for computing single-source shortest paths (SSSP) on directed graphs with non-negative edge weights. It is designed to be faster than Dijkstra's algorithm for large sparse graphs, especially when repeated calls are needed (e.g., in scenario analysis).

## Current Implementation Status

**Milestone 2 Complete**: The BMSSP algorithm is now implemented using a block-based approach. The implementation processes vertices in blocks using a BlockHeap structure, matching Dijkstra's correctness while using the block-processing structure of BMSSP.

## Algorithm Structure

The BMSSP algorithm uses a recursive divide-and-conquer approach:

1. **Parameters**: Compute algorithm parameters (t, k, l) based on graph size
2. **Pivot Selection**: Identify pivot vertices using bounded relaxations
3. **Block Processing**: Process blocks of vertices recursively
4. **Base Case**: Use bounded Dijkstra expansion for small subproblems

### Key Components

- **Block Heap**: Maintains frontier of vertices ordered by distance
- **Pivot Finding**: Selects vertices with large predecessor trees
- **Recursion**: Recursively processes blocks to compute shortest paths

## Complexity

- **Time**: O(m + n log n) expected (improves over Dijkstra's O(m log n) for sparse graphs)
- **Space**: O(m + n)

## When to Use BMSSP

BMSSP is beneficial when:
- Graphs are large (hundreds of thousands to millions of edges)
- Many SSSP calls are needed (different sources, scenarios, weight updates)
- Graphs are sparse (E ~ O(V) to E ~ O(V log V))

For small graphs or single SSSP calls, Dijkstra may be faster due to lower constant factors.

## References

The algorithm is based on research in efficient shortest path algorithms. Reference implementations are available in Go, Rust, and TypeScript.
