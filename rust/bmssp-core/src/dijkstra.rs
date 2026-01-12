use std::collections::BinaryHeap;
use std::cmp::Reverse;
use crate::csr::CsrGraph;
use crate::error::Result;
use num_traits::Float;

/// Run Dijkstra's algorithm to compute single-source shortest paths
///
/// # Arguments
///
/// * `graph` - CSR graph representation
/// * `weights` - Edge weights array (length = number of edges)
/// * `source` - Source vertex index
/// * `enabled` - Optional mask for enabled edges (None = all enabled)
///
/// # Returns
///
/// Vector of distances from source to each vertex (infinity if unreachable)
pub fn dijkstra_sssp<T>(
    graph: &CsrGraph,
    weights: &[T],
    source: usize,
    enabled: Option<&[bool]>,
) -> Result<Vec<T>>
where
    T: Float + Copy,
{
    let (dist, _) = dijkstra_sssp_with_preds(graph, weights, source, enabled)?;
    Ok(dist)
}

/// Run Dijkstra's algorithm with predecessor tracking
///
/// Returns (distances, predecessors) where predecessors[v] = u if u->v is on shortest path,
/// and predecessors[v] = usize::MAX if v is unreachable or v is the source.
pub fn dijkstra_sssp_with_preds<T>(
    graph: &CsrGraph,
    weights: &[T],
    source: usize,
    enabled: Option<&[bool]>,
) -> Result<(Vec<T>, Vec<usize>)>
where
    T: Float + Copy,
{
    let n = graph.num_vertices();
    let mut dist = vec![T::infinity(); n];
    let mut pred = vec![usize::MAX; n];
    dist[source] = T::zero();
    pred[source] = source; // Source's predecessor is itself

    // Priority queue: (distance, vertex)
    // Use Reverse for min-heap behavior
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((T::zero(), source)));

    while let Some(Reverse((d, u))) = heap.pop() {
        // Skip if we've found a better path to u
        if d > dist[u] {
            continue;
        }

        let (start, end) = graph.edge_range(u);
        for (eid, &v) in graph.neighbors(u).iter().enumerate() {
            let edge_idx = start + eid;

            // Check if edge is enabled
            if let Some(enabled_mask) = enabled {
                if !enabled_mask[edge_idx] {
                    continue;
                }
            }

            let w = weights[edge_idx];
            let new_dist = dist[u] + w;

            if new_dist < dist[v] {
                dist[v] = new_dist;
                pred[v] = u;
                heap.push(Reverse((new_dist, v)));
            }
        }
    }

    Ok((dist, pred))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csr::CsrGraph;

    #[test]
    fn test_dijkstra_simple() {
        // Graph: 0 -> 1 (weight 1.0)
        let indptr = vec![0, 1, 1];
        let indices = vec![1];
        let graph = CsrGraph::new(2, indptr, indices).unwrap();
        let weights = vec![1.0f32];
        let dist = dijkstra_sssp(&graph, &weights, 0, None).unwrap();
        assert_eq!(dist[0], 0.0);
        assert_eq!(dist[1], 1.0);
    }

    #[test]
    fn test_dijkstra_chain() {
        // Graph: 0 -> 1 -> 2 (weights 1.0, 2.0)
        let indptr = vec![0, 1, 2, 2];
        let indices = vec![1, 2];
        let graph = CsrGraph::new(3, indptr, indices).unwrap();
        let weights = vec![1.0f32, 2.0f32];
        let dist = dijkstra_sssp(&graph, &weights, 0, None).unwrap();
        assert_eq!(dist[0], 0.0);
        assert_eq!(dist[1], 1.0);
        assert_eq!(dist[2], 3.0);
    }

    #[test]
    fn test_dijkstra_disconnected() {
        // Graph: 0 -> 1, 2 (isolated)
        let indptr = vec![0, 1, 1, 1];
        let indices = vec![1];
        let graph = CsrGraph::new(3, indptr, indices).unwrap();
        let weights = vec![1.0f32];
        let dist = dijkstra_sssp(&graph, &weights, 0, None).unwrap();
        assert_eq!(dist[0], 0.0);
        assert_eq!(dist[1], 1.0);
        assert!(dist[2].is_infinite());
    }
}
