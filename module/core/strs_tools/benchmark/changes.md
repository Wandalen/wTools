# Performance Changes History

**Documentation**: See `benchmark/readme.md` for guidelines and templates.

---

## 2025-08-05 - Baseline Measurements Established

**Change Type**: Infrastructure  
**Description**: Implemented comprehensive benchmarking infrastructure using criterion.rs with baseline scalar performance measurements

**Performance Impact**:
- Single delimiter split (1KB): 147.4 MiB/s (space), 94.6 MiB/s (period)
- Single delimiter split (10KB): 231.1 MiB/s (newline), 115.2 MiB/s (period)
- Multi-delimiter performance baseline established for SIMD comparison
- Target improvement: 6x faster (720-900 MiB/s with SIMD)

**Benchmark Evidence**:
```
Single Delimiter Split (1KB input):
- Space delimiter: 147.4 MiB/s (6.47 µs)
- Newline delimiter: 212.4 MiB/s (4.49 µs) 
- Colon delimiter: 124.3 MiB/s (7.67 µs)
- Semicolon delimiter: 123.9 MiB/s (7.70 µs)
- Comma delimiter: 117.9 MiB/s (8.09 µs)
- Period delimiter: 94.6 MiB/s (10.08 µs)

Single Delimiter Split (10KB input):
- Space delimiter: 140.4 MiB/s (67.9 µs)
- Newline delimiter: 231.1 MiB/s (41.3 µs)
- Colon delimiter: 144.0 MiB/s (66.2 µs)
- Semicolon delimiter: 138.9 MiB/s (68.7 µs)
- Comma delimiter: 132.0 MiB/s (72.2 µs)
- Period delimiter: 115.2 MiB/s (82.8 µs)

Single Delimiter Split (100KB input):
- Space delimiter: 138.5 MiB/s (688.7 µs)
- Comma delimiter: 127.3 MiB/s (749.4 µs)
```

**Environment**:
- Platform: Linux 6.8.0-64-generic x86_64
- Rust: Current stable
- Test data: Generated strings with 30% delimiter density
- Sample sizes: 100B, 1KB, 10KB, 100KB, 1MB
- Measurement: criterion.rs with 15-20 samples, 3s measurement time

**Root Cause Analysis**: Initial baseline establishment - no previous measurements for comparison

**Related Files**:
- `benchmark_baseline_results.md` - Detailed baseline documentation
- `benches/string_operations.rs` - Main benchmark suite
- `benches/memory_usage.rs` - Memory allocation benchmarks
- `task/001_simd_optimization.md` - Implementation task with benchmarking strategy

**Validation**: 
- Benchmarks run successfully across multiple data sizes
- Consistent results with <5% variance across runs
- Target for SIMD optimization: 6x improvement (720-900 MiB/s)
- Key insights: Newline delimiter shows exceptional performance (likely LLVM optimization), period delimiter consistently slowest, good scaling efficiency (90-95%) from 1KB to 100KB