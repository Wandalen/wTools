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


## 2025-08-05 - Fixed benchmark dead loop issues - stable benchmark suite working

**Change Type**: Infrastructure  
**Description**: Fixed benchmark dead loop issues - stable benchmark suite working

**Performance Impact**:
- Performance metrics extracted from benchmark run

**Benchmark Evidence**:
```
stable_operations/single_colon/small
                        time:   [14.836 µs 14.930 µs 15.105 µs]
                        thrpt:  [6.3138 MiB/s 6.3877 MiB/s 6.4282 MiB/s]
                 change:
                        time:   [+0.2503% +2.1367% +5.2601%] (p = 0.10 > 0.05)
                        thrpt:  [-4.9973% -2.0920% -0.2496%]
                        No change in performance detected.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe
stable_operations/multi_delim/small
                        time:   [59.763 µs 60.312 µs 60.955 µs]
                        thrpt:  [1.5646 MiB/s 1.5812 MiB/s 1.5958 MiB/s]
                 change:
                        time:   [-1.0985% -0.1760% +0.8502%] (p = 0.74 > 0.05)
                        thrpt:  [-0.8430% +0.1763% +1.1107%]
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
stable_operations/single_colon/medium
                        time:   [50.087 µs 50.257 µs 50.486 µs]
                        thrpt:  [18.890 MiB/s 18.976 MiB/s 19.040 MiB/s]
                 change:
                        time:   [-0.4895% -0.1349% +0.2295%] (p = 0.52 > 0.05)
                        thrpt:  [-0.2290% +0.1351% +0.4920%]
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
stable_operations/multi_delim/medium
                        time:   [815.27 µs 815.74 µs 816.25 µs]
                        thrpt:  [1.1684 MiB/s 1.1691 MiB/s 1.1698 MiB/s]
                 change:
                        time:   [-1.2188% -0.8639% -0.5484%] (p = 0.00 < 0.05)
                        thrpt:  [+0.5514% +0.8714% +1.2339%]
                        Change within noise threshold.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) low mild
stable_operations/single_colon/large
                        time:   [618.37 µs 621.21 µs 624.92 µs]
                        thrpt:  [15.261 MiB/s 15.352 MiB/s 15.422 MiB/s]
                 change:
                        time:   [+0.1145% +0.7449% +1.3085%] (p = 0.03 < 0.05)
                        thrpt:  [-1.2916% -0.7393% -0.1144%]
                        Change within noise threshold.
stable_operations/multi_delim/large
                        time:   [85.484 ms 85.550 ms 85.657 ms]
                        thrpt:  [114.01 KiB/s 114.15 KiB/s 114.24 KiB/s]
                 change:
                        time:   [-28.291% -18.303% -7.1666%] (p = 0.01 < 0.05)
                        thrpt:  [+7.7198% +22.404% +39.453%]
                        Performance has improved.
[Output truncated - see full logs for complete results]
```

**Environment**:
- Platform: linux aarch64
- Rust: rustc 1.88.0 (6b00bc388 2025-06-23)
- Date: 2025-08-05 21:43:45 UTC
- Test conditions: criterion.rs, 10 samples, 1s measurement time  
- Benchmark type: Baseline

**Root Cause Analysis**: Performance change due to infrastructure implementation

**Related Files**:
- benches/string_operations.rs - Main benchmark suite
- src/string/split/ - String splitting implementation

**Validation**: Automated benchmark run with consistent measurement methodology


## 2025-08-05 - SIMD optimization implementation - baseline measurement

**Change Type**: Infrastructure  
**Description**: SIMD optimization implementation - baseline measurement

**Performance Impact**:
- Performance metrics extracted from benchmark run

**Benchmark Evidence**:
```
minimal_split           time:   [1.2246 µs 1.2266 µs 1.2290 µs]
                        change: [-0.7151% -0.5163% -0.3325%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe


   Compiling strs_tools v0.23.0 (/home/user1/pro/lib/wTools2/module/core/strs_tools)
warning: missing documentation for the crate
  --> module/core/strs_tools/benches/minimal_test.rs:1:1
   |
1  | / use criterion::{ black_box, criterion_group, criterion_main, Criterion };
2  | | use strs_tools::string::split;
3  | |
4  | | /// Ultra-minimal benchmark that cannot hang
...  |
21 | | criterion_group!( benches, bench_minimal_split );
22 | | criterion_main!( benches );
   | |___________________________^
   |
   = note: requested on the command line with `-W missing-docs`

warning: missing documentation for a function
  --> module/core/strs_tools/benches/minimal_test.rs:21:1
   |
21 | criterion_group!( benches, bench_minimal_split );
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this warning originates in the macro `$crate::criterion_group` which comes from the expansion of the macro `criterion_group` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: `strs_tools` (bench "minimal_test") generated 2 warnings
    Finished `bench` profile [optimized] target(s) in 3.73s
     Running benches/minimal_test.rs (/home/user1/pro/lib/wTools2/target/release/deps/minimal_test-b9084ecd4d6b1318)
Gnuplot not found, using plotters backend
Benchmarking minimal_split
Benchmarking minimal_split: Warming up for 3.0000 s
Benchmarking minimal_split: Collecting 10 samples in estimated 1.0000 s (816k iterations)
Benchmarking minimal_split: Analyzing

```

