# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-XX

### Added

- Core BMSSP algorithm implementation with Rust backend
- Python API with clean NumPy integration
- Graph construction from edge lists and CSR format
- Single-source shortest path (SSSP) computation
- Support for both f32 and f64 precision
- Predecessor tracking for path reconstruction
- Scenario utilities for grid and pipeline networks:
  - Edge attributes modeling (base cost, capacity, risk)
  - Weight model for congestion-based cost updates
  - Outage simulation with enabled edge masks
- Path reconstruction helper functions
- Multi-sink cost extraction utilities
- Comprehensive test suite (Rust and Python)
- Performance benchmarks
- Documentation:
  - API reference
  - Tutorial with examples
  - Performance guide
  - Algorithm description
  - Installation guide
- CI/CD workflows for testing and wheel building
- Release process documentation

### Changed

- None (initial release)

### Deprecated

- None

### Removed

- None

### Fixed

- None

### Security

- None

[0.1.0]: https://github.com/tripp-smith/bmssp-py/releases/tag/v0.1.0
