use bmssp_core::block_heap::{BlockHeap, FastBlockHeap};

#[test]
fn test_heap_parity_simple() {
    let mut block_heap = BlockHeap::new();
    let mut fast_heap = FastBlockHeap::new();
    
    // Same operations
    block_heap.push(0, 1.0f32);
    block_heap.push(1, 2.0f32);
    
    fast_heap.push(0, 1.0f32);
    fast_heap.push(1, 2.0f32);
    
    // Compare results
    let (block_result, block_next) = block_heap.pop_block(10);
    let (fast_result, fast_next) = fast_heap.pop_block(10);
    
    assert_eq!(block_result.len(), fast_result.len());
    for i in 0..block_result.len() {
        assert_eq!(block_result[i].0, fast_result[i].0);
        assert!((block_result[i].1 - fast_result[i].1).abs() < 1e-6);
    }
    match (block_next, fast_next) {
        (None, None) => {},
        (Some(b), Some(f)) => assert!((b - f).abs() < 1e-6),
        _ => panic!("b_next mismatch"),
    }
}

#[test]
fn test_heap_parity_random_operations() {
    let mut block_heap = BlockHeap::new();
    let mut fast_heap = FastBlockHeap::new();
    
    // Random sequence of operations
    let operations = vec![
        (0, 5.0f32),
        (1, 3.0f32),
        (2, 7.0f32),
        (3, 1.0f32),
        (4, 9.0f32),
    ];
    
    for (v, d) in operations {
        block_heap.push(v, d);
        fast_heap.push(v, d);
    }
    
    // Extract in blocks
    let (block1, _) = block_heap.pop_block(2);
    let (fast1, _) = fast_heap.pop_block(2);
    
    assert_eq!(block1.len(), fast1.len());
    for i in 0..block1.len() {
        assert_eq!(block1[i].0, fast1[i].0);
        assert!((block1[i].1 - fast1[i].1).abs() < 1e-6);
    }
    
    // Extract remaining
    let (block2, _) = block_heap.pop_block(10);
    let (fast2, _) = fast_heap.pop_block(10);
    
    assert_eq!(block2.len(), fast2.len());
    for i in 0..block2.len() {
        assert_eq!(block2[i].0, fast2[i].0);
        assert!((block2[i].1 - fast2[i].1).abs() < 1e-6);
    }
}

#[test]
fn test_heap_parity_complex() {
    let mut block_heap = BlockHeap::new();
    let mut fast_heap = FastBlockHeap::new();
    
    // Complex scenario with many decrease-key operations
    block_heap.push(0, 10.0f32);
    block_heap.push(1, 20.0f32);
    block_heap.push(2, 30.0f32);
    block_heap.push(3, 40.0f32);
    
    fast_heap.push(0, 10.0f32);
    fast_heap.push(1, 20.0f32);
    fast_heap.push(2, 30.0f32);
    fast_heap.push(3, 40.0f32);
    
    // Decrease keys
    block_heap.decrease_key(0, 5.0f32);
    block_heap.decrease_key(2, 15.0f32);
    block_heap.decrease_key(0, 2.0f32);
    block_heap.decrease_key(1, 8.0f32);
    block_heap.decrease_key(2, 12.0f32);
    
    fast_heap.decrease_key(0, 5.0f32);
    fast_heap.decrease_key(2, 15.0f32);
    fast_heap.decrease_key(0, 2.0f32);
    fast_heap.decrease_key(1, 8.0f32);
    fast_heap.decrease_key(2, 12.0f32);
    
    // Extract all
    let (block_result, _) = block_heap.pop_block(10);
    let (fast_result, _) = fast_heap.pop_block(10);
    
    assert_eq!(block_result.len(), fast_result.len());
    for i in 0..block_result.len() {
        assert_eq!(block_result[i].0, fast_result[i].0);
        assert!((block_result[i].1 - fast_result[i].1).abs() < 1e-6);
    }
}

#[test]
fn test_heap_parity_block_sizes() {
    let mut block_heap = BlockHeap::new();
    let mut fast_heap = FastBlockHeap::new();
    
    // Add 20 vertices
    for i in 0..20 {
        let dist = (i as f32) * 0.5;
        block_heap.push(i, dist);
        fast_heap.push(i, dist);
    }
    
    // Test different block sizes
    for block_size in [1, 3, 5, 10, 20] {
        let mut block_heap_copy = BlockHeap::new();
        let mut fast_heap_copy = FastBlockHeap::new();
        
        for i in 0..20 {
            let dist = (i as f32) * 0.5;
            block_heap_copy.push(i, dist);
            fast_heap_copy.push(i, dist);
        }
        
        let (block_result, _) = block_heap_copy.pop_block(block_size);
        let (fast_result, _) = fast_heap_copy.pop_block(block_size);
        
        assert_eq!(block_result.len(), fast_result.len());
        for i in 0..block_result.len() {
            assert_eq!(block_result[i].0, fast_result[i].0);
            assert!((block_result[i].1 - fast_result[i].1).abs() < 1e-6);
        }
    }
}

#[test]
fn test_heap_parity_edge_cases() {
    // Empty heap
    let mut block_heap = BlockHeap::new();
    let mut fast_heap = FastBlockHeap::new();
    
    assert_eq!(block_heap.is_empty(), fast_heap.is_empty());
    
    let (block_result, block_next) = block_heap.pop_block(10);
    let (fast_result, fast_next) = fast_heap.pop_block(10);
    
    assert_eq!(block_result.len(), fast_result.len());
    assert_eq!(block_next, fast_next);
    
    // Single vertex
    block_heap.push(0, 1.0f32);
    fast_heap.push(0, 1.0f32);
    
    assert_eq!(block_heap.is_empty(), fast_heap.is_empty());
    assert_eq!(block_heap.min_distance(), fast_heap.min_distance());
    
    let (block_result, _) = block_heap.pop_block(1);
    let (fast_result, _) = fast_heap.pop_block(1);
    
    assert_eq!(block_result.len(), fast_result.len());
    assert_eq!(block_result[0].0, fast_result[0].0);
    assert!((block_result[0].1 - fast_result[0].1).abs() < 1e-6);
    
    assert_eq!(block_heap.is_empty(), fast_heap.is_empty());
}
