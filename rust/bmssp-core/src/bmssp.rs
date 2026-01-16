use crate::block_heap::FastBlockHeap;
use crate::csr::CsrGraph;
use crate::error::Result;
use crate::params::BmsspParams;
use num_traits::Float;

#[cfg(feature = "simd")]
use std::any::TypeId;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "simd")]
use wide::{f32x4, f64x2};

/// BMSSP algorithm implementation
///
/// This implements the BMSSP (Blocked Multi-Source Shortest Path) algorithm
/// using block-based processing. For correctness, we use a block-based
/// Dijkstra-like approach that processes vertices in blocks.

fn relax_edges<T>(
    graph: &CsrGraph,
    weights: &[T],
    enabled: Option<&[bool]>,
    u: usize,
    dist: &mut [T],
    pred: &mut [usize],
    heap: &mut FastBlockHeap<T>,
) where
    T: Float + Copy + 'static,
{
    if enabled.is_none() {
        #[cfg(feature = "simd")]
        if try_relax_edges_simd(graph, weights, u, dist, pred, |v, new_dist| {
            heap.push(v, new_dist);
        }) {
            return;
        }
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
            heap.push(v, new_dist);
        }
    }
}

#[cfg(feature = "simd")]
fn try_relax_edges_simd<T>(
    graph: &CsrGraph,
    weights: &[T],
    u: usize,
    dist: &mut [T],
    pred: &mut [usize],
    mut push: impl FnMut(usize, T),
) -> bool
where
    T: Float + Copy + 'static,
{
    if TypeId::of::<T>() == TypeId::of::<f32>() {
        // SAFETY: Verified that T is f32 for this branch.
        let weights_f32 = unsafe { &*(weights as *const [T] as *const [f32]) };
        let dist_f32 = unsafe { &mut *(dist as *mut [T] as *mut [f32]) };
        relax_edges_simd_f32(graph, weights_f32, u, dist_f32, pred, |v, new_dist| {
            push(v, T::from(new_dist).unwrap());
        });
        return true;
    }

    if TypeId::of::<T>() == TypeId::of::<f64>() {
        // SAFETY: Verified that T is f64 for this branch.
        let weights_f64 = unsafe { &*(weights as *const [T] as *const [f64]) };
        let dist_f64 = unsafe { &mut *(dist as *mut [T] as *mut [f64]) };
        relax_edges_simd_f64(graph, weights_f64, u, dist_f64, pred, |v, new_dist| {
            push(v, T::from(new_dist).unwrap());
        });
        return true;
    }

    false
}

#[cfg(feature = "simd")]
fn relax_edges_simd_f32(
    graph: &CsrGraph,
    weights: &[f32],
    u: usize,
    dist: &mut [f32],
    pred: &mut [usize],
    mut push: impl FnMut(usize, f32),
) {
    let neighbors = graph.neighbors(u);
    let (start, _end) = graph.edge_range(u);
    let mut idx = 0;
    let dist_u = dist[u];

    while idx + 4 <= neighbors.len() {
        let edge_idx = start + idx;
        let w = f32x4::new([
            weights[edge_idx],
            weights[edge_idx + 1],
            weights[edge_idx + 2],
            weights[edge_idx + 3],
        ]);
        let new_dist = w + f32x4::splat(dist_u);
        let new_vals = new_dist.to_array();

        for lane in 0..4 {
            let v = neighbors[idx + lane];
            let candidate = new_vals[lane];
            if candidate < dist[v] {
                dist[v] = candidate;
                pred[v] = u;
                push(v, candidate);
            }
        }

        idx += 4;
    }

    for (eid, &v) in neighbors[idx..].iter().enumerate() {
        let edge_idx = start + idx + eid;
        let w = weights[edge_idx];
        let new_dist = dist_u + w;
        if new_dist < dist[v] {
            dist[v] = new_dist;
            pred[v] = u;
            push(v, new_dist);
        }
    }
}

#[cfg(feature = "simd")]
fn relax_edges_simd_f64(
    graph: &CsrGraph,
    weights: &[f64],
    u: usize,
    dist: &mut [f64],
    pred: &mut [usize],
    mut push: impl FnMut(usize, f64),
) {
    let neighbors = graph.neighbors(u);
    let (start, _end) = graph.edge_range(u);
    let mut idx = 0;
    let dist_u = dist[u];

    while idx + 2 <= neighbors.len() {
        let edge_idx = start + idx;
        let w = f64x2::new([weights[edge_idx], weights[edge_idx + 1]]);
        let new_dist = w + f64x2::splat(dist_u);
        let new_vals = new_dist.to_array();

        for lane in 0..2 {
            let v = neighbors[idx + lane];
            let candidate = new_vals[lane];
            if candidate < dist[v] {
                dist[v] = candidate;
                pred[v] = u;
                push(v, candidate);
            }
        }

        idx += 2;
    }

    for (eid, &v) in neighbors[idx..].iter().enumerate() {
        let edge_idx = start + idx + eid;
        let w = weights[edge_idx];
        let new_dist = dist_u + w;
        if new_dist < dist[v] {
            dist[v] = new_dist;
            pred[v] = u;
            push(v, new_dist);
        }
    }
}