**Environment**:
- Platform: linux aarch64
- Rust: rustc 1.88.0 (6b00bc388 2025-06-23)
- Date: 2025-08-05 21:50:38 UTC
- Test conditions: criterion.rs, 10 samples, 1s measurement time  
- Benchmark type: Baseline

**Root Cause Analysis**: Performance change due to infrastructure implementation

**Related Files**:
- benches/string_operations.rs - Main benchmark suite
- src/string/split/ - String splitting implementation

**Validation**: Automated benchmark run with consistent measurement methodology


## 2025-08-05 - SIMD string operations implementation with performance comparison

**Change Type**: Optimization  
**Description**: SIMD string operations implementation with performance comparison

**Performance Impact**:
- Performance metrics extracted from benchmark run

**Benchmark Evidence**:
```
stable_operations/single_colon/small
                        time:   [14.796 µs 14.879 µs 14.948 µs]
                        thrpt:  [6.3800 MiB/s 6.4097 MiB/s 6.4453 MiB/s]
                 change:
                        time:   [-4.4747% -1.6963% +0.2555%] (p = 0.26 > 0.05)
                        thrpt:  [-0.2548% +1.7256% +4.6844%]
                        No change in performance detected.
stable_operations/multi_delim/small
                        time:   [59.597 µs 59.639 µs 59.728 µs]
                        thrpt:  [1.5967 MiB/s 1.5991 MiB/s 1.6002 MiB/s]
                 change:
                        time:   [-1.2470% -0.5223% +0.0441%] (p = 0.15 > 0.05)
                        thrpt:  [-0.0440% +0.5251% +1.2628%]
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
stable_operations/single_colon/medium
                        time:   [49.876 µs 49.896 µs 49.918 µs]
                        thrpt:  [19.105 MiB/s 19.113 MiB/s 19.121 MiB/s]
                 change:
                        time:   [-0.9721% -0.6421% -0.3922%] (p = 0.00 < 0.05)
                        thrpt:  [+0.3937% +0.6463% +0.9816%]
                        Change within noise threshold.
stable_operations/multi_delim/medium
                        time:   [810.05 µs 810.26 µs 810.58 µs]
                        thrpt:  [1.1765 MiB/s 1.1770 MiB/s 1.1773 MiB/s]
                 change:
                        time:   [-0.7146% -0.5841% -0.4167%] (p = 0.00 < 0.05)
                        thrpt:  [+0.4185% +0.5875% +0.7198%]
                        Change within noise threshold.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
stable_operations/single_colon/large
                        time:   [618.11 µs 618.22 µs 618.39 µs]
                        thrpt:  [15.422 MiB/s 15.426 MiB/s 15.429 MiB/s]
                 change:
                        time:   [-0.9085% -0.4543% -0.0391%] (p = 0.07 > 0.05)
                        thrpt:  [+0.0391% +0.4564% +0.9169%]
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
stable_operations/multi_delim/large
                        time:   [85.661 ms 85.742 ms 85.818 ms]
                        thrpt:  [113.79 KiB/s 113.90 KiB/s 114.00 KiB/s]
                 change:
                        time:   [+0.0695% +0.2244% +0.3485%] (p = 0.01 < 0.05)
                        thrpt:  [-0.3472% -0.2238% -0.0694%]
                        Change within noise threshold.


[Output truncated - see full logs for complete results]
```

**Environment**:
- Platform: linux aarch64
- Rust: rustc 1.88.0 (6b00bc388 2025-06-23)
- Date: 2025-08-05 21:52:39 UTC
- Test conditions: criterion.rs, 10 samples, 1s measurement time  
- Benchmark type: Baseline

**Root Cause Analysis**: Performance change due to optimization implementation

**Related Files**:
- benches/string_operations.rs - Main benchmark suite
- src/string/split/ - String splitting implementation

**Validation**: Automated benchmark run with consistent measurement methodology


## 2025-08-06 - Enable SIMD optimizations by default - users now get SIMD acceleration out of the box

**Change Type**: Configuration  
**Description**: Enable SIMD optimizations by default - users now get SIMD acceleration out of the box

**Performance Impact**:
- Performance metrics extracted from benchmark run

