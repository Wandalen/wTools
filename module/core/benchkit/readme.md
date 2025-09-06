# benchkit

[![docs.rs](https://docs.rs/benchkit/badge.svg)](https://docs.rs/benchkit)
[![discord](https://img.shields.io/discord/872391416519647252?color=eee&logo=discord&logoColor=eee&label=ask%20on%20discord)](https://discord.gg/m3YfbXpUUY)

**Practical, Documentation-First Benchmarking for Rust.**

`benchkit` is a lightweight toolkit for performance analysis, born from the hard-learned lessons of optimizing high-performance libraries. It rejects rigid, all-or-nothing frameworks in favor of flexible, composable tools that integrate seamlessly into your existing workflow.

> 🎯 **NEW TO benchkit?** Start with [`usage.md`](usage.md) - Mandatory standards and requirements from production systems.

## The Benchmarking Dilemma

In Rust, developers often face a frustrating choice:

1.  **The Heavy Framework (`criterion`):** Statistically powerful, but forces a rigid structure (`benches/`), complex setup, and produces reports that are difficult to integrate into your project's documentation. You must adapt your project to the framework.
2.  **The Manual Approach (`std::time`):** Simple to start, but statistically naive. It leads to boilerplate, inconsistent measurements, and conclusions that are easily skewed by system noise.

`benchkit` offers a third way.

> **📋 Important**: For production use and development contributions, see [`usage.md`](usage.md) - mandatory standards with proven patterns, requirements, and compliance standards from production systems.

## A Toolkit, Not a Framework

This is the core philosophy of `benchkit`. It doesn't impose a workflow; it provides a set of professional, composable tools that you can use however you see fit.

*   ✅ **Integrate Anywhere:** Write benchmarks in your test files, examples, or binaries. No required directory structure.
*   ✅ **Documentation-First:** Treat performance reports as a first-class part of your documentation, with tools to automatically keep them in sync with your code.
*   ✅ **Practical Focus:** Surface the key metrics needed for optimization decisions, hiding deep statistical complexity until you ask for it.
*   ✅ **Zero Setup:** Start measuring performance in minutes with a simple, intuitive API.

---

## 🚀 Quick Start: Compare, Analyze, and Document

**📖 First time?** Review [`usage.md`](usage.md) for mandatory compliance standards and development requirements.

This example demonstrates the core `benchkit` workflow: comparing two algorithms and automatically updating a performance section in your `readme.md`.

**1. Add to `dev-dependencies` in `Cargo.toml`:**
```toml
[dev-dependencies]
benchkit = { version = "0.8.0", features = [ "full" ] }
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

### 🆕 Enhanced Features

<details>
<summary><strong>🔥 NEW: Comprehensive Regression Analysis System</strong></summary>

Advanced performance regression detection with statistical analysis and trend identification.

```rust
use benchkit::prelude::*;
use std::collections::HashMap;
use std::time::{ Duration, SystemTime };

fn regression_analysis_example() -> Result< (), Box< dyn std::error::Error > > {
    // Current benchmark results
    let mut current_results = HashMap::new();
    let current_times = vec![ Duration::from_micros( 85 ), Duration::from_micros( 88 ), Duration::from_micros( 82 ) ];
    current_results.insert( "fast_sort".to_string(), BenchmarkResult::new( "fast_sort", current_times ) );
    
    // Historical baseline data
    let mut baseline_data = HashMap::new();
    let baseline_times = vec![ Duration::from_micros( 110 ), Duration::from_micros( 115 ), Duration::from_micros( 108 ) ];
    baseline_data.insert( "fast_sort".to_string(), BenchmarkResult::new( "fast_sort", baseline_times ) );
    
    let historical = HistoricalResults::new().with_baseline( baseline_data );
    
    // Configure regression analyzer
    let analyzer = RegressionAnalyzer::new()
        .with_baseline_strategy( BaselineStrategy::FixedBaseline )
        .with_significance_threshold( 0.05 )  // 5% significance level
        .with_trend_window( 5 );
    
    // Perform regression analysis
    let regression_report = analyzer.analyze( &current_results, &historical );
    
    // Check results
    if regression_report.has_significant_changes() {
        println!( "📊 Significant performance changes detected!" );
        
        if let Some( trend ) = regression_report.get_trend_for( "fast_sort" ) {
            match trend {
                PerformanceTrend::Improving => println!( "🟢 Performance improved!" ),
                PerformanceTrend::Degrading => println!( "🔴 Performance regression detected!" ),
                PerformanceTrend::Stable => println!( "🟡 Performance remains stable" ),
            }
        }
        
        // Generate professional markdown report
        let markdown_report = regression_report.format_markdown();
        println!( "{}", markdown_report );
    }
    
    Ok(())
}
```

**Key Features:**
- **Three Baseline Strategies**: Fixed baseline, rolling average, and previous run comparison
- **Statistical Significance**: Configurable thresholds with proper statistical testing
- **Trend Detection**: Automatic identification of improving, degrading, or stable performance
- **Professional Reports**: Publication-quality markdown with statistical analysis
- **CI/CD Integration**: Automated regression detection for deployment pipelines
- **Historical Data Management**: Long-term performance tracking with quality validation

**Use Cases:**
- Automated performance regression detection in CI/CD pipelines
- Long-term performance monitoring and trend analysis
- Code optimization validation with statistical confidence
- Production deployment gates with zero-regression tolerance
- Performance documentation with automated updates

</details>

<details>
<summary><strong>Safe Update Chain Pattern - Atomic Documentation Updates</strong></summary>

Coordinate multiple markdown section updates atomically - either all succeed or none are modified.

```rust
use benchkit::prelude::*;

fn update_markdown_atomically() -> Result< (), Box< dyn std::error::Error > > {
    let performance_markdown = "## Performance Results\n\nFast!";
    let memory_markdown = "## Memory Usage\n\nLow!";
    let cpu_markdown = "## CPU Usage\n\nOptimal!";
    
    // Update multiple sections atomically
    let chain = MarkdownUpdateChain::new("readme.md")?
        .add_section("Performance Benchmarks", performance_markdown)
        .add_section("Memory Analysis", memory_markdown)
        .add_section("CPU Profiling", cpu_markdown);

    // Validate all sections before any updates
    let conflicts = chain.check_all_conflicts()?;
    if !conflicts.is_empty() {
        return Err(format!("Section conflicts detected: {:?}", conflicts).into());
    }

    // Atomic update - either all succeed or all fail
    chain.execute()?;
    Ok(())
}
```

**Key Features:**
- **Atomic Operations**: Either all sections update successfully or none are modified
- **Conflict Detection**: Validates all sections exist and are unambiguous before any changes
- **Automatic Rollback**: Failed operations restore original file state
- **Reduced I/O**: Single read and write operation instead of multiple file accesses
- **Error Recovery**: Comprehensive error handling with detailed diagnostics

**Use Cases:**
- Multi-section benchmark reports that must stay synchronized
- CI/CD pipelines requiring consistent documentation updates
- Coordinated updates across large documentation projects
- Production deployments where partial updates would be problematic

**Advanced Example:**
```rust
use benchkit::prelude::*;

fn complex_update_example() -> Result< (), Box< dyn std::error::Error > > {
    let performance_report = "Performance analysis results";
    let memory_report = "Memory usage analysis";
    let comparison_report = "Algorithm comparison data";
    let validation_report = "Quality assessment report";
    
    // Complex coordinated update across multiple report types
    let chain = MarkdownUpdateChain::new("PROJECT_BENCHMARKS.md")?
        .add_section("Performance Analysis", performance_report)
        .add_section("Memory Usage Analysis", memory_report)
        .add_section("Algorithm Comparison", comparison_report)
        .add_section("Quality Assessment", validation_report);

    // Validate everything before committing any changes
    match chain.check_all_conflicts() {
        Ok(conflicts) if conflicts.is_empty() => {
            println!("✅ All {} sections validated", chain.len());
            chain.execute()?;
        },
        Ok(conflicts) => {
            eprintln!("⚠️ Conflicts: {:?}", conflicts);
            // Handle conflicts or use more specific section names
        },
        Err(e) => eprintln!("❌ Validation failed: {}", e),
    }
    Ok(())
}
```

</details>

<details>
<summary><strong>Professional Report Templates - Research-Grade Documentation</strong></summary>

Generate standardized, publication-quality reports with full statistical analysis and customizable sections.

```rust
use benchkit::prelude::*;
use std::collections::HashMap;

fn generate_reports() -> Result< (), Box< dyn std::error::Error > > {
    let results = HashMap::new();
    let comparison_results = HashMap::new();
    
    // Comprehensive performance analysis
    let performance_template = PerformanceReport::new()
        .title("Algorithm Performance Analysis")
        .add_context("Comparing sequential vs parallel processing approaches")
        .include_statistical_analysis(true)
        .include_regression_analysis(true)
        .add_custom_section(CustomSection::new(
            "Implementation Notes", 
            "Detailed implementation considerations and optimizations applied"
        ));

    let performance_report = performance_template.generate(&results)?;

    // A/B testing comparison with statistical significance
    let comparison_template = ComparisonReport::new()
        .title("Sequential vs Parallel Processing Comparison")
        .baseline("Sequential Processing")
        .candidate("Parallel Processing") 
        .significance_threshold(0.01)     // 1% statistical significance
        .practical_significance_threshold(0.05);  // 5% practical significance

    let comparison_report = comparison_template.generate(&comparison_results)?;
    Ok(())
}
```

**Performance Report Features:**
- **Executive Summary**: Key metrics and performance indicators
- **Statistical Analysis**: Confidence intervals, coefficient of variation, reliability assessment
- **Performance Tables**: Sorted results with throughput, latency, and quality indicators
- **Custom Sections**: Domain-specific analysis and recommendations
- **Professional Formatting**: Publication-ready markdown with proper statistical notation

**Comparison Report Features:**
- **Significance Testing**: Both statistical and practical significance analysis
- **Confidence Intervals**: 95% CI analysis with overlap detection
- **Performance Ratios**: Clear improvement/regression percentages
- **Reliability Assessment**: Quality validation for both baseline and candidate
- **Decision Support**: Clear recommendations based on statistical analysis

**Advanced Template Composition:**
```rust
use benchkit::prelude::*;

fn create_enterprise_template() -> PerformanceReport {
    // Create domain-specific template with multiple custom sections
    let enterprise_template = PerformanceReport::new()
        .title("Enterprise Algorithm Performance Audit")
        .add_context("Monthly performance review for production trading systems")
        .include_statistical_analysis(true)
        .add_custom_section(CustomSection::new(
            "Risk Assessment",
            r#"### Performance Risk Analysis
            
    | Algorithm | Latency Risk | Throughput Risk | Stability | Overall |
    |-----------|-------------|-----------------|-----------|----------|
    | Current   | 🟢 Low     | 🟡 Medium      | 🟢 Low   | 🟡 Medium |
    | Proposed  | 🟢 Low     | 🟢 Low        | 🟢 Low   | 🟢 Low    |"#
        ))
        .add_custom_section(CustomSection::new(
            "Business Impact",
            r#"### Projected Business Impact

    - **Latency Improvement**: 15% faster response times
    - **Throughput Increase**: +2,000 req/sec capacity
    - **Cost Reduction**: -$50K/month in infrastructure
    - **SLA Compliance**: 99.9% → 99.99% uptime"#
        ));
    enterprise_template
}
```

</details>

<details>
<summary><strong>Benchmark Validation Framework - Quality Assurance</strong></summary>

Comprehensive quality assessment system with configurable criteria and automatic reliability analysis.

```rust
use benchkit::prelude::*;
use std::collections::HashMap;

fn validate_benchmark_results() {
    let results = HashMap::new();
    
    // Configure validator for your specific requirements
    let validator = BenchmarkValidator::new()
        .min_samples(20)                           // Require 20+ measurements
        .max_coefficient_variation(0.10)           // 10% maximum variability
        .require_warmup(true)                      // Detect warm-up periods
        .max_time_ratio(3.0)                       // 3x max/min ratio
        .min_measurement_time(Duration::from_micros(50)); // 50μs minimum duration

    // Validate all results with detailed analysis
    let validated_results = ValidatedResults::new(results, validator);

    println!("Reliability: {:.1}%", validated_results.reliability_rate());

    // Get detailed quality warnings
    if let Some(warnings) = validated_results.reliability_warnings() {
        println!("⚠️ Quality Issues Detected:");
        for warning in warnings {
            println!("  - {}", warning);
        }
    }

    // Work with only statistically reliable results
    let reliable_only = validated_results.reliable_results();
    println!("Using {}/{} reliable benchmarks for analysis", 
             reliable_only.len(), validated_results.results.len());
}
```

**Validation Criteria:**
- **Sample Size**: Ensure sufficient measurements for statistical power
- **Variability**: Detect high coefficient of variation indicating noise
- **Measurement Duration**: Flag measurements that may be timing-resolution limited
- **Performance Range**: Identify outliers and wide performance distributions  
- **Warm-up Detection**: Verify proper system warm-up for consistent results

**Warning Types:**
- `InsufficientSamples`: Too few measurements for reliable statistics
- `HighVariability`: Coefficient of variation exceeds threshold
- `ShortMeasurementTime`: Measurements may be affected by timer resolution
- `WidePerformanceRange`: Large ratio between fastest/slowest measurements
- `NoWarmup`: Missing warm-up period may indicate measurement issues

**Domain-Specific Validation:**
```rust
use benchkit::prelude::*;
use std::collections::HashMap;

fn domain_specific_validation() {
    let results = HashMap::new();
    
    // Real-time systems validation (very strict)
    let realtime_validator = BenchmarkValidator::new()
        .min_samples(50)
        .max_coefficient_variation(0.02)  // 2% maximum
        .max_time_ratio(1.5);             // Very tight timing

    // Interactive systems validation (balanced)  
    let interactive_validator = BenchmarkValidator::new()
        .min_samples(15)
        .max_coefficient_variation(0.15)  // 15% acceptable
        .require_warmup(false);           // Interactive may not show warmup

    // Batch processing validation (lenient)
    let batch_validator = BenchmarkValidator::new()
        .min_samples(10)
        .max_coefficient_variation(0.25)  // 25% acceptable  
        .max_time_ratio(5.0);             // Allow more variation

    // Apply appropriate validator for your domain
    let domain_results = ValidatedResults::new(results, realtime_validator);
}
```

**Quality Reporting:**
```rust
use benchkit::prelude::*;
use std::collections::HashMap;

fn generate_validation_report() {
    let results = HashMap::new();
    let validator = BenchmarkValidator::new();
    
    // Generate comprehensive validation report
    let validation_report = validator.generate_validation_report(&results);

    // Validation report includes:
    // - Summary statistics and reliability rates
    // - Detailed warnings with improvement recommendations  
    // - Validation criteria documentation
    // - Quality assessment for each benchmark
    // - Actionable steps to improve measurement quality

    println!("{}", validation_report);
}
```

</details>

<details>
<summary><strong>Complete Integration Examples</strong></summary>

Comprehensive examples demonstrating real-world usage patterns and advanced integration scenarios.

**Development Workflow Integration:**
```rust
use benchkit::prelude::*;

// Complete development cycle: benchmark → validate → document → commit
fn development_workflow() -> Result< (), Box< dyn std::error::Error > > {
    // Mock implementations for doc test
    fn quicksort_implementation() {}
    fn mergesort_implementation() {}
    
    // 1. Run benchmarks
    let mut suite = BenchmarkSuite::new("Algorithm Performance");
    suite.benchmark("quicksort", || quicksort_implementation());
    suite.benchmark("mergesort", || mergesort_implementation());
    let results = suite.run_all();
    
    // 2. Validate quality
    let validator = BenchmarkValidator::new()
        .min_samples(15)
        .max_coefficient_variation(0.15);
    let validated_results = ValidatedResults::new(results.results, validator);
    
    if validated_results.reliability_rate() < 80.0 {
        return Err("Benchmark quality insufficient for analysis".into());
    }
    
    // 3. Generate professional report
    let template = PerformanceReport::new()
        .title("Algorithm Performance Analysis")
        .include_statistical_analysis(true)
        .add_custom_section(CustomSection::new(
            "Development Notes",
            "Analysis conducted during algorithm optimization phase"
        ));
    
    let report = template.generate(&validated_results.results)?;
    
    // 4. Update documentation atomically
    let chain = MarkdownUpdateChain::new("README.md")?
        .add_section("Performance Analysis", report)
        .add_section("Quality Assessment", validated_results.validation_report());
    
    chain.execute()?;
    println!("✅ Development documentation updated successfully");
    
    Ok(())
}
```

**CI/CD Pipeline Integration:**
```rust
use benchkit::prelude::*;
use std::collections::HashMap;

// Automated performance regression detection
fn cicd_performance_check(baseline_results: HashMap<String, BenchmarkResult>, 
                          pr_results: HashMap<String, BenchmarkResult>) -> Result< bool, Box< dyn std::error::Error > > {
    // Validate both result sets
    let validator = BenchmarkValidator::new().require_warmup(false);
    let baseline_validated = ValidatedResults::new(baseline_results.clone(), validator.clone());
    let pr_validated = ValidatedResults::new(pr_results.clone(), validator);
    
    // Require high quality for regression analysis
    if baseline_validated.reliability_rate() < 90.0 || pr_validated.reliability_rate() < 90.0 {
        println!("❌ BLOCK: Insufficient benchmark quality for regression analysis");
        return Ok(false);
    }
    
    // Compare performance for regression detection
    let comparison = ComparisonReport::new()
        .title("Performance Regression Analysis")
        .baseline("baseline_version")
        .candidate("pr_version")
        .practical_significance_threshold(0.05);  // 5% regression threshold
    
    // Create combined results for comparison
    let mut combined = HashMap::new();
    combined.insert("baseline_version".to_string(), 
                   baseline_results.values().next().unwrap().clone());
    combined.insert("pr_version".to_string(), 
                   pr_results.values().next().unwrap().clone());
    
    let regression_report = comparison.generate(&combined)?;
    
    // Check for regressions
    let has_regression = regression_report.contains("slower");
    
    if has_regression {
        println!("❌ BLOCK: Performance regression detected");
        // Save detailed report for review
        std::fs::write("regression_analysis.md", regression_report)?;
        Ok(false)
    } else {
        println!("✅ ALLOW: No performance regressions detected");
        Ok(true)
    }
}
```

**Multi-Project Coordination:**
```rust
use benchkit::prelude::*;
use std::collections::HashMap;

// Coordinate benchmark updates across multiple related projects
fn coordinate_multi_project_benchmarks() -> Result< (), Box< dyn std::error::Error > > {
    let projects = vec!["web-api", "batch-processor", "realtime-analyzer"];
    let mut all_results = HashMap::new();
    
    // Collect results from all projects  
    for project in &projects {
        let project_results = run_project_benchmarks(project)?;
        all_results.extend(project_results);
    }
    
    // Cross-project validation with lenient criteria
    let validator = BenchmarkValidator::new()
        .max_coefficient_variation(0.25)  // Different environments have more noise
        .require_warmup(false);
    
    let cross_project_validated = ValidatedResults::new(all_results.clone(), validator);
    
    // Generate consolidated impact analysis
    let impact_template = PerformanceReport::new()
        .title("Cross-Project Performance Impact Analysis")
        .add_context("Shared library upgrade impact across all dependent projects")
        .include_statistical_analysis(true)
        .add_custom_section(CustomSection::new(
            "Project Impact Summary",
            format_project_impact_analysis(&projects, &all_results)
        ));
    
    let impact_report = impact_template.generate(&all_results)?;
    
    // Update shared documentation
    let shared_chain = MarkdownUpdateChain::new("SHARED_LIBRARY_IMPACT.md")?
        .add_section("Current Impact Analysis", &impact_report)
        .add_section("Quality Assessment", &cross_project_validated.validation_report());
    
    shared_chain.execute()?;
    
    // Notify project maintainers
    notify_project_teams(&projects, &impact_report)?;
    
    Ok(())
}

// Helper functions for the example
fn run_project_benchmarks(_project: &str) -> Result< HashMap< String, BenchmarkResult >, Box< dyn std::error::Error > > {
    // Mock implementation for doc test
    Ok(HashMap::new())
}

fn format_project_impact_analysis(_projects: &[&str], _results: &HashMap< String, BenchmarkResult >) -> String {
    // Mock implementation for doc test  
    "Impact analysis summary".to_string()
}

fn notify_project_teams(_projects: &[&str], _report: &str) -> Result< (), Box< dyn std::error::Error > > {
    // Mock implementation for doc test
    Ok(())
}
```

</details>

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

## 📁 MANDATORY `benches/` Directory - NO ALTERNATIVES

**ABSOLUTE REQUIREMENT**: ALL benchmark-related files MUST be in the `benches/` directory. This is NON-NEGOTIABLE for proper benchkit functionality:

- 🚫 **NEVER in `tests/`**: Benchmarks are NOT tests and MUST NOT be mixed with unit tests
- 🚫 **NEVER in `examples/`**: Examples are demonstrations, NOT performance measurements  
- 🚫 **NEVER in `src/bin/`**: Source binaries are NOT benchmarks
- ✅ **ONLY in `benches/`**: This is the EXCLUSIVE location for ALL benchmark content

**Why This Requirement Exists:**

- ⚡ **Cargo Requirement**: `cargo bench` ONLY works with `benches/` directory
- 🏗️ **Ecosystem Standard**: ALL professional Rust projects use `benches/` EXCLUSIVELY
- 🔧 **Tool Compatibility**: IDEs, CI systems, linters expect benchmarks ONLY in `benches/`
- 📊 **Performance Isolation**: Benchmarks require different compilation and execution than tests

### Why This Matters

**Ecosystem Integration**: The `benches/` directory is the official Rust standard, ensuring compatibility with the entire Rust toolchain.

**Zero Configuration**: `cargo bench` automatically discovers and runs benchmarks in the `benches/` directory without additional setup.

**Community Expectations**: Developers expect to find benchmarks in `benches/` - this follows the principle of least surprise.

**Tool Compatibility**: All Rust tooling (IDEs, CI/CD, linters) is designed around the standard `benches/` structure.

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
benchkit = { version = "0.8.0", features = ["full"], optional = true }
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

## 📚 Comprehensive Examples

`benchkit` includes extensive examples demonstrating every feature and usage pattern:

### 🎯 Feature-Specific Examples

- **[Update Chain Comprehensive](examples/update_chain_comprehensive.rs)**: Complete demonstration of atomic documentation updates
  - Single and multi-section updates with conflict detection  
  - Error handling and recovery patterns
  - Advanced conflict resolution strategies
  - Performance optimization for bulk updates
  - Full integration with validation and templates

- **[Templates Comprehensive](examples/templates_comprehensive.rs)**: Professional report generation in all scenarios
  - Basic and fully customized Performance Report templates
  - A/B testing with Comparison Report templates  
  - Custom sections with advanced markdown formatting
  - Multiple comparison scenarios and batch processing
  - Business impact analysis and risk assessment templates
  - Comprehensive error handling for edge cases

- **[Validation Comprehensive](examples/validation_comprehensive.rs)**: Quality assurance for reliable benchmarking
  - Default and custom validator configurations
  - Individual warning types with detailed analysis
  - Validation report generation and interpretation
  - Reliable results filtering for analysis
  - Domain-specific validation scenarios (research, development, production, micro)
  - Full integration with templates and update chains

- **[Regression Analysis Comprehensive](examples/regression_analysis_comprehensive.rs)**: Complete regression analysis system demonstration
  - All baseline strategies (Fixed, Rolling Average, Previous Run)
  - Performance trend detection (Improving, Degrading, Stable)
  - Statistical significance testing with configurable thresholds
  - Professional markdown report generation with regression insights
  - Real-world optimization scenarios and configuration guidance
  - Full integration with PerformanceReport templates

- **[Historical Data Management](examples/historical_data_management.rs)**: Managing long-term performance data
  - Incremental historical data building and TimestampedResults creation
  - Data quality validation and cleanup procedures
  - Performance trend analysis across multiple time windows
  - Storage and serialization strategy recommendations
  - Data retention and archival best practices
  - Integration with RegressionAnalyzer for trend detection

### 🔧 Integration Examples

- **[Integration Workflows](examples/integration_workflows.rs)**: Real-world workflow automation
  - Development cycle: benchmark → validate → document → commit
  - CI/CD pipeline: regression detection → merge decision → automated reporting
  - Multi-project coordination: impact analysis → consolidated reporting → team alignment
  - Production monitoring: continuous tracking → alerting → dashboard updates

- **[Error Handling Patterns](examples/error_handling_patterns.rs)**: Robust operation under adverse conditions
  - Update Chain file system errors (permissions, conflicts, recovery)
  - Template generation errors (missing data, invalid parameters)
  - Validation framework edge cases (malformed data, extreme variance)
  - System errors (resource limits, concurrent access)
  - Graceful degradation strategies with automatic fallbacks

- **[Advanced Usage Patterns](examples/advanced_usage_patterns.rs)**: Enterprise-scale benchmarking
  - Domain-specific validation criteria (real-time, interactive, batch processing)
  - Template composition and inheritance patterns
  - Coordinated multi-document updates with consistency guarantees
  - Memory-efficient large-scale processing (1000+ algorithms)
  - Performance optimization techniques (caching, concurrency, incremental processing)

- **[CI/CD Regression Detection](examples/cicd_regression_detection.rs)**: Automated performance validation in CI/CD pipelines
  - Multi-environment validation (development, staging, production)
  - Configurable regression thresholds and statistical significance levels
  - Automated performance gate decisions with proper exit codes
  - GitHub Actions compatible reporting and documentation updates
  - Progressive validation pipeline with halt-on-failure
  - Real-world CI/CD integration patterns and best practices

- **🚨 [Cargo Bench Integration](examples/cargo_bench_integration.rs)**: CRITICAL - Standard `cargo bench` integration patterns
  - Seamless integration with Rust's standard `cargo bench` command
  - Automatic documentation updates during benchmark execution
  - Standard `benches/` directory structure support
  - Criterion compatibility layer for zero-migration adoption
  - CI/CD integration with standard workflows and conventions
  - Real-world project structure and configuration examples
  - **This is the foundation requirement for benchkit adoption**

### 🚀 Running the Examples

```bash
# Feature-specific examples
cargo run --example update_chain_comprehensive --all-features
cargo run --example templates_comprehensive --all-features  
cargo run --example validation_comprehensive --all-features

# NEW: Regression Analysis Examples
cargo run --example regression_analysis_comprehensive --all-features
cargo run --example historical_data_management --all-features

# Integration examples
cargo run --example integration_workflows --all-features
cargo run --example error_handling_patterns --all-features
cargo run --example advanced_usage_patterns --all-features

# NEW: CI/CD Integration Example
cargo run --example cicd_regression_detection --all-features

# 🚨 CRITICAL: Cargo Bench Integration Example
cargo run --example cargo_bench_integration --all-features

# Original enhanced features demo
cargo run --example enhanced_features_demo --all-features
```

Each example is fully documented with detailed explanations and demonstrates production-ready patterns you can adapt to your specific needs.

## Installation

Add `benchkit` to your `[dev-dependencies]` in `Cargo.toml`.

```toml
[dev-dependencies]
# For core functionality
benchkit = "0.1"

# Or enable all features for the full toolkit
benchkit = { version = "0.8.0", features = [ "full" ] }
```

## 📋 Development Guidelines & Best Practices

**⚠️ IMPORTANT**: Before using benchkit in production or contributing to development, **strongly review** the comprehensive [`usage.md`](usage.md) file. This document contains essential requirements, best practices, and lessons learned from real-world performance analysis work.

The recommendations cover:
- ✅ **Core philosophy** and toolkit vs framework principles
- ✅ **Technical architecture** requirements and feature organization
- ✅ **Performance analysis** best practices with standardized data patterns
- ✅ **Documentation integration** requirements for automated reporting
- ✅ **Statistical analysis** requirements for reliable measurements

**📖 Read [`usage.md`](usage.md) first** - it will save you time and ensure you're following proven patterns.

## Contributing

Contributions are welcome! `benchkit` aims to be a community-driven toolkit that solves real-world benchmarking problems. 

**Before contributing:**
1. **📖 Read [`usage.md`](usage.md)** - Contains all development requirements and design principles
2. Review open tasks in the [`task/`](task/) directory  
3. Check our contribution guidelines

All contributions must align with the principles and requirements outlined in [`usage.md`](usage.md).

## License

This project is licensed under the **MIT License**.


## Performance

*This section is automatically updated by benchkit when you run benchmarks.*

