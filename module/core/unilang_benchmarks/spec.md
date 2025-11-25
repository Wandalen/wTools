# unilang_benchmarks

Performance benchmarks for the unilang CLI framework.

## Overview

`unilang_benchmarks` is an internal benchmarking crate that measures performance characteristics of the unilang framework. It is separated from the main unilang crate to avoid polluting production dependencies with benchmarking tools.

**Note**: This crate is not published to crates.io (publish = false) and is for internal development use only.

### Scope

#### Responsibility

unilang_benchmarks is responsible for providing comprehensive performance benchmarks for unilang's core components including parsing, command dispatch, string interning, and SIMD optimizations.

#### In-Scope

- **Throughput benchmarks**: Measure command processing speed
- **String interning benchmarks**: Test string deduplication performance
- **SIMD benchmarks**: Validate SIMD optimization effectiveness
- **Parser benchmarks**: Measure argument parsing performance
- **Comparative benchmarks**: Compare against alternative approaches
- **Markdown reports**: Generate benchmark result reports

#### Out-of-Scope

- **Production use**: Benchmarking tools only
- **Public API**: Internal development tool
- **Runtime metrics**: Compile/benchmark-time only
- **Framework functionality**: No unilang features

#### Boundaries

- **Upstream**: Uses unilang, unilang_parser, benchkit
- **Downstream**: Used by developers optimizing unilang
- **Development only**: Not for production deployment

## Architecture

### Module Structure

```
unilang_benchmarks/
├── benches/
│   ├── throughput_benchmark.rs
│   ├── string_interning_benchmark.rs
│   ├── simd_json_benchmark.rs
│   ├── integrated_string_interning_benchmark.rs
│   ├── throughput_benchmark_original.rs
│   └── strs_tools_benchmark.rs
├── src/
│   └── lib.rs
├── Cargo.toml
├── readme.md
└── spec.md
```

### Benchmark Categories

```
┌─────────────────────────────────────────────────────────────┐
│                   Benchmark Categories                       │
├─────────────────────────────────────────────────────────────┤
│  Throughput        │ Command processing rate                │
│  String Interning  │ String deduplication efficiency        │
│  SIMD JSON         │ SIMD-accelerated JSON parsing          │
│  Parser            │ Argument parsing performance           │
│  strs_tools        │ String utility performance             │
└─────────────────────────────────────────────────────────────┘
```

## Public API

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench throughput_benchmark

# Run with specific features
cargo bench --features "full"

# Generate HTML report
cargo bench -- --save-baseline main
```

### Benchmark Names

- `throughput_benchmark` - Core command processing throughput
- `string_interning_benchmark` - String interning performance
- `simd_json_benchmark` - SIMD JSON parsing benchmarks
- `integrated_string_interning_benchmark` - Full integration string tests
- `throughput_benchmark_original` - Baseline throughput comparison
- `strs_tools_benchmark` - String tool performance

## Feature Flags

No public features - uses workspace-defined features.

## Dependencies and Consumers

### Dependencies

| Dependency | Purpose |
|------------|---------|
| `unilang` | Framework under test |
| `unilang_parser` | Parser under test (with SIMD) |
| `benchkit` | Benchmarking framework |
| `criterion` | Statistical benchmarking |
| `clap` | CLI parsing comparison |
| `pico-args` | Alternative parser comparison |
| `rand` | Random data generation |
| `sysinfo` | System information |
| `phf` | PHF map comparison |

### Consumers

- unilang developers optimizing performance
- CI/CD performance regression detection
- Documentation of performance characteristics

## Usage Patterns

### Development Benchmarking

```bash
# Baseline measurement
cargo bench --bench throughput_benchmark -- --save-baseline before

# Make changes, then compare
cargo bench --bench throughput_benchmark -- --baseline before

# View detailed results
open target/criterion/report/index.html
```

### CI Performance Testing

```bash
# Run benchmarks in CI
cargo bench --bench throughput_benchmark -- --noplot

# Check for regressions
cargo bench --bench throughput_benchmark -- --baseline main
```

### SIMD Optimization Testing

```bash
# Test with SIMD enabled
cargo bench --bench simd_json_benchmark --features "simd"

# Compare without SIMD
cargo bench --bench simd_json_benchmark
```

## Design Rationale

### Why Separate Crate?

Benchmark dependencies are heavy and not needed in production:
1. `criterion` - Statistical analysis
2. `rand` - Random data generation
3. `sysinfo` - System profiling

Separating keeps unilang's dependency tree clean.

### Why Multiple Benchmarks?

Different aspects require different measurement approaches:
- Throughput: Requests per second
- Latency: Time per operation
- Memory: Allocation patterns
- Comparison: Against alternatives

### Why Not Published?

Internal tool for development:
1. Specific to unilang internals
2. Contains comparison code (clap, pico-args)
3. Results vary by hardware

## Testing Strategy

### Benchmark Validation

1. **Reproducibility**: Same inputs produce similar results
2. **Baseline comparison**: Track against known baselines
3. **Statistical significance**: Use criterion's statistical tests

### Running Tests

```bash
# Verify benchmarks compile
cargo build --benches

# Run with minimal iterations
cargo bench -- --quick
```

## Future Considerations

### Potential Enhancements

1. **Memory benchmarks**: Track allocation patterns
2. **Flamegraph integration**: CPU profiling
3. **Automated regression detection**: CI integration
4. **Cross-platform comparison**: Different architectures

### Known Limitations

1. **Hardware dependent**: Results vary by machine
2. **No automated regression**: Manual comparison needed
3. **Development only**: Not for production metrics

## Adoption Guidelines

### When to Use

- Optimizing unilang performance
- Validating SIMD optimizations
- Comparing parser implementations
- Detecting performance regressions

### When Not to Use

- Production monitoring
- Runtime metrics collection
- User-facing benchmarks

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `unilang` | Framework under test |
| `unilang_parser` | Parser under test |
| `benchkit` | Benchmarking framework |
| `criterion` | Statistical benchmarking |

## References

- [Criterion User Guide](https://bheisler.github.io/criterion.rs/book/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
