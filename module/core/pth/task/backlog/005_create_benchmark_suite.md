# Task 005: Create Benchmark Suite

## Overview

Create comprehensive benchmark suite to measure performance of core path operations and establish baseline metrics for future optimization work.

## Status

🔄 Planned

## Priority Metrics

- **Value**: 7/10 (needed for v1.0.0, performance baseline)
- **Easiness**: 5/10 (moderate effort, straightforward implementation)
- **Priority**: 4/5 (needed for release quality)
- **Safety**: 5/5 (safe, just adding benchmarks)
- **Advisability**: 700

## Problem Statement

**From Spec** (Success Criteria #5):
> **5. Performance**
> - **Specific**: Path normalization ≤ 1μs per component on 3GHz CPU; ≤ 1 allocation per operation
> - **Measurable**: Benchmark suite shows `normalize()` at 50,000 ops/sec for typical paths
> - **Current Status**: Algorithm is O(n); benchmarking suite pending

Currently there is no benchmark suite to:
- Measure baseline performance of core operations
- Detect performance regressions in CI
- Validate performance claims in specification
- Guide optimization efforts

## Required Benchmarks

### Core Operations (from spec)

1. **normalize()** - various path lengths
   - Short paths (< 20 chars)
   - Medium paths (20-100 chars)
   - Long paths (> 100 chars)
   - Paths with many `..` components

2. **canonicalize()** - typical usage
   - Already normalized paths
   - Paths needing normalization
   - Windows verbatim prefix stripping

3. **path_common()** - different path counts
   - 2 paths (common case)
   - 5 paths
   - 10 paths
   - No common prefix case

4. **path_relative()** - various scenarios
   - Same directory
   - Parent directory
   - Deep nesting (5+ levels)

5. **join() / iter_join()** - multiple segments
   - 2 segments
   - 5 segments
   - 10 segments

### Type Conversions

6. **TryIntoPath** - various source types
7. **TryIntoCowPath** - borrowed vs owned
8. **AsPath** - reference types

## Deliverables

1. **File**: `benches/path_ops.rs` with all benchmarks
2. **Documentation**: Baseline performance report
3. **CI Integration**: Performance regression detection (optional)

## Implementation Strategy

### Benchmark Structure

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pth::*;

fn bench_normalize(c: &mut Criterion) {
  let mut group = c.benchmark_group("normalize");

  group.bench_function("short_path", |b| {
    b.iter(|| normalize(black_box("./foo/bar")))
  });

  group.bench_function("long_path", |b| {
    b.iter(|| normalize(black_box("./a/b/c/d/e/f/g/h/i/j/k")))
  });

  group.bench_function("many_dotdots", |b| {
    b.iter(|| normalize(black_box("./a/b/c/../../d/../../e")))
  });

  group.finish();
}

criterion_group!(benches, bench_normalize, /* ... */);
criterion_main!(benches);
```

### Add Criterion Dependency

Update `Cargo.toml`:
```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "path_ops"
harness = false
```

## Acceptance Criteria

- [ ] `benches/path_ops.rs` created with all 8 benchmark categories
- [ ] At least 20 individual benchmark cases
- [ ] Criterion configured correctly
- [ ] `cargo bench` runs successfully
- [ ] Baseline report generated and documented
- [ ] Performance targets from spec validated or updated
- [ ] Optional: CI configured to track performance over time

## Estimated Effort

4-6 hours
- Setup: 30 min
- Implement benchmarks: 3-4 hours
- Documentation: 1 hour
- CI integration: 1 hour (optional)

## Target Milestone

v0.29.0 (P1 - HIGH)

## Related Issues

- Discovery issue 4.1: Missing Benchmark Suite
- Spec Success Criteria #5: Performance metrics
- Discovery issue 6.7: Reduce Allocations in iter_join() (will use benchmarks)

## Implementation Notes

### Performance Targets (from spec)

- `normalize()`: ≥ 50,000 ops/sec for typical paths
- Single pass: ≤ 1μs per path component on 3GHz CPU
- Allocations: ≤ 1 allocation per operation

### Example Output Format

Document baseline in `benches/BASELINE.md`:
```markdown
# Baseline Performance Report

**Date**: 2025-10-29
**CPU**: 3GHz (specify actual)
**Rust**: 1.83.0

## Results

| Operation | Input Size | Throughput | Time per Op |
|-----------|------------|------------|-------------|
| normalize() | short (< 20 chars) | 120,000 ops/sec | 8.3 μs |
| normalize() | long (> 100 chars) | 45,000 ops/sec | 22.2 μs |
| path_common() | 2 paths | 80,000 ops/sec | 12.5 μs |
| iter_join() | 5 segments | 60,000 ops/sec | 16.7 μs |

## Analysis

All operations meet or exceed performance targets from specification.
```

### Optional: CI Integration

Add to GitHub Actions:
```yaml
- name: Run benchmarks
  run: cargo bench --bench path_ops -- --save-baseline current

- name: Compare with previous
  run: cargo bench --bench path_ops -- --baseline previous
```
