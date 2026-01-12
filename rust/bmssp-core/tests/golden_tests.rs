use bmssp_core::csr::CsrGraph;
use bmssp_core::{dijkstra_sssp, bmssp_sssp};

/// Generate a random directed graph
fn random_graph(n: usize, num_edges: usize, seed: u64) -> (CsrGraph, Vec<f32>) {
    use std::collections::HashSet;
    let mut rng = SimpleRng::new(seed);
    let mut edges = HashSet::new();
    
    // Generate random edges
    while edges.len() < num_edges {
        let u = rng.next() % n;
        let v = rng.next() % n;
        if u != v {
            edges.insert((u, v));
        }
    }
    
    // Build CSR
    let mut edge_list: Vec<(usize, usize, f32)> = edges
        .into_iter()
        .map(|(u, v)| (u, v, (rng.next() % 100) as f32 + 1.0))
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
    
    // Compare distances (within floating point tolerance)
    for i in 0..n {
        if dist_dijkstra[i].is_infinite() {
            assert!(dist_bmssp[i].is_infinite(), "Vertex {}: dijkstra=inf, bmssp={}", i, dist_bmssp[i]);
        } else {
            let diff = (dist_dijkstra[i] - dist_bmssp[i]).abs();
            assert!(diff < 1e-5, "Vertex {}: dijkstra={}, bmssp={}, diff={}", i, dist_dijkstra[i], dist_bmssp[i], diff);
        }
    }
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
    
    for i in 0..n {
        let diff = (dist_dijkstra[i] - dist_bmssp[i]).abs();
        assert!(diff < 1e-5, "Vertex {}: dijkstra={}, bmssp={}", i, dist_dijkstra[i], dist_bmssp[i]);
    }
}
