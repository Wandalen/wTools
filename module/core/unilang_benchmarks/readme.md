# unilang_benchmarks

Performance benchmarks for the unilang framework.

## Overview

This is a dedicated workspace crate for benchmarking the `unilang` framework. It's separated from the main `unilang` crate to keep production dependencies clean and avoid polluting the dependency tree with benchmark-specific dependencies like `criterion`, `benchkit`, `clap`, etc.

## Running Benchmarks

From the workspace root:

```sh
# Run all benchmarks
cargo bench -p unilang_benchmarks

# Run specific benchmark
cargo bench -p unilang_benchmarks --bench throughput_benchmark

# Run with environment configuration
BENCHMARK_ENV=production cargo bench -p unilang_benchmarks
```

## Available Benchmarks

### Core Framework Benchmarks

- **throughput_benchmark** - Overall framework throughput and end-to-end performance
- **throughput_benchmark_original** - Original baseline benchmarks for comparison

### Component Benchmarks

- **string_interning_benchmark** - String interning system performance
- **integrated_string_interning_benchmark** - Integrated interning with command lookups
- **simd_json_benchmark** - SIMD-accelerated JSON parsing performance
- **strs_tools_benchmark** - String manipulation utilities performance

## Benchmark Configuration

Benchmarks adapt to the execution environment via the `BENCHMARK_ENV` variable:

| Environment  | CV Tolerance | Sample Size | Use Case                  |
|--------------|--------------|-------------|---------------------------|
| development  | 15%          | 20-50       | Fast feedback during dev  |
| ci/staging   | 10%          | 50-100      | Regression detection      |
| production   | 5%           | 100-200     | High-accuracy analysis    |

### Examples

```sh
# Fast feedback (default)
cargo bench -p unilang_benchmarks

# CI/CD regression detection
BENCHMARK_ENV=ci cargo bench -p unilang_benchmarks

# Production-grade analysis
BENCHMARK_ENV=production cargo bench -p unilang_benchmarks --bench throughput_benchmark
```

## Benchmark Modules

### benchmark_config

Environment-aware configuration that adapts coefficient of variation (CV) requirements, sample counts, and performance thresholds based on execution environment.

### benchmark_data_sizes

Standard data size categories for consistent performance comparison:

- **Small** (10) - Quick operations, edge cases
- **Medium** (100) - Typical usage scenarios
- **Large** (1,000) - Stress testing, scaling analysis
- **Huge** (10,000) - Performance bottleneck detection

## Design Principles

1. **Separation of Concerns** - Benchmarks live in dedicated crate, not mixed with tests
2. **No Production Impact** - Benchmark dependencies don't pollute main crate
3. **Consistent Methodology** - Standard data sizes and statistical rigor via `benchkit`
4. **Environment Awareness** - Adapt to dev/CI/production contexts automatically

## Integration with Main Crate

This crate imports `unilang` and `unilang_parser` with all features enabled to comprehensively benchmark all code paths. The main crate remains clean and focused on production use.

## Related Documentation

- Main unilang crate: `../unilang/readme.md`
- Development rules: `../unilang/development_rules.md`
- Optimization guide: `../unilang/docs/optimization_guide.md`
