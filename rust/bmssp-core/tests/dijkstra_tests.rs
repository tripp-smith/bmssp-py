use bmssp_core::csr::CsrGraph;
use bmssp_core::dijkstra::dijkstra_sssp;

#[test]
fn test_single_edge() {
    let indptr = vec![0, 1, 1];
    let indices = vec![1];
    let graph = CsrGraph::new(2, indptr, indices).unwrap();
    let weights = vec![1.0f32];
    let dist = dijkstra_sssp(&graph, &weights, 0, None).unwrap();
    assert_eq!(dist[0], 0.0);
    assert_eq!(dist[1], 1.0);
}

#[test]
fn test_chain() {
    // 0 -> 1 -> 2
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
fn test_small_grid() {
    // 2x2 grid:
    // 0 -> 1
    // |    |
    // v    v
    // 2 -> 3
    let indptr = vec![0, 2, 3, 4, 4];
    let indices = vec![1, 2, 3, 3];
    let graph = CsrGraph::new(4, indptr, indices).unwrap();
    let weights = vec![1.0f32, 1.0f32, 1.0f32, 1.0f32];
    let dist = dijkstra_sssp(&graph, &weights, 0, None).unwrap();
    assert_eq!(dist[0], 0.0);
    assert_eq!(dist[1], 1.0);
    assert_eq!(dist[2], 1.0);
    assert_eq!(dist[3], 2.0);
}

#[test]
fn test_disconnected() {
    // 0 -> 1, 2 (isolated)
    let indptr = vec![0, 1, 1, 1];
    let indices = vec![1];
    let graph = CsrGraph::new(3, indptr, indices).unwrap();
    let weights = vec![1.0f32];
    let dist = dijkstra_sssp(&graph, &weights, 0, None).unwrap();
    assert_eq!(dist[0], 0.0);
    assert_eq!(dist[1], 1.0);
    assert!(dist[2].is_infinite());
}

#[test]
fn test_enabled_mask() {
    // 0 -> 1 -> 2, but disable edge 0->1
    let indptr = vec![0, 1, 2, 2];
    let indices = vec![1, 2];
    let graph = CsrGraph::new(3, indptr, indices).unwrap();
    let weights = vec![1.0f32, 2.0f32];
    let enabled = vec![false, true];
    let dist = dijkstra_sssp(&graph, &weights, 0, Some(&enabled)).unwrap();
    assert_eq!(dist[0], 0.0);
    assert!(dist[1].is_infinite());
    assert!(dist[2].is_infinite());
}
