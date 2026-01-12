use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bmssp_core::csr::CsrGraph;
use bmssp_core::{dijkstra_sssp, bmssp_sssp};

fn generate_random_graph(n: usize, num_edges: usize, seed: u64) -> (CsrGraph, Vec<f32>) {
    use std::collections::HashSet;
    let mut rng = SimpleRng::new(seed);
    let mut edges = HashSet::new();
    
    while edges.len() < num_edges {
        let u = rng.next() % n;
        let v = rng.next() % n;
        if u != v {
            edges.insert((u, v));
        }
    }
    
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

fn bench_dijkstra(c: &mut Criterion) {
    let (graph, weights) = generate_random_graph(1000, 5000, 42);
    
    c.bench_function("dijkstra_1000v_5000e", |b| {
        b.iter(|| {
            dijkstra_sssp(black_box(&graph), black_box(&weights), black_box(0), black_box(None))
        })
    });
}

fn bench_bmssp(c: &mut Criterion) {
    let (graph, weights) = generate_random_graph(1000, 5000, 42);
    
    c.bench_function("bmssp_1000v_5000e", |b| {
        b.iter(|| {
            bmssp_sssp(black_box(&graph), black_box(&weights), black_box(0), black_box(None))
        })
    });
}

criterion_group!(benches, bench_dijkstra, bench_bmssp);
criterion_main!(benches);
