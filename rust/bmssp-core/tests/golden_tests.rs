use bmssp_core::csr::CsrGraph;
use bmssp_core::{dijkstra_sssp, dijkstra_sssp_with_preds, bmssp_sssp, bmssp_sssp_with_preds};

/// Helper to compare distances with tolerance
fn distances_match<T: num_traits::Float>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if a[i].is_infinite() {
            if !b[i].is_infinite() {
                return false;
            }
        } else if b[i].is_infinite() {
            return false;
        } else {
            let diff = (a[i] - b[i]).abs();
            let rel_tol = diff / a[i].abs().max(T::one());
            if rel_tol > T::epsilon() * T::from(100.0).unwrap() {
                return false;
            }
        }
    }
    true
}

/// Generate a random directed graph
fn random_graph(n: usize, num_edges: usize, seed: u64) -> (CsrGraph, Vec<f32>) {
    use std::collections::HashSet;
    let mut rng = SimpleRng::new(seed);
    let mut edges = HashSet::new();
    
    // Generate random edges
    while edges.len() < num_edges {
        let u = (rng.next() as usize) % n;
        let v = (rng.next() as usize) % n;
        if u != v {
            edges.insert((u, v));
        }
    }
    
    // Build CSR
    let mut edge_list: Vec<(usize, usize, f32)> = edges
        .into_iter()
        .map(|(u, v)| (u, v, ((rng.next() as usize % 100) as f32 + 1.0)))
        .collect();
    edge_list.sort_by_key(|(u, _, _)| *u);
    
    let mut indptr = vec![0; n + 1];
    let mut indices = Vec::new();
    let mut weights = Vec::new();
    
    for (u, v, w) in edge_list {
        indices.push(v);
        weights.push(w);
        for i in (u + 1)..=n {
            indptr[i] += 1;
        }
    }
    
    let graph = CsrGraph::new(n, indptr, indices).unwrap();
    (graph, weights)
}

/// Simple RNG for testing
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        self.state
    }
}

#[test]
fn test_golden_small() {
    let n = 10;
    let num_edges = 20;
    let (graph, weights) = random_graph(n, num_edges, 42);
    
    let dist_dijkstra = dijkstra_sssp(&graph, &weights, 0, None).unwrap();
    let dist_bmssp = bmssp_sssp(&graph, &weights, 0, None).unwrap();
    
    assert!(distances_match(&dist_dijkstra, &dist_bmssp),
        "Distances don't match for small graph");
}

#[test]
fn test_golden_grid() {
    // Create a small grid graph
    let rows = 4;
    let cols = 4;
    let n = rows * cols;
    
    let mut indptr = vec![0; n + 1];
    let mut indices = Vec::new();
    let mut weights = Vec::new();
    
    for i in 0..rows {
        for j in 0..cols {
            let u = i * cols + j;
            if j < cols - 1 {
                let v = u + 1;
                indices.push(v);
                weights.push(1.0f32);
                indptr[u + 1] += 1;
            }
            if i < rows - 1 {
                let v = u + cols;
                indices.push(v);
                weights.push(1.0f32);
                indptr[u + 1] += 1;
            }
        }
    }
    
    // Build cumulative indptr
    for i in 1..=n {
        indptr[i] += indptr[i - 1];
    }
    
    let graph = CsrGraph::new(n, indptr, indices).unwrap();
    
    let dist_dijkstra = dijkstra_sssp(&graph, &weights, 0, None).unwrap();
    let dist_bmssp = bmssp_sssp(&graph, &weights, 0, None).unwrap();
    
    assert!(distances_match(&dist_dijkstra, &dist_bmssp),
        "Distances don't match for grid graph");
}

#[test]
fn test_parity_random_graphs() {
    // Test multiple random graphs of varying sizes
    let test_cases = vec![
        (10, 15, 1),
        (20, 30, 2),
        (50, 75, 3),
        (100, 150, 4),
        (200, 300, 5),
    ];
    
    for (n, num_edges, seed) in test_cases {
        let (graph, weights) = random_graph(n, num_edges, seed);
        let dist_dijkstra = dijkstra_sssp(&graph, &weights, 0, None).unwrap();
        let dist_bmssp = bmssp_sssp(&graph, &weights, 0, None).unwrap();
        
        assert!(distances_match(&dist_dijkstra, &dist_bmssp),
            "Distances don't match for random graph: n={}, edges={}", n, num_edges);
    }
}

