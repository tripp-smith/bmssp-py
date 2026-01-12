use bmssp_core::csr::CsrGraph;

#[test]
fn test_graph_construction() {
    let indptr = vec![0, 1, 2];
    let indices = vec![1, 0];
    let graph = CsrGraph::new(2, indptr, indices).unwrap();
    assert_eq!(graph.num_vertices(), 2);
    assert_eq!(graph.num_edges(), 2);
}

#[test]
fn test_graph_neighbors() {
    let indptr = vec![0, 2, 3, 4];
    let indices = vec![1, 2, 0, 1];
    let graph = CsrGraph::new(3, indptr, indices).unwrap();
    
    assert_eq!(graph.neighbors(0), &[1, 2]);
    assert_eq!(graph.neighbors(1), &[0]);
    assert_eq!(graph.neighbors(2), &[1]);
}

#[test]
fn test_graph_edge_range() {
    let indptr = vec![0, 2, 3, 4];
    let indices = vec![1, 2, 0, 1];
    let graph = CsrGraph::new(3, indptr, indices).unwrap();
    
    assert_eq!(graph.edge_range(0), (0, 2));
    assert_eq!(graph.edge_range(1), (2, 3));
    assert_eq!(graph.edge_range(2), (3, 4));
}
