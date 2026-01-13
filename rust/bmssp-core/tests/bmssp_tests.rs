use bmssp_core::csr::CsrGraph;
use bmssp_core::{bmssp_sssp, bmssp_sssp_with_preds};

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
    
    // Check predecessors
    assert_eq!(pred[0], 0); // Source
    assert_eq!(pred[1], 0);
    // pred[2] can be either 0 (direct path) or 1 (path through 1) since both have cost 2.0
    assert!(pred[2] == 0 || pred[2] == 1);
}

#[test]
fn test_bmssp_chain() {
    // Chain: 0 -> 1 -> 2 -> 3
    let indptr = vec![0, 1, 2, 3, 3];
    let indices = vec![1, 2, 3];
    let graph = CsrGraph::new(4, indptr, indices).unwrap();
    let weights = vec![1.0f32, 2.0f32, 3.0f32];
    let dist = bmssp_sssp(&graph, &weights, 0, None).unwrap();
    
    assert_eq!(dist[0], 0.0);
    assert_eq!(dist[1], 1.0);
    assert_eq!(dist[2], 3.0);
    assert_eq!(dist[3], 6.0);
}

#[test]
fn test_bmssp_grid_2x2() {
    // 2x2 grid:
    // 0 -> 1
    // |    |
    // v    v
    // 2 -> 3
    let indptr = vec![0, 2, 3, 4, 4];
    let indices = vec![1, 2, 3, 3];
    let graph = CsrGraph::new(4, indptr, indices).unwrap();
    let weights = vec![1.0f32, 1.0f32, 1.0f32, 1.0f32];
    let dist = bmssp_sssp(&graph, &weights, 0, None).unwrap();
    
    assert_eq!(dist[0], 0.0);
    assert_eq!(dist[1], 1.0);
    assert_eq!(dist[2], 1.0);
    assert_eq!(dist[3], 2.0);
}

#[test]
fn test_bmssp_disconnected() {
    // 0 -> 1, 2 (isolated)
    let indptr = vec![0, 1, 1, 1];
    let indices = vec![1];
    let graph = CsrGraph::new(3, indptr, indices).unwrap();
    let weights = vec![1.0f32];
    let dist = bmssp_sssp(&graph, &weights, 0, None).unwrap();
    
    assert_eq!(dist[0], 0.0);
    assert_eq!(dist[1], 1.0);
    assert!(dist[2].is_infinite());
}

#[test]
fn test_bmssp_enabled_mask() {
    // 0 -> 1 -> 2, but disable edge 0->1
    let indptr = vec![0, 1, 2, 2];
    let indices = vec![1, 2];
    let graph = CsrGraph::new(3, indptr, indices).unwrap();
    let weights = vec![1.0f32, 2.0f32];
    let enabled = vec![false, true];
    let dist = bmssp_sssp(&graph, &weights, 0, Some(&enabled)).unwrap();
    
    assert_eq!(dist[0], 0.0);
    assert!(dist[1].is_infinite());
    assert!(dist[2].is_infinite());
}

#[test]
fn test_bmssp_cycle() {
    // Cycle: 0 -> 1 -> 2 -> 0
    let indptr = vec![0, 1, 2, 3];
    let indices = vec![1, 2, 0];
    let graph = CsrGraph::new(3, indptr, indices).unwrap();
    let weights = vec![1.0f32, 1.0f32, 1.0f32];
    let dist = bmssp_sssp(&graph, &weights, 0, None).unwrap();
    
    assert_eq!(dist[0], 0.0);
    assert_eq!(dist[1], 1.0);
    assert_eq!(dist[2], 2.0);
}

#[test]
fn test_bmssp_multiple_paths() {
    // Two paths from 0 to 3: 0->1->3 (cost 3) and 0->2->3 (cost 4)
    let indptr = vec![0, 2, 3, 4, 4];
    let indices = vec![1, 2, 3, 3];
    let graph = CsrGraph::new(4, indptr, indices).unwrap();
    let weights = vec![1.0f32, 2.0f32, 2.0f32, 2.0f32];
    let dist = bmssp_sssp(&graph, &weights, 0, None).unwrap();
    
    assert_eq!(dist[0], 0.0);
    assert_eq!(dist[1], 1.0);
    assert_eq!(dist[2], 2.0);
    assert_eq!(dist[3], 3.0); // Takes path 0->1->3
}

#[test]
fn test_bmssp_predecessor_path_reconstruction() {
    // Graph with a path: 0 -> 1 -> 2 -> 3
    let indptr = vec![0, 1, 2, 3, 3];
    let indices = vec![1, 2, 3];
    let graph = CsrGraph::new(4, indptr, indices).unwrap();
    let weights = vec![1.0f32, 2.0f32, 3.0f32];
    let (dist, pred) = bmssp_sssp_with_preds(&graph, &weights, 0, None).unwrap();
    
    // Verify distances
    assert_eq!(dist[3], 6.0);
    
    // Reconstruct path manually
    let mut path = Vec::new();
    let mut current = 3;
    while current != 0 {
        path.push(current);
        current = pred[current];
        if current == usize::MAX {
            break;
        }
    }
    path.push(0);
    path.reverse();
    
    assert_eq!(path, vec![0, 1, 2, 3]);
    
    // Verify path cost matches distance
    let path_cost: f32 = path.windows(2).map(|w| {
        let u = w[0];
        let v = w[1];
        let (start, _) = graph.edge_range(u);
        let edge_idx = start + graph.neighbors(u).iter().position(|&x| x == v).unwrap();
        weights[edge_idx]
    }).sum();
    assert_eq!(path_cost, dist[3]);
}

#[test]
fn test_bmssp_predecessor_source() {
    let indptr = vec![0, 1, 1];
    let indices = vec![1];
    let graph = CsrGraph::new(2, indptr, indices).unwrap();
    let weights = vec![1.0f32];
    let (_, pred) = bmssp_sssp_with_preds(&graph, &weights, 0, None).unwrap();
    
    // Source's predecessor should be itself
    assert_eq!(pred[0], 0);
}

#[test]
fn test_bmssp_predecessor_unreachable() {
    // 0 -> 1, 2 (isolated)
    let indptr = vec![0, 1, 1, 1];
    let indices = vec![1];
    let graph = CsrGraph::new(3, indptr, indices).unwrap();
    let weights = vec![1.0f32];
    let (_, pred) = bmssp_sssp_with_preds(&graph, &weights, 0, None).unwrap();
    
    // Unreachable vertex should have MAX predecessor
    assert_eq!(pred[2], usize::MAX);
}
