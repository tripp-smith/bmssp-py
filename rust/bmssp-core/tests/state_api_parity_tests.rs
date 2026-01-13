use bmssp_core::csr::CsrGraph;
use bmssp_core::{bmssp_sssp, bmssp_sssp_with_preds, bmssp_sssp_with_state, bmssp_sssp_with_preds_and_state, BmsspState};

#[test]
fn test_state_api_parity_simple() {
    let indptr = vec![0, 1, 1];
    let indices = vec![1];
    let graph = CsrGraph::new(2, indptr, indices).unwrap();
    let weights = vec![1.0f32];
    
    // Regular API
    let dist_regular = bmssp_sssp(&graph, &weights, 0, None).unwrap();
    
    // State API
    let mut state = BmsspState::new(2);
    let dist_state = bmssp_sssp_with_state(&mut state, &graph, &weights, 0, None).unwrap();
    
    assert_eq!(dist_regular.len(), dist_state.len());
    for i in 0..dist_regular.len() {
        if dist_regular[i].is_infinite() {
            assert!(dist_state[i].is_infinite(), "Mismatch at vertex {}: regular=inf, state={}", i, dist_state[i]);
        } else {
            assert!((dist_regular[i] - dist_state[i]).abs() < 1e-6,
                    "Mismatch at vertex {}: regular={}, state={}", i, dist_regular[i], dist_state[i]);
        }
    }
}

#[test]
fn test_state_api_parity_random() {
    // Create a small random graph
    let indptr = vec![0, 2, 4, 5, 5];
    let indices = vec![1, 2, 2, 3, 3];
    let graph = CsrGraph::new(4, indptr, indices).unwrap();
    let weights = vec![1.0f32, 2.0f32, 1.0f32, 3.0f32, 1.0f32];
    
    // Regular API
    let dist_regular = bmssp_sssp(&graph, &weights, 0, None).unwrap();
    
    // State API
    let mut state = BmsspState::new(4);
    let dist_state = bmssp_sssp_with_state(&mut state, &graph, &weights, 0, None).unwrap();
    
    assert_eq!(dist_regular.len(), dist_state.len());
    for i in 0..dist_regular.len() {
        if dist_regular[i].is_infinite() {
            assert!(dist_state[i].is_infinite(), "Mismatch at vertex {}: regular=inf, state={}", i, dist_state[i]);
        } else {
            assert!((dist_regular[i] - dist_state[i]).abs() < 1e-6, 
                    "Mismatch at vertex {}: regular={}, state={}", i, dist_regular[i], dist_state[i]);
        }
    }
}

