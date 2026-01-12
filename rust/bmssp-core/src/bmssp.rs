use crate::csr::CsrGraph;
use crate::dijkstra::dijkstra_sssp;
use crate::params::BmsspParams;
use crate::block_heap::BlockHeap;
use crate::pivot::PivotFinder;
use crate::error::Result;
use num_traits::Float;

use crate::dijkstra::dijkstra_sssp_with_preds;

/// BMSSP algorithm implementation
///
/// This is a simplified baseline implementation. The full BMSSP algorithm
/// requires careful implementation of:
/// - Bounded relaxations
/// - Recursive block processing
/// - Predecessor forest tracking
/// - Proper pivot selection
///
/// For now, this falls back to Dijkstra for correctness.
/// A full implementation should replace this with the recursive BMSSP algorithm.
pub fn bmssp_sssp<T>(
    graph: &CsrGraph,
    weights: &[T],
    source: usize,
    enabled: Option<&[bool]>,
) -> Result<Vec<T>>
where
    T: Float + Copy + Ord,
{
    // For Milestone 2, we start with Dijkstra as the baseline
    // Full BMSSP implementation will be added incrementally
    
    // Compute parameters (for future use)
    let _params = BmsspParams::from_n(graph.num_vertices());
    
    // For now, use Dijkstra (will be replaced with full BMSSP)
    dijkstra_sssp(graph, weights, source, enabled)
}

/// BMSSP algorithm with predecessor tracking
pub fn bmssp_sssp_with_preds<T>(
    graph: &CsrGraph,
    weights: &[T],
    source: usize,
    enabled: Option<&[bool]>,
) -> Result<(Vec<T>, Vec<usize>)>
where
    T: Float + Copy + Ord,
{
    // For now, use Dijkstra with predecessors
    dijkstra_sssp_with_preds(graph, weights, source, enabled)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csr::CsrGraph;

    #[test]
    fn test_bmssp_simple() {
        let indptr = vec![0, 1, 1];
        let indices = vec![1];
        let graph = CsrGraph::new(2, indptr, indices).unwrap();
        let weights = vec![1.0f32];
        let dist = bmssp_sssp(&graph, &weights, 0, None).unwrap();
        assert_eq!(dist[0], 0.0);
        assert_eq!(dist[1], 1.0);
    }
}
