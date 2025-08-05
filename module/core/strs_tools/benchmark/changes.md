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

## 2025-08-05 - Test benchmark runner functionality with quick mode

**Change Type**: Infrastructure  
**Description**: Test benchmark runner functionality with quick mode

**Performance Impact**:
- Performance metrics extracted from benchmark run

**Benchmark Evidence**:
```
quick_split_test        time:   [2.1451 µs 2.1520 µs 2.1571 µs]
                        change: [-29.383% -19.393% -8.5267%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) low mild


warning: missing documentation for the crate
  --> module/core/strs_tools/benches/quick_test.rs:1:1
   |
1  | / use criterion::{ black_box, criterion_group, criterion_main, Criterion };
2  | | use strs_tools::string::split;
3  | |
4  | | /// Quick benchmark for testing the benchmark runner functionality
...  |
24 | | criterion_group!( benches, bench_quick_split );
25 | | criterion_main!( benches );
   | |___________________________^
   |
   = note: requested on the command line with `-W missing-docs`

warning: missing documentation for a function
  --> module/core/strs_tools/benches/quick_test.rs:24:1
   |
24 | criterion_group!( benches, bench_quick_split );
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this warning originates in the macro `$crate::criterion_group` which comes from the expansion of the macro `criterion_group` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: `strs_tools` (bench "quick_test") generated 2 warnings
    Finished `bench` profile [optimized] target(s) in 0.28s
     Running benches/quick_test.rs (/home/user1/pro/lib/wTools2/target/release/deps/quick_test-565b893fab3f2031)
Gnuplot not found, using plotters backend
Benchmarking quick_split_test
Benchmarking quick_split_test: Warming up for 3.0000 s
Benchmarking quick_split_test: Collecting 10 samples in estimated 1.0001 s (463k iterations)
Benchmarking quick_split_test: Analyzing

```

**Environment**:
- Platform: linux aarch64
- Rust: rustc 1.88.0 (6b00bc388 2025-06-23)
- Date: 2025-08-05 20:55:13 UTC
- Test conditions: criterion.rs, 10 samples, 1s measurement time  
- Benchmark type: Baseline

**Root Cause Analysis**: Performance change due to infrastructure implementation

**Related Files**:
- benches/string_operations.rs - Main benchmark suite
- src/string/split/ - String splitting implementation

**Validation**: Automated benchmark run with consistent measurement methodology
