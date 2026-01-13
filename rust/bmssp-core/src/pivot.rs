use crate::csr::CsrGraph;
use crate::params::BmsspParams;
use num_traits::Float;
use std::collections::VecDeque;

/// Find pivot vertices for BMSSP algorithm
///
/// Performs bounded relaxations to build a candidate set W,
/// tracks predecessor forest for tree sizes, and selects
/// pivot set P (sources that have large enough trees).
pub struct PivotFinder;

impl PivotFinder {
    /// Find pivots using bounded relaxations and predecessor forest tracking
    ///
    /// Performs bounded relaxations from candidate vertices up to distance `bound`,
    /// builds a predecessor forest, computes tree sizes, and selects pivots
    /// with tree sizes >= threshold (based on parameter `t`).
    ///
    /// Returns (pivot_set, candidate_set)
    pub fn find_pivots<T>(
        graph: &CsrGraph,
        weights: &[T],
        distances: &[T],
        enabled: Option<&[bool]>,
        bound: T,
        params: &BmsspParams,
    ) -> (Vec<usize>, Vec<usize>)
    where
        T: Float + Copy,
    {
        let n = graph.num_vertices();
        let mut candidates = Vec::new();
        
        // Step 1: Build candidate set W from vertices within bound
        for u in 0..n {
            if distances[u].is_finite() && distances[u] <= bound {
                candidates.push(u);
            }
        }
        
        if candidates.is_empty() {
            return (Vec::new(), Vec::new());
        }
        
        // Step 2: Perform bounded relaxations to build predecessor forest
        let mut pred_forest = vec![usize::MAX; n];
        let mut bounded_dist = distances.to_vec();
        
        // BFS-like bounded relaxation from candidates
        let mut queue = VecDeque::new();
        for &u in &candidates {
            queue.push_back(u);
            pred_forest[u] = u; // Self-loop for roots
        }
        
        while let Some(u) = queue.pop_front() {
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
                let new_dist = bounded_dist[u] + w;
                
                // Only relax if within bound
                if new_dist <= bound && new_dist < bounded_dist[v] {
                    bounded_dist[v] = new_dist;
                    pred_forest[v] = u;
                    queue.push_back(v);
                }
            }
        }
        
        // Step 3: Compute tree sizes using DFS
        let mut tree_sizes = vec![1; n]; // Each vertex counts itself
        
        // Mark which vertices are in the forest
        let mut in_forest = vec![false; n];
        for &u in &candidates {
            in_forest[u] = true;
        }
        for v in 0..n {
            if pred_forest[v] != usize::MAX && pred_forest[v] != v {
                in_forest[v] = true;
            }
        }
        
        // Compute tree sizes bottom-up
        // Sort vertices by distance (descending) to process leaves first
        let mut vertices_by_dist: Vec<(usize, T)> = (0..n)
            .filter(|&v| in_forest[v])
            .map(|v| (v, bounded_dist[v]))
            .collect();
        vertices_by_dist.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        for (v, _) in vertices_by_dist {
            if pred_forest[v] != usize::MAX && pred_forest[v] != v {
                tree_sizes[pred_forest[v]] += tree_sizes[v];
            }
        }
        
        // Step 4: Select pivots based on tree size threshold
        let threshold = params.t.max(2);
        let mut pivots = Vec::new();
        
        for &u in &candidates {
            // Only consider vertices that are roots (self-loop in pred_forest)
            // or have large enough tree sizes
            if pred_forest[u] == u && tree_sizes[u] >= threshold {
                pivots.push(u);
            }
        }
        
        (pivots, candidates)
    }
}
