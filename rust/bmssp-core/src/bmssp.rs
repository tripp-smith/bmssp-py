use crate::csr::CsrGraph;
use crate::params::BmsspParams;
use crate::block_heap::FastBlockHeap;
use crate::error::Result;
use num_traits::Float;

/// BMSSP algorithm implementation
///
/// This implements the BMSSP (Blocked Multi-Source Shortest Path) algorithm
/// using block-based processing. For correctness, we use a block-based
/// Dijkstra-like approach that processes vertices in blocks.

/// BMSSP algorithm for single-source shortest paths
pub fn bmssp_sssp<T>(
    graph: &CsrGraph,
    weights: &[T],
    source: usize,
    enabled: Option<&[bool]>,
) -> Result<Vec<T>>
where
    T: Float + Copy,
{
    let (dist, _) = bmssp_sssp_with_preds(graph, weights, source, enabled)?;
    Ok(dist)
}

/// BMSSP algorithm with predecessor tracking
///
/// Implements a block-based shortest path algorithm that processes vertices
/// in blocks rather than one at a time. This is simpler than full recursive
/// BMSSP but maintains the block-processing structure.
pub fn bmssp_sssp_with_preds<T>(
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
    pred[source] = source;
    
    // For very small graphs, use simple edge relaxation
    if n <= 4 {
        let mut changed = true;
        for _ in 0..n {
            if !changed {
                break;
            }
            changed = false;
            for u in 0..n {
                if !dist[u].is_finite() {
                    continue;
                }
                let (start, end) = graph.edge_range(u);
                for (eid, &v) in graph.neighbors(u).iter().enumerate() {
                    let edge_idx = start + eid;
                    
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
                        changed = true;
                    }
                }
            }
        }
        return Ok((dist, pred));
    }
    
    // Compute parameters for block processing
    let params = BmsspParams::from_n(n);
    
    // Initialize block heap with source
    let mut heap = FastBlockHeap::new();
    heap.push(source, T::zero());
    
    // Main loop: process blocks
    while !heap.is_empty() {
        // Extract a block of up to k vertices
        let (block, _b_next) = heap.pop_block(params.k);
        
        // Process each vertex in the block
        for (u, d) in block {
            // Skip if we've found a better path
            if d > dist[u] {
                continue;
            }
            
            // Relax edges from u
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
                    heap.push(v, new_dist);
                }
            }
        }
    }
    
    Ok((dist, pred))
}

/// Reusable state for BMSSP algorithm
///
/// This structure holds buffers that can be reused across multiple SSSP calls,
/// reducing allocations for performance-critical scenarios with repeated calls.
pub struct BmsspState<T> {
    /// Distance array (reusable buffer)
    distances: Vec<T>,
    /// Predecessor array (reusable buffer)
    predecessors: Vec<usize>,
    /// Block heap (reusable)
    heap: FastBlockHeap<T>,
}

impl<T> BmsspState<T>
where
    T: Float + Copy,
{
    /// Create a new state structure sized for graphs with up to `n` vertices
    pub fn new(n: usize) -> Self {
        Self {
            distances: vec![T::infinity(); n],
            predecessors: vec![usize::MAX; n],
            heap: FastBlockHeap::new(),
        }
    }

    /// Reset the state for a new SSSP computation
    ///
    /// This clears the heap and resets distances/predecessors arrays.
    /// The arrays are resized if needed to accommodate a graph with `n` vertices.
    pub fn reset(&mut self, n: usize) {
        // Resize arrays if needed
        if self.distances.len() < n {
            self.distances.resize(n, T::infinity());
            self.predecessors.resize(n, usize::MAX);
        } else {
            // Reset existing arrays
            self.distances[..n].fill(T::infinity());
            self.predecessors[..n].fill(usize::MAX);
        }
        // Clear heap
        self.heap = FastBlockHeap::new();
    }

    /// Get a reference to the distances array
    pub fn distances(&self) -> &[T] {
        &self.distances
    }

    /// Get a reference to the predecessors array
    pub fn predecessors(&self) -> &[usize] {
        &self.predecessors
    }
}

/// BMSSP algorithm using reusable state
///
/// This version uses a pre-allocated `BmsspState` to avoid allocations
/// between calls, improving performance for repeated SSSP computations.
pub fn bmssp_sssp_with_state<'a, T>(
    state: &'a mut BmsspState<T>,
    graph: &CsrGraph,
    weights: &[T],
    source: usize,
    enabled: Option<&[bool]>,
) -> Result<&'a [T]>
where
    T: Float + Copy,
{
    let (dist, _) = bmssp_sssp_with_preds_and_state(state, graph, weights, source, enabled)?;
    Ok(dist)
}