/// BMSSP algorithm for single-source shortest paths
pub fn bmssp_sssp<T>(
    graph: &CsrGraph,
    weights: &[T],
    source: usize,
    enabled: Option<&[bool]>,
) -> Result<Vec<T>>
where
    T: Float + Copy + Send + Sync + 'static,
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
    T: Float + Copy + Send + Sync + 'static,
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
        
        #[cfg(feature = "parallel")]
        {
            let dist_snapshot = &dist[..];
            let candidates = block
                .par_iter()
                .fold(Vec::new, |mut acc, (u, d)| {
                    if *d > dist_snapshot[*u] {
                        return acc;
                    }

                    let (start, _end) = graph.edge_range(*u);
                    for (eid, &v) in graph.neighbors(*u).iter().enumerate() {
                        let edge_idx = start + eid;

                        if let Some(enabled_mask) = enabled {
                            if !enabled_mask[edge_idx] {
                                continue;
                            }
                        }

                        let w = weights[edge_idx];
                        let new_dist = dist_snapshot[*u] + w;

                        if new_dist < dist_snapshot[v] {
                            acc.push((v, new_dist, *u));
                        }
                    }

                    acc
                })
                .reduce(Vec::new, |mut a: Vec<(usize, T, usize)>, mut b| {
                    a.append(&mut b);
                    a
                });

            for (v, new_dist, u) in candidates {
                if new_dist < dist[v] {
                    dist[v] = new_dist;
                    pred[v] = u;
                    heap.push(v, new_dist);
                }
            }
        }

        #[cfg(not(feature = "parallel"))]
        {
            // Process each vertex in the block
            for (u, d) in block {
                // Skip if we've found a better path
                if d > dist[u] {
                    continue;
                }

                relax_edges(graph, weights, enabled, u, &mut dist, &mut pred, &mut heap);
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
    T: Float + Copy + Send + Sync + 'static,
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
    T: Float + Copy + Send + Sync + 'static,
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
        
        #[cfg(feature = "parallel")]
        {
            let dist_snapshot = &dist[..];
            let candidates = block
                .par_iter()
                .fold(Vec::new, |mut acc, (u, d)| {
                    if *d > dist_snapshot[*u] {
                        return acc;
                    }

                    let (start, _end) = graph.edge_range(*u);
                    for (eid, &v) in graph.neighbors(*u).iter().enumerate() {
                        let edge_idx = start + eid;

                        if let Some(enabled_mask) = enabled {
                            if !enabled_mask[edge_idx] {
                                continue;
                            }
                        }

                        let w = weights[edge_idx];
                        let new_dist = dist_snapshot[*u] + w;

                        if new_dist < dist_snapshot[v] {
                            acc.push((v, new_dist, *u));
                        }
                    }

                    acc
                })
                .reduce(Vec::new, |mut a: Vec<(usize, T, usize)>, mut b| {
                    a.append(&mut b);
                    a
                });

            for (v, new_dist, u) in candidates {
                if new_dist < dist[v] {
                    dist[v] = new_dist;
                    pred[v] = u;
                    state.heap.push(v, new_dist);
                }
            }
        }

        #[cfg(not(feature = "parallel"))]
        {
            // Process each vertex in the block
            for (u, d) in block {
                // Skip if we've found a better path
                if d > dist[u] {
                    continue;
                }

                relax_edges(graph, weights, enabled, u, dist, pred, &mut state.heap);
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

    #[cfg(feature = "simd")]
    #[test]
    fn test_bmssp_simd_relaxation() {
        let indptr = vec![0, 4, 4, 4, 4, 4];
        let indices = vec![1, 2, 3, 4];
        let graph = CsrGraph::new(5, indptr, indices).unwrap();
        let weights = vec![1.0f32, 2.0f32, 3.0f32, 4.0f32];

        let dist = bmssp_sssp(&graph, &weights, 0, None).unwrap();
        assert_eq!(dist[0], 0.0);
        assert_eq!(dist[1], 1.0);
        assert_eq!(dist[2], 2.0);
        assert_eq!(dist[3], 3.0);
        assert_eq!(dist[4], 4.0);
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn test_bmssp_parallel_relaxation() {
        let indptr = vec![0, 2, 4, 6, 6];
        let indices = vec![1, 2, 2, 3, 3, 0];
        let graph = CsrGraph::new(4, indptr, indices).unwrap();
        let weights = vec![1.0f32, 5.0f32, 1.0f32, 1.0f32, 1.0f32, 10.0f32];

        let dist = bmssp_sssp(&graph, &weights, 0, None).unwrap();
        assert_eq!(dist[0], 0.0);
        assert_eq!(dist[1], 1.0);
        assert_eq!(dist[2], 2.0);
        assert_eq!(dist[3], 2.0);
    }
}