#[test]
fn test_parity_multiple_sources() {
    let n = 20;
    let num_edges = 40;
    let (graph, weights) = random_graph(n, num_edges, 42);
    
    // Test multiple source vertices
    let sources = vec![0, 5, 10, 15];
    for source in sources {
        let dist_dijkstra = dijkstra_sssp(&graph, &weights, source, None).unwrap();
        let dist_bmssp = bmssp_sssp(&graph, &weights, source, None).unwrap();
        
        assert!(distances_match(&dist_dijkstra, &dist_bmssp),
            "Distances don't match for source {}", source);
    }
}

#[test]
fn test_parity_grid_graphs() {
    // Test multiple grid sizes
    let grid_sizes = vec![(2, 2), (3, 3), (4, 4), (5, 5)];
    
    for (rows, cols) in grid_sizes {
        let n = rows * cols;
        let mut indptr = vec![0; n + 1];
        let mut indices = Vec::new();
        let mut weights = Vec::new();
        
        for i in 0..rows {
            for j in 0..cols {
                let u = i * cols + j;
                if j < cols - 1 {
                    let v = u + 1;
                    indices.push(v);
                    weights.push(1.0f32);
                    indptr[u + 1] += 1;
                }
                if i < rows - 1 {
                    let v = u + cols;
                    indices.push(v);
                    weights.push(1.0f32);
                    indptr[u + 1] += 1;
                }
            }
        }
        
        for i in 1..=n {
            indptr[i] += indptr[i - 1];
        }
        
        let graph = CsrGraph::new(n, indptr, indices).unwrap();
        let dist_dijkstra = dijkstra_sssp(&graph, &weights, 0, None).unwrap();
        let dist_bmssp = bmssp_sssp(&graph, &weights, 0, None).unwrap();
        
        assert!(distances_match(&dist_dijkstra, &dist_bmssp),
            "Distances don't match for {}x{} grid", rows, cols);
    }
}

#[test]
fn test_parity_pipeline_like() {
    // Chain graph with sparse bypass links (pipeline-like structure)
    let n = 20;
    let mut indptr = vec![0; n + 1];
    let mut indices = Vec::new();
    let mut weights = Vec::new();
    
    // Main chain: 0 -> 1 -> 2 -> ... -> n-1
    for i in 0..(n - 1) {
        indices.push(i + 1);
        weights.push(1.0f32);
        indptr[i + 1] += 1;
    }
    
    // Add some bypass links: skip 2-3 vertices
    for i in 0..(n - 3) {
        indices.push(i + 3);
        weights.push(2.5f32); // Slightly more expensive than chain
        indptr[i + 1] += 1;
    }
    
    // Build cumulative indptr
    for i in 1..=n {
        indptr[i] += indptr[i - 1];
    }
    
    let graph = CsrGraph::new(n, indptr, indices).unwrap();
    let dist_dijkstra = dijkstra_sssp(&graph, &weights, 0, None).unwrap();
    let dist_bmssp = bmssp_sssp(&graph, &weights, 0, None).unwrap();
    
    assert!(distances_match(&dist_dijkstra, &dist_bmssp),
        "Distances don't match for pipeline-like graph");
}

#[test]
fn test_parity_with_enabled_mask() {
    let n = 20;
    let num_edges = 40;
    let (graph, weights) = random_graph(n, num_edges, 42);
    
    // Create enabled mask: disable 20% of edges
    let mut enabled = vec![true; weights.len()];
    for i in 0..weights.len() {
        if i % 5 == 0 {
            enabled[i] = false;
        }
    }
    
    let dist_dijkstra = dijkstra_sssp(&graph, &weights, 0, Some(&enabled)).unwrap();
    let dist_bmssp = bmssp_sssp(&graph, &weights, 0, Some(&enabled)).unwrap();
    
    assert!(distances_match(&dist_dijkstra, &dist_bmssp),
        "Distances don't match with enabled mask");
}

