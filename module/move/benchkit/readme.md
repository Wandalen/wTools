# benchkit

[![docs.rs](https://docs.rs/benchkit/badge.svg)](https://docs.rs/benchkit)
[![discord](https://img.shields.io/discord/872391416519647252?color=eee&logo=discord&logoColor=eee&label=ask%20on%20discord)](https://discord.gg/m3YfbXpUUY)

**Practical, Documentation-First Benchmarking for Rust.**

`benchkit` is a lightweight toolkit for performance analysis, born from the hard-learned lessons of optimizing high-performance libraries. It rejects rigid, all-or-nothing frameworks in favor of flexible, composable tools that integrate seamlessly into your existing workflow.

## The Benchmarking Dilemma

In Rust, developers often face a frustrating choice:

1.  **The Heavy Framework (`criterion`):** Statistically powerful, but forces a rigid structure (`benches/`), complex setup, and produces reports that are difficult to integrate into your project's documentation. You must adapt your project to the framework.
2.  **The Manual Approach (`std::time`):** Simple to start, but statistically naive. It leads to boilerplate, inconsistent measurements, and conclusions that are easily skewed by system noise.

`benchkit` offers a third way.

## A Toolkit, Not a Framework

This is the core philosophy of `benchkit`. It doesn't impose a workflow; it provides a set of professional, composable tools that you can use however you see fit.

*   ✅ **Integrate Anywhere:** Write benchmarks in your test files, examples, or binaries. No required directory structure.
*   ✅ **Documentation-First:** Treat performance reports as a first-class part of your documentation, with tools to automatically keep them in sync with your code.
*   ✅ **Practical Focus:** Surface the key metrics needed for optimization decisions, hiding deep statistical complexity until you ask for it.
*   ✅ **Zero Setup:** Start measuring performance in minutes with a simple, intuitive API.

---

## 🚀 Quick Start: Compare, Analyze, and Document

This example demonstrates the core `benchkit` workflow: comparing two algorithms and automatically updating a performance section in your `readme.md`.

**1. Add to `dev-dependencies` in `Cargo.toml`:**
```toml
[dev-dependencies]
benchkit = { version = "0.1", features = [ "full" ] }
```

**2. Create a benchmark in your `benches` directory:**

```rust  
// In benches/performance_demo.rs
#![ cfg( feature = "enabled" ) ]
use benchkit::prelude::*;

fn generate_data( size : usize ) -> Vec< u32 >
{
  ( 0..size ).map( | x | x as u32 ).collect()
}

#[ test ]
fn update_readme_performance_docs()
{
  let mut comparison = ComparativeAnalysis::new( "Sorting Algorithms" );
  let data = generate_data( 1000 );

  // Benchmark the first algorithm
  comparison = comparison.algorithm
  (
    "std_stable_sort",
    {
      let mut d = data.clone();
      move ||
      {
        d.sort();
      }
    }
  );

  // Benchmark the second algorithm
  comparison = comparison.algorithm
  (
    "std_unstable_sort",
    {
      let mut d = data.clone();
      move ||
      {
        d.sort_unstable();
      }
    }
  );

  // Run the comparison and update benchmark_results.md
  let report = comparison.run();
  let markdown = report.to_markdown();

  let updater = MarkdownUpdater::new( "benchmark_results.md", "Benchmark Results" ).unwrap();
  updater.update_section( &markdown ).unwrap();
}
```

**3. Run your benchmark and watch benchmark_results.md update automatically:**
```bash
cargo run --bin performance_demo --features enabled  
```

---

## 🧰 What's in the Toolkit?

`benchkit` provides a suite of composable tools. Use only what you need.

<details>
<summary><strong>Measure: Core Timing and Profiling</strong></summary>

At its heart, `benchkit` provides simple and accurate measurement primitives.

```rust
use benchkit::prelude::*;

// A robust measurement with multiple iterations and statistical cleanup.
let result = bench_function
(
  "summation_1000",
  ||
  {
    ( 0..1000 ).fold( 0, | acc, x | acc + x )
  }
);
println!( "Avg time: {:.2?}", result.mean_time() );
println!( "Throughput: {:.0} ops/sec", result.operations_per_second() );

// Track memory usage patterns alongside timing.
let memory_benchmark = MemoryBenchmark::new( "allocation_test" );
let ( timing, memory_stats ) = memory_benchmark.run_with_tracking
(
  10,
  ||
  {
    let data = vec![ 0u8; 1024 ];
    memory_benchmark.tracker.record_allocation( 1024 );
    std::hint::black_box( data );
  }
);
println!( "Peak memory usage: {} bytes", memory_stats.peak_usage );
```

</details>

<details>
<summary><strong>Analyze: Find Insights and Regressions</strong></summary>

Turn raw numbers into actionable insights.

```rust
use benchkit::prelude::*;

// Compare multiple implementations to find the best one.
let report = ComparativeAnalysis::new( "Hashing" )
.algorithm( "fnv", || { /* ... */ } )
.algorithm( "siphash", || { /* ... */ } )
.run();

if let Some( ( fastest_name, _ ) ) = report.fastest()
{
  println!( "Fastest algorithm: {}", fastest_name );
}

// Example benchmark results
let result_a = bench_function( "test_a", || { /* ... */ } );
let result_b = bench_function( "test_b", || { /* ... */ } );

// Compare two benchmark results
let comparison = result_a.compare( &result_b );
if comparison.is_improvement()
{
  println!( "Performance improved!" );
}
```

</details>

<details>
<summary><strong>Generate: Create Realistic Test Data</strong></summary>

Stop writing boilerplate to create test data. `benchkit` provides generators for common scenarios.

```rust
use benchkit::prelude::*;

// Generate a comma-separated list of 100 items.
let list_data = generate_list_data( DataSize::Medium );

// Generate realistic unilang command strings for parser benchmarking.
let command_generator = DataGenerator::new()
.complexity( DataComplexity::Complex );
let commands = command_generator.generate_unilang_commands( 10 );

// Create reproducible data with a specific seed.
let mut seeded_gen = SeededGenerator::new( 42 );
let random_data = seeded_gen.random_string( 1024 );
```

</details>

<details>
<summary><strong>Document: Automate Your Reports</strong></summary>

The "documentation-first" philosophy is enabled by powerful report generation and file updating tools.

```rust
use benchkit::prelude::*;

fn main() -> Result< (), Box< dyn std::error::Error > >
{
  let mut suite = BenchmarkSuite::new( "api_performance" );
  suite.benchmark( "get_user", || { /* ... */ } );
  suite.benchmark( "create_user", || { /* ... */ } );
  let results = suite.run_analysis();

  // Generate a markdown report from the results.
  let markdown_report = results.generate_markdown_report().generate();

  // Automatically update the "## Performance" section of a file.
  let updater = MarkdownUpdater::new( "readme.md", "Performance" )?;
  updater.update_section( &markdown_report )?;
  
  Ok( () )
}
```

</details>

## The `benchkit` Workflow

`benchkit` is designed to make performance analysis a natural part of your development cycle.

```text
[ 1. Write Code ] -> [ 2. Add Benchmark in `benches/` ] -> [ 3. Run `cargo run --bin` ]
       ^                                                                   |
       |                                                                   v
[ 5. Commit Code + Perf Docs ] <- [ 4. Auto-Update `benchmark_results.md` ] <- [ Analyze Results ]
```

## 📁 Standard `benches/` Directory Integration

`benchkit` fully embraces the standard Rust `benches/` directory structure with enhanced capabilities. **ALL benchmark-related files must be located in the standard `benches/` directory** - this is where Rust expects performance benchmarks to live.

- ✅ **Use `benches/`**: The standard directory for ALL benchmark-related files (this is the Rust convention)
- ✅ **Comprehensive reporting**: `benchmark_results.md` automatically updated with benchmark results
- ✅ **Organized structure**: Keep all performance analysis, data, and reports in `benches/`
- ✅ **Standard compliance**: Follow Rust ecosystem conventions for benchmark organization
- ❌ **Never use `tests/`**: Keep performance benchmarks separate from unit tests (tests/ is for correctness, not performance)

### Why This Matters

**Standard Rust Convention**: The `benches/` directory is the established Rust ecosystem standard for ALL benchmark-related files - following this convention ensures consistency across projects.

**Automatic Documentation**: `benchkit` automatically updates `benchmark_results.md` with comprehensive benchmark reports, creating living documentation that stays current with your performance characteristics.

**Organized Performance Analysis**: Keep ALL benchmark code, data generation, analysis scripts, and reports centralized in `benches/` for easy maintenance and discovery.

**Ecosystem Integration**: Tools like `cargo bench` expect benchmarks in `benches/` - following this standard ensures compatibility with the broader Rust toolchain.

**Separation of Concerns**: Performance benchmarks serve different purposes than correctness tests - `benches/` keeps them properly separated from `tests/` while following Rust conventions.

### Automatic `benchmark_results.md` Reports

`benchkit` excels at maintaining comprehensive, automatically updated documentation in your `benchmark_results.md` file:

```markdown
# Benchmark Results

## Algorithm Comparison

| Algorithm | Mean Time | Throughput | Relative |
|-----------|-----------|------------|----------|
| quicksort | 1.23ms    | 815 ops/s  | baseline |
| mergesort | 1.45ms    | 689 ops/s  | 1.18x    |
| heapsort  | 1.67ms    | 599 ops/s  | 1.36x    |

*Last updated: 2024-01-15 14:32:18 UTC*
*Generated by benchkit v0.4.0*

## Performance Trends

- quicksort maintains consistent performance across data sizes
- mergesort shows better cache behavior on large datasets
- heapsort provides predictable O(n log n) guarantees

## Test Configuration

- Hardware: 16-core AMD Ryzen, 32GB RAM
- Rust version: 1.75.0
- Optimization: --release
- Iterations: 1000 per benchmark
```

This documentation is automatically generated and updated every time you run benchmarks.

### Integration Examples

```rust
// ✅ In standard benches/ directory - ALL benchmark files belong here
// benches/algorithm_comparison.rs
use benchkit::prelude::*;

fn main()
{
  let mut suite = BenchmarkSuite::new( "Algorithm Comparison" );
  
  suite.benchmark( "quick_sort", ||
  {
    // Your quicksort implementation
  });
  
  suite.benchmark( "merge_sort", ||  
  {
    // Your mergesort implementation
  });
  
  let results = suite.run_all();
  
  // Automatically update benchmark_results.md with results
  let updater = MarkdownUpdater::new( "benchmark_results.md", "Benchmark Results" ).unwrap();
  updater.update_section( &results.generate_markdown_report().generate() ).unwrap();
}
```

```rust
// ✅ Standard benches/ structure with comprehensive reporting
// benches/performance_suite.rs  
use benchkit::prelude::*;

fn main()
{
  let mut comprehensive = BenchmarkSuite::new( "Comprehensive Performance Analysis" );
  
  // Add multiple benchmarks
  comprehensive.benchmark( "data_processing", || { /* code */ } );
  comprehensive.benchmark( "memory_operations", || { /* code */ } );
  comprehensive.benchmark( "io_operations", || { /* code */ } );
  
  let results = comprehensive.run_all();
  
  // Update benchmark_results.md with comprehensive report
  let report = results.generate_markdown_report();
  let updater = MarkdownUpdater::new( "benchmark_results.md", "Performance Analysis" ).unwrap();
  updater.update_section( &report.generate() ).unwrap();
  
  println!( "Updated benchmark_results.md with latest performance results" );
}
```

### 🔧 Feature Flag Recommendations

For optimal build performance and clean separation, put your benchmark code behind feature flags:

```rust
// ✅ Standard benches/ directory with feature flags for optional execution
// benches/comprehensive_benchmark.rs
#[ cfg( feature = "enabled" ) ]
use benchkit::prelude::*;

#[ cfg( feature = "enabled" ) ]
fn main()
{
  let mut suite = BenchmarkSuite::new( "Comprehensive Performance Suite" );
  
  suite.benchmark( "algorithm_a", || { /* implementation */ } );
  suite.benchmark( "algorithm_b", || { /* implementation */ } );
  suite.benchmark( "data_structure_ops", || { /* implementation */ } );
  
  let results = suite.run_all();
  
  // Automatically update benchmark_results.md
  let updater = MarkdownUpdater::new( "benchmark_results.md", "Latest Results" ).unwrap();
  updater.update_section( &results.generate_markdown_report().generate() ).unwrap();
  
  println!( "Benchmarks completed - benchmark_results.md updated" );
}

#[ cfg( not( feature = "enabled" ) ) ]
fn main()
{
  println!( "Run with: cargo run --bin comprehensive_benchmark --features enabled" );
  println!( "Results will be automatically saved to benchmark_results.md" );
}
```

Add to your `Cargo.toml`:

```toml
[features]
benchmark = ["benchkit"]

[dev-dependencies]
benchkit = { version = "0.1", features = ["full"], optional = true }
```

Run benchmarks selectively:
```bash
# Run only unit tests (fast)
cargo test

# Run specific benchmark binary (updates benchmark_results.md)
cargo run --bin comprehensive_benchmark --features enabled

# Run all benchmark binaries in benches/
find benches/ -name "*.rs" -exec basename {} .rs \; | xargs -I {} cargo run --bin {} --features enabled

# Use cargo bench for standard Rust benchmarking
cargo bench --features enabled
```

This approach keeps your regular builds fast while making comprehensive performance testing available when needed.

## Installation

Add `benchkit` to your `[dev-dependencies]` in `Cargo.toml`.

```toml
[dev-dependencies]
# For core functionality
benchkit = "0.1"

# Or enable all features for the full toolkit
benchkit = { version = "0.1", features = [ "full" ] }
```

## Contributing

Contributions are welcome! `benchkit` aims to be a community-driven toolkit that solves real-world benchmarking problems. Please see our contribution guidelines and open tasks.

## License

This project is licensed under the **MIT License**.

## Performance

## api_performance Results

| Benchmark | Mean Time | Ops/sec | Min | Max | Std Dev |
|-----------|-----------|---------|-----|-----|----------|
| get_user | 40.00ns | 25000000 | 0.00ns | 80.00ns | 19.00ns |
| create_user | 40.00ns | 25000000 | 40.00ns | 40.00ns | 0.00ns |

### Key Insights

- **Fastest operation**: get_user (40.00ns)
- **Performance range**: 1.0x difference between fastest and slowest



## api_performance Results

| Benchmark | Mean Time | Ops/sec | Min | Max | Std Dev |
|-----------|-----------|---------|-----|-----|----------|
| create_user | 40.00ns | 25000000 | 40.00ns | 40.00ns | 0.00ns |
| get_user | 40.00ns | 25000000 | 40.00ns | 40.00ns | 0.00ns |

### Key Insights

- **Fastest operation**: create_user (40.00ns)
- **Performance range**: 1.0x difference between fastest and slowest



## api_performance Results

| Benchmark | Mean Time | Ops/sec | Min | Max | Std Dev |
|-----------|-----------|---------|-----|-----|----------|
| create_user | 36.00ns | 27777778 | 0.00ns | 40.00ns | 13.00ns |
| get_user | 36.00ns | 27777778 | 0.00ns | 40.00ns | 13.00ns |

### Key Insights

- **Fastest operation**: create_user (36.00ns)
- **Performance range**: 1.0x difference between fastest and slowest



## api_performance Results

| Benchmark | Mean Time | Ops/sec | Min | Max | Std Dev |
|-----------|-----------|---------|-----|-----|----------|
| create_user | 36.00ns | 27777778 | 0.00ns | 40.00ns | 13.00ns |
| get_user | 40.00ns | 25000000 | 40.00ns | 40.00ns | 0.00ns |

### Key Insights

- **Fastest operation**: create_user (36.00ns)
- **Performance range**: 1.1x difference between fastest and slowest



## api_performance Results

| Benchmark | Mean Time | Ops/sec | Min | Max | Std Dev |
|-----------|-----------|---------|-----|-----|----------|
| get_user | 68.00ns | 14705882 | 40.00ns | 80.00ns | 19.00ns |
| create_user | 92.00ns | 10869565 | 40.00ns | 160.00ns | 33.00ns |

### Key Insights

- **Fastest operation**: get_user (68.00ns)
- **Performance range**: 1.4x difference between fastest and slowest