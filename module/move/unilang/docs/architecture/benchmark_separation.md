# Benchmark Architecture - Separate Crate Design

## Overview

Performance benchmarks for unilang are maintained in a separate workspace crate (`unilang_benchmarks`) rather than the main `unilang` crate. This architectural decision improves dependency isolation and production build performance.

## Rationale

### Dependency Isolation

Benchmark tooling requires specialized dependencies that production users never need:

- `benchkit` - Benchmark framework with statistical analysis
- `clap`, `pico-args` - CLI argument parsing for benchmark runners
- `num_cpus` - CPU detection for parallel benchmarking
- `rand` - Random data generation for realistic test scenarios
- `sysinfo` - System information gathering

By separating benchmarks into their own crate, these dependencies:
- Never pollute production dependency trees
- Don't increase compilation time for end users
- Don't bloat production binaries
- Allow independent versioning

### Build Performance

Production builds of `unilang` are faster because:
- Fewer optional features to check
- Smaller dependency graph
- No benchmark code in compilation units
- Cleaner feature flag matrix

### Maintenance Benefits

Separation provides:
- Clear boundary between production and performance testing code
- Independent benchmark versioning and releases
- Easier benchmark infrastructure evolution
- Clearer API surface for production users

## Structure

```
wTools/module/move/
├── unilang/                    # Main crate (production)
│   ├── src/
│   ├── tests/                 # Functional tests only
│   └── Cargo.toml             # No benchmark dependencies
│
└── unilang_benchmarks/         # Benchmark crate
    ├── src/
    │   ├── benchmark_config.rs
    │   ├── benchmark_data_sizes.rs
    │   └── realistic_test_data.rs
    ├── benches/               # All performance benchmarks
    │   ├── throughput_benchmark.rs
    │   ├── simd_json_benchmark.rs
    │   └── ...
    ├── tests/                 # Benchmark validation tests
    └── Cargo.toml             # All benchmark dependencies
```

## Usage

### Running Benchmarks

```sh
# From workspace root
cargo bench -p unilang_benchmarks

# Specific benchmark
cargo bench -p unilang_benchmarks --bench throughput_benchmark

# With environment configuration
BENCHMARK_ENV=production cargo bench -p unilang_benchmarks
```

### Adding New Benchmarks

1. Add benchmark code to `unilang_benchmarks/benches/`
2. Use `unilang_benchmarks::prelude::*` for common imports
3. Follow benchkit conventions for measurement
4. Document benchmark purpose and methodology

### Benchmark Development

The `unilang_benchmarks` crate imports `unilang` with `features = ["full"]` to access all functionality for comprehensive performance testing.

## Migration Notes

- All benchmark code moved from `unilang/benches/` → `unilang_benchmarks/benches/`
- Benchmark modules moved from `unilang/src/` → `unilang_benchmarks/src/`
- Feature flags `benchmarks` and `advanced_benchmarks` removed from main crate
- Documentation updated to reference separate benchmark crate

## See Also

- `unilang_benchmarks/readme.md` - Benchmark crate documentation
- `docs/performance.md` - Performance characteristics guide
- `docs/optimization_guide.md` - Optimization techniques
