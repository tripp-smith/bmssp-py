use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use bmssp_core::block_heap::{BlockHeap, FastBlockHeap};

fn bench_push_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("push_operations");
    
    for size in [10, 100, 1000, 10000].iter() {
        // Benchmark BTreeSet-based BlockHeap
        group.bench_with_input(
            BenchmarkId::new("BlockHeap_BTreeSet", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut heap = BlockHeap::new();
                    for i in 0..size {
                        heap.push(i, (i as f32) * 1.5);
                    }
                    black_box(&heap);
                })
            },
        );
        
        // Benchmark BinaryHeap-based FastBlockHeap
        group.bench_with_input(
            BenchmarkId::new("FastBlockHeap_BinaryHeap", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut heap = FastBlockHeap::new();
                    for i in 0..size {
                        heap.push(i, (i as f32) * 1.5);
                    }
                    black_box(&heap);
                })
            },
        );
    }
    
    group.finish();
}

fn bench_decrease_key_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("decrease_key_operations");
    
    for size in [10, 100, 1000].iter() {
        // Benchmark BTreeSet-based BlockHeap
        group.bench_with_input(
            BenchmarkId::new("BlockHeap_BTreeSet", size),
            size,
            |b, &size| {
                let mut heap = BlockHeap::new();
                for i in 0..size {
                    heap.push(i, (i as f32) * 10.0);
                }
                b.iter(|| {
                    for i in 0..size {
                        heap.decrease_key(i, (i as f32) * 5.0);
                    }
                    black_box(&heap);
                })
            },
        );
        
        // Benchmark BinaryHeap-based FastBlockHeap
        group.bench_with_input(
            BenchmarkId::new("FastBlockHeap_BinaryHeap", size),
            size,
            |b, &size| {
                let mut heap = FastBlockHeap::new();
                for i in 0..size {
                    heap.push(i, (i as f32) * 10.0);
                }
                b.iter(|| {
                    for i in 0..size {
                        heap.decrease_key(i, (i as f32) * 5.0);
                    }
                    black_box(&heap);
                })
            },
        );
    }
    
    group.finish();
}

fn bench_pop_block_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("pop_block_operations");
    
    for (total_size, block_size) in [(100, 10), (1000, 50), (10000, 100)].iter() {
        let total = *total_size;
        let block = *block_size;
        // Benchmark BTreeSet-based BlockHeap
        group.bench_with_input(
            BenchmarkId::new("BlockHeap_BTreeSet", format!("{}_block{}", total, block)),
            &(total, block),
            |b, &(total_size, block_size)| {
                b.iter(|| {
                    let mut heap = BlockHeap::new();
                    for i in 0..total_size {
                        heap.push(i, (i as f32) * 0.1);
                    }
                    let mut count = 0;
                    while !heap.is_empty() && count < total_size {
                        let (block_result, _) = heap.pop_block(block_size);
                        count += block_result.len();
                        black_box(&block_result);
                    }
                })
            },
        );
        
        // Benchmark BinaryHeap-based FastBlockHeap
        group.bench_with_input(
            BenchmarkId::new("FastBlockHeap_BinaryHeap", format!("{}_block{}", total, block)),
            &(total, block),
            |b, &(total_size, block_size)| {
                b.iter(|| {
                    let mut heap = FastBlockHeap::new();
                    for i in 0..total_size {
                        heap.push(i, (i as f32) * 0.1);
                    }
                    let mut count = 0;
                    while !heap.is_empty() && count < total_size {
                        let (block_result, _) = heap.pop_block(block_size);
                        count += block_result.len();
                        black_box(&block_result);
                    }
                })
            },
        );
    }
    
    group.finish();
}

fn bench_mixed_workload(c: &mut Criterion) {
    let mut group = c.benchmark_group("mixed_workload");
    
    for size in [100, 1000, 10000].iter() {
        // Benchmark BTreeSet-based BlockHeap
        group.bench_with_input(
            BenchmarkId::new("BlockHeap_BTreeSet", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut heap = BlockHeap::new();
                    // Push initial entries
                    for i in 0..size {
                        heap.push(i, (i as f32) * 2.0);
                    }
                    // Mix of decrease-key and pop operations
                    for _ in 0..(size / 10) {
                        // Pop a small block
                        let (block, _) = heap.pop_block(5);
                        black_box(&block);
                        // Decrease keys for some entries
                        for i in 0..10 {
                            heap.decrease_key(i, (i as f32) * 1.0);
                        }
                    }
                    black_box(&heap);
                })
            },
        );
        
        // Benchmark BinaryHeap-based FastBlockHeap
        group.bench_with_input(
            BenchmarkId::new("FastBlockHeap_BinaryHeap", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut heap = FastBlockHeap::new();
                    // Push initial entries
                    for i in 0..size {
                        heap.push(i, (i as f32) * 2.0);
                    }
                    // Mix of decrease-key and pop operations
                    for _ in 0..(size / 10) {
                        // Pop a small block
                        let (block, _) = heap.pop_block(5);
                        black_box(&block);
                        // Decrease keys for some entries
                        for i in 0..10 {
                            heap.decrease_key(i, (i as f32) * 1.0);
                        }
                    }
                    black_box(&heap);
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_push_operations,
    bench_decrease_key_operations,
    bench_pop_block_operations,
    bench_mixed_workload
);
criterion_main!(benches);