#[test]
fn test_state_api_parity_with_preds() {
    let indptr = vec![0, 2, 3, 3];
    let indices = vec![1, 2, 2];
    let graph = CsrGraph::new(3, indptr, indices).unwrap();
    let weights = vec![1.0f32, 2.0f32, 1.0f32];
    
    // Regular API
    let (dist_regular, pred_regular) = bmssp_sssp_with_preds(&graph, &weights, 0, None).unwrap();
    
    // State API
    let mut state = BmsspState::new(3);
    let (dist_state, pred_state) = bmssp_sssp_with_preds_and_state(&mut state, &graph, &weights, 0, None).unwrap();
    
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
fn test_state_api_parity_with_enabled_mask() {
    // 0 -> 1 -> 2, but disable edge 0->1
    let indptr = vec![0, 1, 2, 2];
    let indices = vec![1, 2];
    let graph = CsrGraph::new(3, indptr, indices).unwrap();
    let weights = vec![1.0f32, 2.0f32];
    let enabled = vec![false, true];
    
    // Regular API
    let dist_regular = bmssp_sssp(&graph, &weights, 0, Some(&enabled)).unwrap();
    
    // State API
    let mut state = BmsspState::new(3);
    let dist_state = bmssp_sssp_with_state(&mut state, &graph, &weights, 0, Some(&enabled)).unwrap();
    
    assert_eq!(dist_regular.len(), dist_state.len());
    for i in 0..dist_regular.len() {
        if dist_regular[i].is_infinite() {
            assert!(dist_state[i].is_infinite(), "Mismatch at vertex {}: regular=inf, state={}", i, dist_state[i]);
        } else {
            assert!((dist_regular[i] - dist_state[i]).abs() < 1e-6,
                    "Mismatch at vertex {}: regular={}, state={}", i, dist_regular[i], dist_state[i]);
        }
    }
}

#[test]
fn test_state_api_parity_large_graphs() {
    // 5x5 grid graph
    let n = 25;
    let mut indptr = Vec::new();
    let mut indices = Vec::new();
    let mut weights = Vec::new();
    
    let mut edge_idx = 0;
    for row in 0..5 {
        for col in 0..5 {
            let u = row * 5 + col;
            indptr.push(edge_idx);
            
            // Right neighbor
            if col < 4 {
                indices.push(u + 1);
                weights.push(1.0f32);
                edge_idx += 1;
            }
            // Bottom neighbor
            if row < 4 {
                indices.push(u + 5);
                weights.push(1.0f32);
                edge_idx += 1;
            }
        }
    }
    indptr.push(edge_idx);
    
    let graph = CsrGraph::new(n, indptr, indices).unwrap();
    
    // Regular API
    let dist_regular = bmssp_sssp(&graph, &weights, 0, None).unwrap();
    
    // State API
    let mut state = BmsspState::new(n);
    let dist_state = bmssp_sssp_with_state(&mut state, &graph, &weights, 0, None).unwrap();
    
    assert_eq!(dist_regular.len(), dist_state.len());
    for i in 0..dist_regular.len() {
        assert!((dist_regular[i] - dist_state[i]).abs() < 1e-6,
                "Mismatch at vertex {}: regular={}, state={}", i, dist_regular[i], dist_state[i]);
    }
}

#[test]
fn test_state_api_parity_multiple_sources() {
    let indptr = vec![0, 2, 3, 3];
    let indices = vec![1, 2, 2];
    let graph = CsrGraph::new(3, indptr, indices).unwrap();
    let weights = vec![1.0f32, 2.0f32, 1.0f32];
    
    let mut state = BmsspState::new(3);
    
    // Test multiple sources
    for source in 0..3 {
        // Regular API
        let dist_regular = bmssp_sssp(&graph, &weights, source, None).unwrap();
        
        // State API
        let dist_state = bmssp_sssp_with_state(&mut state, &graph, &weights, source, None).unwrap();
        
        assert_eq!(dist_regular.len(), dist_state.len());
        for i in 0..dist_regular.len() {
            if dist_regular[i].is_infinite() {
                assert!(dist_state[i].is_infinite(), 
                        "Mismatch at source {} vertex {}: regular=inf, state={}", source, i, dist_state[i]);
            } else {
                assert!((dist_regular[i] - dist_state[i]).abs() < 1e-6,
                        "Mismatch at source {} vertex {}: regular={}, state={}", 
                        source, i, dist_regular[i], dist_state[i]);
            }
        }
    }
}

#[test]
fn test_state_api_parity_disconnected() {
    // Graph with disconnected component
    let indptr = vec![0, 1, 1, 2, 2];
    let indices = vec![1, 3];
    let graph = CsrGraph::new(4, indptr, indices).unwrap();
    let weights = vec![1.0f32, 1.0f32];
    
    // Regular API
    let dist_regular = bmssp_sssp(&graph, &weights, 0, None).unwrap();
    
    // State API
    let mut state = BmsspState::new(4);
    let dist_state = bmssp_sssp_with_state(&mut state, &graph, &weights, 0, None).unwrap();
    
    assert_eq!(dist_regular.len(), dist_state.len());
    for i in 0..dist_regular.len() {
        // Both should have infinity for unreachable vertices
        assert_eq!(dist_regular[i].is_infinite(), dist_state[i].is_infinite(),
                   "Mismatch at vertex {}: regular.is_infinite()={}, state.is_infinite()={}", 
                   i, dist_regular[i].is_infinite(), dist_state[i].is_infinite());
        if !dist_regular[i].is_infinite() {
            assert!((dist_regular[i] - dist_state[i]).abs() < 1e-6,
                    "Mismatch at vertex {}: regular={}, state={}", i, dist_regular[i], dist_state[i]);
        }
    }
}
