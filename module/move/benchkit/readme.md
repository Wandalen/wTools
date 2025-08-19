# benchkit

[![docs.rs](https://docs.rs/benchkit/badge.svg)](https://docs.rs/benchkit)
[![discord](https://img.shields.io/discord/872391416519647252?color=eee&logo=discord&logoColor=eee&label=ask%20on%20discord)](https://discord.gg/m3YfbXpUUY)

**Practical, Documentation-First Benchmarking for Rust.**

`benchkit` is a lightweight toolkit for performance analysis, born from the hard-learned lessons of optimizing high-performance libraries. It rejects rigid, all-or-nothing frameworks in favor of flexible, composable tools that integrate seamlessly into your existing workflow.

> 🎯 **NEW TO benchkit?** Start with [`recommendations.md`](recommendations.md) - Essential guidelines from real-world performance optimization experience.

## The Benchmarking Dilemma

In Rust, developers often face a frustrating choice:

1.  **The Heavy Framework (`criterion`):** Statistically powerful, but forces a rigid structure (`benches/`), complex setup, and produces reports that are difficult to integrate into your project's documentation. You must adapt your project to the framework.
2.  **The Manual Approach (`std::time`):** Simple to start, but statistically naive. It leads to boilerplate, inconsistent measurements, and conclusions that are easily skewed by system noise.

`benchkit` offers a third way.

> **📋 Important**: For production use and development contributions, see [`recommendations.md`](recommendations.md) - a comprehensive guide with proven patterns, requirements, and best practices from real-world benchmarking experience.

## A Toolkit, Not a Framework

This is the core philosophy of `benchkit`. It doesn't impose a workflow; it provides a set of professional, composable tools that you can use however you see fit.

*   ✅ **Integrate Anywhere:** Write benchmarks in your test files, examples, or binaries. No required directory structure.
*   ✅ **Documentation-First:** Treat performance reports as a first-class part of your documentation, with tools to automatically keep them in sync with your code.
*   ✅ **Practical Focus:** Surface the key metrics needed for optimization decisions, hiding deep statistical complexity until you ask for it.
*   ✅ **Zero Setup:** Start measuring performance in minutes with a simple, intuitive API.

---

## 🚀 Quick Start: Compare, Analyze, and Document

**📖 First time?** Review [`recommendations.md`](recommendations.md) for comprehensive best practices and development guidelines.

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

  // Run the comparison and update readme.md
  let report = comparison.run();
  let markdown = report.to_markdown();

  let updater = MarkdownUpdater::new( "readme.md", "Benchmark Results" ).unwrap();
  updater.update_section( &markdown ).unwrap();
}
```

**3. Run your benchmark and watch readme.md update automatically:**
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

```rust,no_run
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

## 📁 Why Not `benches/`? Standard Directory Integration

The traditional `benches/` directory creates artificial separation between ALL your benchmark content and the standard Rust project structure. `benchkit` encourages you to use standard directories for ALL benchmark-related files:

- ✅ **Use `tests/`**: Performance benchmarks alongside unit tests
- ✅ **Use `examples/`**: Demonstration benchmarks and showcases  
- ✅ **Use `src/bin/`**: Dedicated benchmark executables
- ✅ **Standard integration**: Keep ALL benchmark content in standard Rust directories
- ❌ **Avoid `benches/`**: Don't isolate ANY benchmark files in framework-specific directories

### Why This Matters

**Workflow Integration**: ALL benchmark content should be part of regular development, not isolated in framework-specific directories.

**Documentation Proximity**: ALL benchmark files are documentation - keep them integrated with your standard project structure for better maintainability.

**Testing Philosophy**: Performance is part of correctness validation - integrate benchmarks with your existing test suite.

**Toolkit vs Framework**: Frameworks enforce rigid `benches/` isolation; toolkits integrate with your existing project structure.

### Automatic Documentation Updates

`benchkit` excels at maintaining comprehensive, automatically updated documentation in your project files:

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

```rust,no_run
// ✅ In standard tests/ directory alongside unit tests
// tests/performance_comparison.rs
use benchkit::prelude::*;

#[test]
fn benchmark_algorithms()
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
  
  // Automatically update readme.md with results
  let updater = MarkdownUpdater::new( "readme.md", "Performance" ).unwrap();
  updater.update_section( &results.generate_markdown_report().generate() ).unwrap();
}
```

```rust,no_run
// ✅ In examples/ directory for demonstrations
// examples/comprehensive_benchmark.rs  
use benchkit::prelude::*;

fn main()
{
  let mut comprehensive = BenchmarkSuite::new( "Comprehensive Performance Analysis" );
  
  // Add multiple benchmarks
  comprehensive.benchmark( "data_processing", || { /* code */ } );
  comprehensive.benchmark( "memory_operations", || { /* code */ } );
  comprehensive.benchmark( "io_operations", || { /* code */ } );
  
  let results = comprehensive.run_all();
  
  // Update readme.md with comprehensive report
  let report = results.generate_markdown_report();
  let updater = MarkdownUpdater::new( "readme.md", "Performance Analysis" ).unwrap();
  updater.update_section( &report.generate() ).unwrap();
  
  println!( "Updated readme.md with latest performance results" );
}
```

### 🔧 Feature Flag Recommendations

For optimal build performance and clean separation, put your benchmark code behind feature flags:

```rust,no_run
// ✅ In src/bin/ directory for dedicated benchmark executables  
// src/bin/comprehensive_benchmark.rs
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
  
  // Automatically update readme.md
  let updater = MarkdownUpdater::new( "readme.md", "Latest Results" ).unwrap();
  updater.update_section( &results.generate_markdown_report().generate() ).unwrap();
  
  println!( "Benchmarks completed - readme.md updated" );
}

#[ cfg( not( feature = "enabled" ) ) ]
fn main()
{
  println!( "Run with: cargo run --bin comprehensive_benchmark --features enabled" );
  println!( "Results will be automatically saved to readme.md" );
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

# Run specific benchmark binary (updates readme.md)
cargo run --bin comprehensive_benchmark --features enabled

# Run benchmarks from examples/
cargo run --example performance_demo --features enabled

# Run all binaries containing benchmarks
cargo run --bin performance_suite --features enabled
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

## 📋 Development Guidelines & Best Practices

**⚠️ IMPORTANT**: Before using benchkit in production or contributing to development, **strongly review** the comprehensive [`recommendations.md`](recommendations.md) file. This document contains essential requirements, best practices, and lessons learned from real-world performance analysis work.

The recommendations cover:
- ✅ **Core philosophy** and toolkit vs framework principles
- ✅ **Technical architecture** requirements and feature organization
- ✅ **Performance analysis** best practices with standardized data patterns
- ✅ **Documentation integration** requirements for automated reporting
- ✅ **Statistical analysis** requirements for reliable measurements

**📖 Read [`recommendations.md`](recommendations.md) first** - it will save you time and ensure you're following proven patterns.

## Contributing

Contributions are welcome! `benchkit` aims to be a community-driven toolkit that solves real-world benchmarking problems. 

**Before contributing:**
1. **📖 Read [`recommendations.md`](recommendations.md)** - Contains all development requirements and design principles
2. Review open tasks in the [`task/`](task/) directory  
3. Check our contribution guidelines

All contributions must align with the principles and requirements outlined in [`recommendations.md`](recommendations.md).

## License

This project is licensed under the **MIT License**.


## Performance

*This section is automatically updated by benchkit when you run benchmarks.*