#[test]
fn test_parity_predecessors() {
    let n = 30;
    let num_edges = 60;
    let (graph, weights) = random_graph(n, num_edges, 42);
    
    let (dist_dijkstra, pred_dijkstra) = dijkstra_sssp_with_preds(&graph, &weights, 0, None).unwrap();
    let (dist_bmssp, pred_bmssp) = bmssp_sssp_with_preds(&graph, &weights, 0, None).unwrap();
    
    // Check distances match
    assert!(distances_match(&dist_dijkstra, &dist_bmssp),
        "Distances don't match with predecessors");
    
    // Check that predecessor paths reconstruct to correct distances
    for v in 0..n {
        if dist_dijkstra[v].is_infinite() {
            assert_eq!(pred_dijkstra[v], usize::MAX);
            assert_eq!(pred_bmssp[v], usize::MAX);
        } else if v == 0 {
            assert_eq!(pred_dijkstra[v], 0);
            assert_eq!(pred_bmssp[v], 0);
        } else {
            // Verify predecessor is valid
            assert!(pred_dijkstra[v] < n);
            assert!(pred_bmssp[v] < n);
            
            // Verify path cost matches distance (for BMSSP)
            let mut path_cost_bmssp = 0.0f32;
            let mut current = v;
            let mut visited = std::collections::HashSet::new();
            while current != 0 && pred_bmssp[current] != usize::MAX {
                if visited.contains(&current) {
                    break; // Cycle detected, skip
                }
                visited.insert(current);
                
                let prev = pred_bmssp[current];
                if prev == current {
                    break;
                }
                
                // Find edge weight
                let (start, _) = graph.edge_range(prev);
                if let Some(pos) = graph.neighbors(prev).iter().position(|&x| x == current) {
                    path_cost_bmssp += weights[start + pos];
                }
                current = prev;
            }
            
            // Allow some tolerance for floating point
            let diff = (path_cost_bmssp - dist_bmssp[v]).abs();
            assert!(diff < 1e-4, 
                "Path cost {} doesn't match distance {} for vertex {}", 
                path_cost_bmssp, dist_bmssp[v], v);
        }
    }
}

#[test]
fn test_parity_sparse_graph() {
    // Very sparse graph: E ≈ 2V
    let n = 50;
    let num_edges = 100;
    let (graph, weights) = random_graph(n, num_edges, 100);
    
    let dist_dijkstra = dijkstra_sssp(&graph, &weights, 0, None).unwrap();
    let dist_bmssp = bmssp_sssp(&graph, &weights, 0, None).unwrap();
    
    assert!(distances_match(&dist_dijkstra, &dist_bmssp),
        "Distances don't match for sparse graph");
}

#[test]
fn test_parity_dense_graph() {
    // Moderately dense graph: E ≈ 5V
    let n = 30;
    let num_edges = 150;
    let (graph, weights) = random_graph(n, num_edges, 200);
    
    let dist_dijkstra = dijkstra_sssp(&graph, &weights, 0, None).unwrap();
    let dist_bmssp = bmssp_sssp(&graph, &weights, 0, None).unwrap();
    
    assert!(distances_match(&dist_dijkstra, &dist_bmssp),
        "Distances don't match for dense graph");
}

#[test]
fn test_parity_weight_variations() {
    let n = 20;
    let num_edges = 40;
    let (graph, _) = random_graph(n, num_edges, 42);
    
    // Test with different weight distributions
    let weight_sets = vec![
        // Uniform weights
        (0..num_edges).map(|_| 1.0f32).collect::<Vec<_>>(),
        // Powers of 2
        (0..num_edges).map(|i| (2.0f32).powi((i % 5) as i32)).collect::<Vec<_>>(),
        // Small weights
        (0..num_edges).map(|i| 0.1f32 + (i % 10) as f32 * 0.01).collect::<Vec<_>>(),
        // Large weights
        (0..num_edges).map(|i| 100.0f32 + (i % 50) as f32).collect::<Vec<_>>(),
    ];
    
    for weights in weight_sets {
        let dist_dijkstra = dijkstra_sssp(&graph, &weights, 0, None).unwrap();
        let dist_bmssp = bmssp_sssp(&graph, &weights, 0, None).unwrap();
        
        assert!(distances_match(&dist_dijkstra, &dist_bmssp),
            "Distances don't match for weight variation");
    }
}
