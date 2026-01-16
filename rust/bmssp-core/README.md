# bmssp-core

## Benchmarking SIMD and parallel relaxations

Run the default scalar benchmarks:

```bash
cargo bench -p bmssp-core --bench dijkstra_vs_bmssp
```

Enable SIMD vectorization:

```bash
cargo bench -p bmssp-core --bench dijkstra_vs_bmssp --features simd
```

Enable Rayon parallel relaxation:

```bash
cargo bench -p bmssp-core --bench dijkstra_vs_bmssp --features parallel
```

Enable SIMD + parallel together:

```bash
cargo bench -p bmssp-core --bench dijkstra_vs_bmssp --features "simd parallel"
```
