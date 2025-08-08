# benchkit

[![docs.rs](https://docs.rs/benchkit/badge.svg)](https://docs.rs/benchkit)
[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=brightgreen&logo=gitpod)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fbenchkit_trivial%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20benchkit_trivial/https://github.com/Wandalen/wTools)
[![discord](https://img.shields.io/discord/872391416519647252?color=eee&logo=discord&logoColor=eee&label=ask%20on%20discord)](https://discord.gg/m3YfbXpUUY)

Lightweight benchmarking toolkit focused on practical performance analysis and report generation. **benchkit** is a **toolkit, not a framework** - it provides flexible building blocks for creating custom benchmarking solutions without imposing rigid workflows.

## Quick Examples

### Basic Performance Measurement

```rust
use benchkit::prelude::*;

fn main()
{
  // Measure a simple operation
  let result = bench_function( "string_processing", ||
  {
    "hello world".chars().collect::<Vec<_>>()
  });

  println!( "Time: {:.2?}", result.mean_time() );
  println!( "Throughput: {:.2} ops/sec", result.operations_per_second() );
}
```

### Comparative Algorithm Analysis

```rust
use benchkit::prelude::*;

fn generate_random_vec( size : usize ) -> Vec< u32 >
{
  ( 0..size ).map( |x| x as u32 ).collect()
}

fn main()
{
  let mut comparison = ComparativeAnalysis::new( "sorting_algorithms" );

  // Compare different sorting approaches
  for size in [ 100, 1000, 10000 ]
  {
    let data = generate_random_vec( size );

    comparison = comparison.algorithm( &format!( "std_sort_{}", size ),
    {
      let mut d = data.clone();
      move ||
      {
        d.sort();
      }
    });

    comparison = comparison.algorithm( &format!( "unstable_sort_{}", size ),
    {
      let mut d = data.clone();
      move ||
      {
        d.sort_unstable();
      }
    });
  }

  let report = comparison.run();
  println!( "Fastest: {:?}", report.fastest() );
}
```

### Automatic Documentation Updates

```rust
use benchkit::prelude::*;

#[ cfg( test ) ]
mod performance_docs
{
  #[ test ]
  fn update_readme_performance()
  {
    let mut suite = BenchmarkSuite::new( "api_performance" );

    // Benchmark your API functions
    suite.benchmark( "parse_small", || parse_input( "small data" ) );
    suite.benchmark( "parse_large", || parse_input( "large data" ) );

    // Automatically update README.md performance section
    suite.generate_markdown_report()
         .update_file( "README.md", "## Performance" )
         .expect( "Failed to update documentation" );
  }
}
```

## Why benchkit Exists

### The Problem with Existing Solutions

**Criterion is great, but...**
- **Too opinionated**: Forces specific workflow and report formats
- **Complex integration**: Requires separate benchmark directory structure
- **Poor documentation integration**: Results don't easily flow into README/docs
- **Framework mentality**: You adapt to criterion, not the other way around

**DIY benchmarking has issues:**
- **Boilerplate heavy**: Same measurement/reporting code copied everywhere
- **Statistical naive**: Raw timings without proper analysis
- **Inconsistent**: Different projects use different approaches
- **Manual work**: Copy-pasting results into documentation

### The benchkit Solution

**benchkit is a toolkit, not a framework:**

✅ **Flexible Integration** - Use only the pieces you need
✅ **Markdown-First** - Designed for documentation integration
✅ **Zero Setup** - Works in any test file or binary
✅ **Statistical Sound** - Proper analysis without complexity
✅ **Composable** - Build custom workflows easily

## Core Features

### 🔧 **Toolkit Philosophy**
- **Building blocks, not walls** - Compose functionality as needed
- **Your workflow** - Integrate into existing code organization
- **Minimal assumptions** - Work with your project structure

### 📊 **Smart Analysis**
- **Statistical rigor** - Confidence intervals, outlier detection
- **Performance insights** - Automatic regression detection
- **Scaling analysis** - How performance changes with input size
- **Comparison tools** - Before/after, A/B testing made easy
- **Git-style diffing** - Compare benchmark results across commits or implementations

### 📝 **Documentation Integration**
- **Markdown-native** - Generate tables and sections directly
- **Version controlled** - Benchmark results tracked with code
- **Automatic updates** - Keep docs current with performance reality
- **Template system** - Customize report formats

### 🎯 **Practical Focus**
- **Key metrics first** - Surface what matters for optimization decisions
- **Hide complexity** - Detailed statistics available but not overwhelming
- **Actionable results** - Clear improvement/regression percentages
- **Real-world patterns** - Data generators for common scenarios

## Usage Patterns

### Pattern 1: Quick Performance Check

Perfect for ad-hoc performance analysis:

```rust
use benchkit::prelude::*;

fn old_algorithm(data: &[u32]) -> u32 {
    data.iter().sum()
}

fn new_algorithm(data: &[u32]) -> u32 {
    data.iter().fold(0, |acc, x| acc + x)
}

let data = vec![1, 2, 3, 4, 5];

// Quick check - is this optimization working?
let before = bench_once(|| old_algorithm(&data));
let after = bench_once(|| new_algorithm(&data));

let comparison = before.compare(&after);
println!("Improvement: {:.1}%", comparison.improvement_percentage);
```

### Pattern 2: Comprehensive Analysis

For thorough performance characterization:

```rust
use benchkit::prelude::*;

fn generate_test_data(size: usize) -> Vec<u32> {
    (0..size).map(|x| x as u32).collect()
}

fn run_algorithm(algorithm: &str, data: &[u32]) -> u32 {
    match algorithm {
        "baseline" => data.iter().sum(),
        "optimized" => data.iter().fold(0, |acc, x| acc + x),
        "simd" => data.iter().sum::<u32>(),
        _ => 0,
    }
}

fn analyze_performance() {
    let mut suite = BenchmarkSuite::new("comprehensive_analysis");

    // Test across multiple dimensions
    for size in [10, 100, 1000, 10000] {
        for algorithm in ["baseline", "optimized", "simd"] {
            let data = generate_test_data(size);
            let alg = algorithm.to_string();
            suite.benchmark(&format!("{}_size_{}", algorithm, size), move || {
                run_algorithm(&alg, &data);
            });
        }
    }

    let analysis = suite.run_analysis();

    // Generate comprehensive report
    let report = analysis.generate_markdown_report();
    println!("{}", report.generate());
}
```

### Pattern 3: CI/CD Integration

For continuous performance monitoring:

```rust
use benchkit::prelude::*;

#[test]
fn performance_regression_check() {
    let suite = BenchmarkSuite::from_baseline("benchmarks/baseline.json");

    suite.benchmark("critical_path", || critical_operation());

    let results = suite.run();

    // Fail CI if performance regresses significantly
    assert!(results.regression_percentage() < 10.0,
            "Performance regression detected: {:.1}%",
            results.regression_percentage());

    // Update baseline if this is main branch
    if cfg!(feature = "update_baseline") {
        results.save_as_baseline("benchmarks/baseline.json");
    }
}
```

### Pattern 4: Git-Style Performance Diffing

Compare performance across implementations or commits:

```rust,ignore
use benchkit::prelude::*;

// Baseline results (old implementation)
let baseline_results = vec![
    ("string_ops".to_string(), bench_function("old_string_ops", || old_implementation())),
    ("hash_compute".to_string(), bench_function("old_hash", || old_hash_function())),
];

// Current results (new implementation)
let current_results = vec![
    ("string_ops".to_string(), bench_function("new_string_ops", || new_implementation())),
    ("hash_compute".to_string(), bench_function("new_hash", || new_hash_function())),
];

// Generate git-style diff
let diff_set = diff_benchmark_sets(&baseline_results, &current_results);

// Show summary
println!("Performance changes:");
for diff in &diff_set.diffs {
    println!("{}", diff.to_summary());
}

// Show detailed analysis for regressions
for regression in diff_set.regressions() {
    println!("\n⚠️ Regression detected:");
    println!("{}", regression.to_diff_format());
}
```

### Pattern 5: Documentation Automation

Keep performance docs always up-to-date:

```rust
use benchkit::prelude::*;

#[cfg(test)]
mod doc_benchmarks {
    #[test]
    fn update_performance_docs() {
        // Run standard benchmark suite
        let suite = BenchmarkSuite::from_config("bench_config.toml");
        let results = suite.run_all();

        // Update multiple documentation files
        results.update_markdown_section("README.md", "## Performance")
               .update_markdown_section("docs/performance.md", "## Latest Results")
               .generate_comparison_chart("docs/performance_chart.md");
    }
}
```

## Feature Flags

benchkit uses feature flags for optional functionality:

```toml
[dependencies]
benchkit = { version = "0.1", features = ["full"] }

# Or pick specific features:
benchkit = {
    version = "0.1",
    features = [
        "markdown_reports",    # Markdown generation (default)
        "html_reports",        # HTML output
        "statistical_analysis", # Advanced statistics
        "optimization_hints",   # Performance recommendations
        "diff_analysis",        # Git-style benchmark diffing
    ]
}
```

| Feature | Description | Default |
|---------|-------------|---------|
| `enabled` | Core timing and measurement | ✓ |
| `markdown_reports` | Markdown report generation | ✓ |
| `data_generators` | Common data generation patterns | ✓ |
| `criterion_compat` | Compatibility with criterion | ✓ |
| `html_reports` | HTML report generation | - |
| `json_reports` | JSON output format | - |
| `statistical_analysis` | Advanced statistical analysis | - |
| `comparative_analysis` | A/B testing capabilities | - |
| `optimization_hints` | Performance optimization suggestions | - |
| `diff_analysis` | Git-style benchmark result diffing | - |

## When to Use benchkit vs Criterion

### Use **benchkit** when:
- ✅ You want to integrate benchmarks into existing test files
- ✅ You need automatic documentation updates
- ✅ You want flexible, composable measurement tools
- ✅ You're doing ad-hoc performance analysis
- ✅ You need before/after comparisons
- ✅ You want minimal setup overhead

### Use **criterion** when:
- ✅ You want a complete benchmarking framework
- ✅ You need sophisticated statistical analysis
- ✅ You want HTML visualization and detailed reports
- ✅ You're fine with separate benchmark organization
- ✅ You need industrial-strength benchmarking infrastructure

### Use **both** when:
- ✅ Use criterion for comprehensive benchmark suites
- ✅ Use benchkit for quick checks and documentation integration
- ✅ benchkit provides a `criterion_compat` feature for easy migration

## Installation

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
benchkit = "0.1"
```

For full functionality:

```toml
[dev-dependencies]
benchkit = { version = "0.1", features = ["full"] }
```

## Examples

See the [`examples/`](examples/) directory for complete examples:

- [`basic_usage.rs`](examples/basic_usage.rs) - Simple timing and measurement
- [`markdown_generation.rs`](examples/markdown_generation.rs) - Report generation
- [`comparative_benchmark.rs`](examples/comparative_benchmark.rs) - Algorithm comparison
- [`documentation_integration.rs`](examples/documentation_integration.rs) - Automatic doc updates

## Contributing

We welcome contributions! benchkit is designed to be a community-driven toolkit that solves real-world benchmarking problems.

### Development Philosophy

1. **Toolkit over framework** - Provide flexible building blocks
2. **Practical focus** - Solve real problems developers face
3. **Simple integration** - Minimize setup and learning curve
4. **Documentation-driven** - Make results easy to share and version

### Areas for Contribution

- **Data generators** - Common patterns for different domains
- **Analysis tools** - Statistical methods and insights
- **Report templates** - New output formats and visualizations
- **Integration examples** - Real-world usage patterns
- **Performance optimizations** - Keep the toolkit fast

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## About wTools

benchkit is part of the [wTools ecosystem](https://github.com/Wandalen/wTools) - a collection of Rust tools focused on developer productivity and performance. Check out our other tools:

- **[error_tools](https://github.com/Wandalen/wTools/tree/master/module/core/error_tools)** - Unified error handling
- **[strs_tools](https://github.com/Wandalen/wTools/tree/master/module/core/strs_tools)** - High-performance string operations
- **[unilang](https://github.com/Wandalen/wTools/tree/master/module/move/unilang)** - Universal command-line interface framework