**Benchmark Evidence**:
```
stable_operations/single_colon/small
                        time:   [15.194 µs 16.870 µs 18.902 µs]
                        thrpt:  [5.0455 MiB/s 5.6529 MiB/s 6.2765 MiB/s]
                 change:
                        time:   [+2.7442% +8.8332% +16.327%] (p = 0.02 < 0.05)
                        thrpt:  [-14.035% -8.1163% -2.6709%]
                        Performance has regressed.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
stable_operations/multi_delim/small
                        time:   [58.273 µs 58.333 µs 58.430 µs]
                        thrpt:  [1.6322 MiB/s 1.6349 MiB/s 1.6366 MiB/s]
                 change:
                        time:   [-2.4312% -2.1372% -1.7585%] (p = 0.00 < 0.05)
                        thrpt:  [+1.7899% +2.1838% +2.4918%]
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
stable_operations/single_colon/medium
                        time:   [48.118 µs 48.132 µs 48.142 µs]
                        thrpt:  [19.809 MiB/s 19.814 MiB/s 19.819 MiB/s]
                 change:
                        time:   [-3.5957% -3.5594% -3.5229%] (p = 0.00 < 0.05)
                        thrpt:  [+3.6516% +3.6908% +3.7298%]
                        Performance has improved.
stable_operations/multi_delim/medium
                        time:   [790.09 µs 790.40 µs 790.70 µs]
                        thrpt:  [1.2061 MiB/s 1.2066 MiB/s 1.2070 MiB/s]
                 change:
                        time:   [-2.6214% -2.4900% -2.3917%] (p = 0.00 < 0.05)
                        thrpt:  [+2.4503% +2.5536% +2.6920%]
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
stable_operations/single_colon/large
                        time:   [601.26 µs 601.96 µs 603.30 µs]
                        thrpt:  [15.808 MiB/s 15.843 MiB/s 15.861 MiB/s]
                 change:
                        time:   [-2.6549% -2.4210% -2.1559%] (p = 0.00 < 0.05)
                        thrpt:  [+2.2034% +2.4811% +2.7273%]
                        Performance has improved.
stable_operations/multi_delim/large
                        time:   [83.429 ms 83.441 ms 83.456 ms]
                        thrpt:  [117.02 KiB/s 117.04 KiB/s 117.05 KiB/s]
                 change:
                        time:   [-2.7715% -2.6840% -2.5900%] (p = 0.00 < 0.05)
                        thrpt:  [+2.6589% +2.7581% +2.8505%]
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
[Output truncated - see full logs for complete results]
```

**Environment**:
- Platform: linux aarch64
- Rust: rustc 1.88.0 (6b00bc388 2025-06-23)
- Date: 2025-08-06 06:21:21 UTC
- Test conditions: criterion.rs, 10 samples, 1s measurement time  
- Benchmark type: Baseline

**Root Cause Analysis**: Performance change due to configuration implementation

**Related Files**:
- benches/string_operations.rs - Main benchmark suite
- src/string/split/ - String splitting implementation

**Validation**: Automated benchmark run with consistent measurement methodology


## 2025-08-06 - Updated benchmark runner to avoid creating backup files

**Change Type**: Configuration  
**Description**: Updated benchmark runner to avoid creating backup files

**Performance Impact**:
- Performance metrics extracted from benchmark run

**Benchmark Evidence**:
```
minimal_split           time:   [1.2047 µs 1.2052 µs 1.2060 µs]
                        change: [-1.7726% -1.6443% -1.5400%] (p = 0.00 < 0.05)
                        Performance has improved.


   Compiling strs_tools v0.23.0 (/home/user1/pro/lib/wTools2/module/core/strs_tools)
warning: missing documentation for the crate
  --> module/core/strs_tools/benches/minimal_test.rs:1:1
   |
1  | / use criterion::{ black_box, criterion_group, criterion_main, Criterion };
2  | | use strs_tools::string::split;
3  | |
4  | | /// Ultra-minimal benchmark that cannot hang
...  |
21 | | criterion_group!( benches, bench_minimal_split );
22 | | criterion_main!( benches );
   | |___________________________^
   |
   = note: requested on the command line with `-W missing-docs`

warning: missing documentation for a function
  --> module/core/strs_tools/benches/minimal_test.rs:21:1
   |
21 | criterion_group!( benches, bench_minimal_split );
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this warning originates in the macro `$crate::criterion_group` which comes from the expansion of the macro `criterion_group` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: `strs_tools` (bench "minimal_test") generated 2 warnings
    Finished `bench` profile [optimized] target(s) in 1.00s
     Running benches/minimal_test.rs (/home/user1/pro/lib/wTools2/target/release/deps/minimal_test-b5d9e7ac6e13c8a5)
Gnuplot not found, using plotters backend
Benchmarking minimal_split
Benchmarking minimal_split: Warming up for 3.0000 s
Benchmarking minimal_split: Collecting 10 samples in estimated 1.0000 s (830k iterations)
Benchmarking minimal_split: Analyzing

```

**Environment**:
- Platform: linux aarch64
- Rust: rustc 1.88.0 (6b00bc388 2025-06-23)
- Date: 2025-08-06 06:23:24 UTC
- Test conditions: criterion.rs, 10 samples, 1s measurement time  
- Benchmark type: Baseline

**Root Cause Analysis**: Performance change due to configuration implementation

**Related Files**:
- benches/string_operations.rs - Main benchmark suite
- src/string/split/ - String splitting implementation

**Validation**: Automated benchmark run with consistent measurement methodology
