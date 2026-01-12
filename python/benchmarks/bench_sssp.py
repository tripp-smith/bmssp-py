"""Benchmarks for SSSP performance."""

import numpy as np
import pytest
from bmssp import Graph, sssp


def generate_random_graph(n: int, num_edges: int, seed: int = 42) -> tuple[Graph, np.ndarray]:
    """Generate a random directed graph."""
    np.random.seed(seed)
    edges_set = set()
    
    while len(edges_set) < num_edges:
        u = np.random.randint(0, n)
        v = np.random.randint(0, n)
        if u != v:
            edges_set.add((u, v))
    
    edges = np.array(list(edges_set), dtype=np.int64)
    weights = np.random.random(len(edges)).astype(np.float32) * 10.0 + 1.0
    
    graph, _ = Graph.from_edges(n, edges, weights=weights)
    return graph, weights


@pytest.mark.benchmark(group="sssp_small")
def test_bench_sssp_small(benchmark):
    """Benchmark SSSP on small graph (100 vertices, 500 edges)."""
    graph, weights = generate_random_graph(100, 500)
    benchmark(sssp, graph, weights, source=0)


@pytest.mark.benchmark(group="sssp_medium")
def test_bench_sssp_medium(benchmark):
    """Benchmark SSSP on medium graph (1000 vertices, 5000 edges)."""
    graph, weights = generate_random_graph(1000, 5000)
    benchmark(sssp, graph, weights, source=0)


@pytest.mark.benchmark(group="sssp_repeated")
def test_bench_sssp_repeated(benchmark):
    """Benchmark repeated SSSP calls (scenario simulation)."""
    graph, weights = generate_random_graph(500, 2500)
    
    def run_scenarios():
        for _ in range(100):
            # Simulate weight updates
            updated_weights = weights * (1.0 + np.random.random(len(weights)) * 0.1)
            sssp(graph, updated_weights, source=0)
    
    benchmark(run_scenarios)


@pytest.mark.benchmark(group="sssp_with_outage")
def test_bench_sssp_with_outage(benchmark):
    """Benchmark SSSP with enabled mask (outage simulation)."""
    graph, weights = generate_random_graph(500, 2500)
    
    # Create enabled mask (disable 10% of edges)
    enabled = np.ones(graph.num_edges(), dtype=bool)
    disable_indices = np.random.choice(graph.num_edges(), size=graph.num_edges() // 10, replace=False)
    enabled[disable_indices] = False
    
    benchmark(sssp, graph, weights, source=0, enabled=enabled)