/// BMSSP algorithm with predecessor tracking using reusable state
///
/// This version uses a pre-allocated `BmsspState` to avoid allocations
/// between calls, improving performance for repeated SSSP computations.
pub fn bmssp_sssp_with_preds_and_state<'a, T>(
    state: &'a mut BmsspState<T>,
    graph: &CsrGraph,
    weights: &[T],
    source: usize,
    enabled: Option<&[bool]>,
) -> Result<(&'a [T], &'a [usize])>
where
    T: Float + Copy,
{
    let n = graph.num_vertices();
    state.reset(n);
    
    let dist = &mut state.distances[..n];
    let pred = &mut state.predecessors[..n];
    
    dist[source] = T::zero();
    pred[source] = source;
    
    // For very small graphs, use simple edge relaxation
    if n <= 4 {
        let mut changed = true;
        for _ in 0..n {
            if !changed {
                break;
            }
            changed = false;
            for u in 0..n {
                if !dist[u].is_finite() {
                    continue;
                }
                let (start, _end) = graph.edge_range(u);
                for (eid, &v) in graph.neighbors(u).iter().enumerate() {
                    let edge_idx = start + eid;
                    
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
                        changed = true;
                    }
                }
            }
        }
        return Ok((dist, pred));
    }
    
    // Compute parameters for block processing
    let params = BmsspParams::from_n(n);
    
    // Initialize block heap with source
    state.heap.push(source, T::zero());
    
    // Main loop: process blocks
    while !state.heap.is_empty() {
        // Extract a block of up to k vertices
        let (block, _b_next) = state.heap.pop_block(params.k);
        
        // Process each vertex in the block
        for (u, d) in block {
            // Skip if we've found a better path
            if d > dist[u] {
                continue;
            }
            
            // Relax edges from u
            let (start, _end) = graph.edge_range(u);
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
                    state.heap.push(v, new_dist);
                }
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
    fn test_bmssp_simple() {
        let indptr = vec![0, 1, 1];
        let indices = vec![1];
        let graph = CsrGraph::new(2, indptr, indices).unwrap();
        let weights = vec![1.0f32];
        let dist = bmssp_sssp(&graph, &weights, 0, None).unwrap();
        assert_eq!(dist[0], 0.0);
        assert_eq!(dist[1], 1.0);
    }
    
    #[test]
    fn test_bmssp_with_preds() {
        let indptr = vec![0, 2, 3, 3];
        let indices = vec![1, 2, 2];
        let graph = CsrGraph::new(3, indptr, indices).unwrap();
        let weights = vec![1.0f32, 2.0f32, 1.0f32];
        let (dist, pred) = bmssp_sssp_with_preds(&graph, &weights, 0, None).unwrap();
        
        assert_eq!(dist[0], 0.0);
        assert_eq!(dist[1], 1.0);
        assert_eq!(dist[2], 2.0); // Two paths with same cost: 0->2 (direct) or 0->1->2
        
        assert_eq!(pred[0], 0);
        assert_eq!(pred[1], 0);
        // pred[2] can be either 0 (direct path) or 1 (path through 1) since both have cost 2.0
        assert!(pred[2] == 0 || pred[2] == 1);
    }
    
    #[test]
    fn test_bmssp_state() {
        let indptr = vec![0, 1, 1];
        let indices = vec![1];
        let graph = CsrGraph::new(2, indptr, indices).unwrap();
        let weights = vec![1.0f32];
        
        let mut state = BmsspState::new(2);
        let dist = bmssp_sssp_with_state(&mut state, &graph, &weights, 0, None).unwrap();
        
        assert_eq!(dist[0], 0.0);
        assert_eq!(dist[1], 1.0);
    }
    
    #[test]
    fn test_bmssp_state_with_preds() {
        let indptr = vec![0, 2, 3, 3];
        let indices = vec![1, 2, 2];
        let graph = CsrGraph::new(3, indptr, indices).unwrap();
        let weights = vec![1.0f32, 2.0f32, 1.0f32];
        
        let mut state = BmsspState::new(3);
        let (dist, pred) = bmssp_sssp_with_preds_and_state(&mut state, &graph, &weights, 0, None).unwrap();
        
        assert_eq!(dist[0], 0.0);
        assert_eq!(dist[1], 1.0);
        assert_eq!(dist[2], 2.0);
        
        assert_eq!(pred[0], 0);
        assert_eq!(pred[1], 0);
        assert!(pred[2] == 0 || pred[2] == 1);
    }
    
    #[test]
    fn test_bmssp_state_reuse() {
        let indptr = vec![0, 1, 1];
        let indices = vec![1];
        let graph = CsrGraph::new(2, indptr, indices).unwrap();
        let weights = vec![1.0f32];
        
        let mut state = BmsspState::new(2);
        
        // First call
        let dist1 = bmssp_sssp_with_state(&mut state, &graph, &weights, 0, None).unwrap();
        assert_eq!(dist1[1], 1.0);
        
        // Second call (reusing state)
        let dist2 = bmssp_sssp_with_state(&mut state, &graph, &weights, 0, None).unwrap();
        assert_eq!(dist2[1], 1.0);
    }
    
    #[test]
    fn test_bmssp_state_reset() {
        let indptr = vec![0, 1, 1];
        let indices = vec![1];
        let graph = CsrGraph::new(2, indptr, indices).unwrap();
        let weights = vec![1.0f32];
        
        let mut state = BmsspState::new(2);
        
        // First computation
        let dist1 = bmssp_sssp_with_state(&mut state, &graph, &weights, 0, None).unwrap();
        assert_eq!(dist1[1], 1.0);
        
        // Manually reset and verify
        state.reset(2);
        assert_eq!(state.distances()[0], f32::INFINITY);
        assert_eq!(state.distances()[1], f32::INFINITY);
        assert!(state.heap.is_empty());
        
        // Second computation after reset
        let dist2 = bmssp_sssp_with_state(&mut state, &graph, &weights, 0, None).unwrap();
        assert_eq!(dist2[1], 1.0);
    }
    
    #[test]
    fn test_bmssp_state_resize() {
        // Start with small state
        let mut state = BmsspState::new(2);
        
        // Use with larger graph
        let indptr = vec![0, 1, 2, 3, 3];
        let indices = vec![1, 2, 3];
        let graph = CsrGraph::new(4, indptr, indices).unwrap();
        let weights = vec![1.0f32, 1.0f32, 1.0f32];
        
        // State should resize automatically
        let dist = bmssp_sssp_with_state(&mut state, &graph, &weights, 0, None).unwrap();
        assert_eq!(dist[0], 0.0);
        assert_eq!(dist[1], 1.0);
        assert_eq!(dist[2], 2.0);
        assert_eq!(dist[3], 3.0);
        
        // Verify state was resized
        assert!(state.distances().len() >= 4);
    }
    
    #[test]
    fn test_bmssp_state_multiple_sources() {
        let indptr = vec![0, 1, 2, 2];
        let indices = vec![1, 2];
        let graph = CsrGraph::new(3, indptr, indices).unwrap();
        let weights = vec![1.0f32, 1.0f32];
        
        let mut state = BmsspState::new(3);
        
        // Compute from source 0
        let dist1 = bmssp_sssp_with_state(&mut state, &graph, &weights, 0, None).unwrap();
        assert_eq!(dist1[1], 1.0);
        assert_eq!(dist1[2], 2.0);
        
        // Compute from source 1 (reusing state)
        let dist2 = bmssp_sssp_with_state(&mut state, &graph, &weights, 1, None).unwrap();
        assert_eq!(dist2[0], f32::INFINITY);  // Not reachable from 1
        assert_eq!(dist2[1], 0.0);
        assert_eq!(dist2[2], 1.0);
    }
    
    #[test]
    fn test_bmssp_state_with_enabled_mask() {
        // 0 -> 1 -> 2, but disable edge 0->1
        let indptr = vec![0, 1, 2, 2];
        let indices = vec![1, 2];
        let graph = CsrGraph::new(3, indptr, indices).unwrap();
        let weights = vec![1.0f32, 2.0f32];
        let enabled = vec![false, true];
        
        let mut state = BmsspState::new(3);
        let dist = bmssp_sssp_with_state(&mut state, &graph, &weights, 0, Some(&enabled)).unwrap();
        
        assert_eq!(dist[0], 0.0);
        assert!(dist[1].is_infinite());
        assert!(dist[2].is_infinite());
    }
    
    #[test]
    fn test_bmssp_state_vs_regular_api() {
        // Compare state API with regular API
        let indptr = vec![0, 2, 3, 3];
        let indices = vec![1, 2, 2];
        let graph = CsrGraph::new(3, indptr, indices).unwrap();
        let weights = vec![1.0f32, 2.0f32, 1.0f32];
        
        // Regular API
        let (dist_regular, pred_regular) = bmssp_sssp_with_preds(&graph, &weights, 0, None).unwrap();
        
        // State API
        let mut state = BmsspState::new(3);
        let (dist_state, pred_state) = bmssp_sssp_with_preds_and_state(&mut state, &graph, &weights, 0, None).unwrap();
        
        // Compare results
        assert_eq!(dist_regular.len(), dist_state.len());
        for i in 0..dist_regular.len() {
            assert!((dist_regular[i] - dist_state[i]).abs() < 1e-6);
        }
        
        assert_eq!(pred_regular.len(), pred_state.len());
        for i in 0..pred_regular.len() {
            assert_eq!(pred_regular[i], pred_state[i]);
        }
    }
    
    #[test]
    fn test_bmssp_state_lifetime() {
        // Test that state can be reused multiple times
        let indptr = vec![0, 1, 1];
        let indices = vec![1];
        let graph = CsrGraph::new(2, indptr, indices).unwrap();
        let weights = vec![1.0f32];
        
        let mut state = BmsspState::new(2);
        
        // Multiple calls
        for _ in 0..5 {
            let dist = bmssp_sssp_with_state(&mut state, &graph, &weights, 0, None).unwrap();
            assert_eq!(dist[1], 1.0);
        }
        
        // Verify state is still usable
        assert!(state.distances().len() >= 2);
    }
}
