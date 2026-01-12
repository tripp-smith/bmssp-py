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
    assert_eq!(dist[2], 2.0); // Path: 0->1->2
    
    // Check predecessors
    assert_eq!(pred[0], 0); // Source
    assert_eq!(pred[1], 0);
    assert_eq!(pred[2], 1);
}
