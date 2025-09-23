//! Cargo Bench Integration Example
//!
//! This example demonstrates EXACTLY how benchkit should integrate with `cargo bench` :
//! - Standard `benches/` directory structure usage
//! - Automatic documentation updates during benchmarks
//! - Regression analysis integration with cargo bench
//! - Criterion compatibility for migration scenarios
//! - Production-ready patterns for real-world adoption

#![ cfg( feature = "enabled" ) ]
#![ cfg( feature = "markdown_reports" ) ]
#![ allow( clippy ::uninlined_format_args ) ]
#![ allow( clippy ::format_push_string ) ]
#![ allow( clippy ::cast_lossless ) ]
#![ allow( clippy ::cast_possible_truncation ) ]
#![ allow( clippy ::cast_precision_loss ) ]
#![ allow( clippy ::std_instead_of_core ) ]
#![ allow( clippy ::needless_raw_string_hashes ) ]
#![ allow( clippy ::too_many_lines ) ]

use benchkit ::prelude :: *;

/// Simulate algorithm implementations for benchmarking
mod algorithms 
{
  use std ::time ::Duration;
  
  pub fn quicksort_implementation()
  {
  // Simulate quicksort work
  std ::thread ::sleep(Duration ::from_micros(95));
 }
  
  pub fn mergesort_implementation()
  {
  // Simulate mergesort work  
  std ::thread ::sleep(Duration ::from_micros(110));
 }
  
  pub fn heapsort_implementation()
  {
  // Simulate heapsort work
  std ::thread ::sleep(Duration ::from_micros(135));
 }
  
  pub fn bubblesort_implementation()
  {
  // Simulate bubblesort work (intentionally slow)
  std ::thread ::sleep(Duration ::from_micros(2500));
 }
}

/// Demonstrate the IDEAL cargo bench integration pattern
/// 
/// This is how a typical `benches/performance_suite.rs` file should look
/// when using benchkit with cargo bench integration.
fn demonstrate_ideal_cargo_bench_pattern() 
{
  println!("🚀 IDEAL CARGO BENCH INTEGRATION PATTERN");
  println!("========================================");
  println!("This demonstrates how benchkit should work with `cargo bench` : \n");
  
  // STEP 1 : Standard benchmark suite creation
  println!("📊 1. Creating benchmark suite (just like criterion) : ");
  let mut suite = BenchmarkSuite ::new("Algorithm Performance Suite");
  
  // Add benchmarks using the standard pattern
  suite.benchmark("quicksort", algorithms ::quicksort_implementation);
  suite.benchmark("mergesort", algorithms ::mergesort_implementation);  
  suite.benchmark("heapsort", algorithms ::heapsort_implementation);
  suite.benchmark("bubblesort", algorithms ::bubblesort_implementation);
  
  println!("   ✅ Added 4 benchmarks to suite");
  
  // STEP 2 : Run benchmarks (this happens during `cargo bench`)
  println!("\n📈 2. Running benchmarks (cargo bench execution) : ");
  let results = suite.run_all();
  println!("   ✅ Completed {} benchmark runs", results.results.len());
  
  // STEP 3 : Automatic documentation updates (CRITICAL FEATURE)
  println!("\n📝 3. Automatic documentation updates: ");
  
  // Generate performance markdown
  let performance_template = PerformanceReport ::new()
  .title("Algorithm Performance Benchmark Results")
  .add_context("Comprehensive comparison of sorting algorithms")
  .include_statistical_analysis(true)
  .include_regression_analysis(false); // No historical data for this example
  
  match performance_template.generate(&results.results) 
  {
  Ok(performance_report) =>
  {
  println!("   ✅ Generated performance report ({} chars)", performance_report.len());
  
  // Simulate updating README.md (this should happen automatically)
  println!("   📄 Would update README.md section: ## Performance");
  println!("   📄 Would update PERFORMANCE.md section: ## Latest Results");
  
  // Show what the markdown would look like
  println!("\n📋 EXAMPLE GENERATED MARKDOWN: ");
  println!("------------------------------");
  let lines: Vec< &str > = performance_report.lines().take(15).collect();
  for line in lines 
  {
  println!("{}", line);
 }
  println!("... (truncated for demonstration)");
 },
  Err(e) =>
  {
  println!("   ❌ Failed to generate report: {}", e);
 }
 }
  
  // STEP 4 : Regression analysis (if historical data available)
  println!("\n🔍 4. Regression analysis (with historical data) : ");
  println!("   📊 Would load historical performance data");
  println!("   📈 Would detect performance trends");
  println!("   🚨 Would alert on regressions > 5%");
  println!("   📝 Would update regression analysis documentation");
  
  println!("\n✅ Cargo bench integration complete!");
}

/// Demonstrate criterion compatibility and migration patterns
fn demonstrate_criterion_compatibility() 
{
  println!("\n🔄 CRITERION COMPATIBILITY DEMONSTRATION");
  println!("=======================================");
  println!("Showing how benchkit should provide smooth migration from criterion: \n");
  
  println!("📋 ORIGINAL CRITERION CODE: ");
  println!("---------------------------");
  println!(r#"
// Before: criterion benchmark
use criterion :: {{ black_box, criterion_group, criterion_main, Criterion }};

fn quicksort_benchmark(c: &mut Criterion) 
{{
  c.bench_function("quicksort", |b| b.iter(|| quicksort_implementation()));
}}

criterion_group!(benches, quicksort_benchmark);
criterion_main!(benches);
"#);
  
  println!("📋 AFTER: BENCHKIT WITH CRITERION COMPATIBILITY: ");
  println!("-----------------------------------------------");
  println!("// After: benchkit with criterion compatibility layer");
  println!("use benchkit ::prelude :: *;");
  println!("use benchkit ::criterion_compat :: {{criterion_group, criterion_main, Criterion }};");
  println!();
  println!("fn quicksort_benchmark(c: &mut Criterion) {{");
  println!("    c.bench_function(\"quicksort\", |b| b.iter(|| quicksort_implementation()));");
  println!("}}");
  println!();
  println!("// SAME API - zero migration effort!");
  println!("criterion_group!(benches, quicksort_benchmark);");
  println!("criterion_main!(benches);");
  println!();
  println!("// But now with automatic documentation updates and regression analysis!");
  
  println!("✅ Migration requires ZERO code changes with compatibility layer!");
  
  println!("\n📋 PURE BENCHKIT PATTERN (RECOMMENDED) : ");
  println!("--------------------------------------");
  println!("// Pure benchkit pattern - cleaner and more powerful");
  println!("use benchkit ::prelude :: *;");
  println!();
  println!("fn main() {{");
  println!("    let mut suite = BenchmarkSuite ::new(\"Algorithm Performance\");");
  println!("    ");
  println!("    suite.benchmark(\"quicksort\", || quicksort_implementation());");
  println!("    suite.benchmark(\"mergesort\", || mergesort_implementation());");
  println!("    ");
  println!("    // Automatically update documentation during cargo bench");
  println!("    let results = suite.run_with_auto_docs(&[ ");
  println!("        (\"README.md\", \"Performance Results\"),");
  println!("        (\"PERFORMANCE.md\", \"Latest Results\"),");
  println!(" ]);");
  println!("    ");
  println!("    // Automatic regression analysis");
  println!("    results.check_regressions_and_update_docs();");
  println!("}}");
  
  println!("✅ Pure benchkit pattern provides enhanced functionality!");
}

/// Demonstrate CI/CD integration patterns
fn demonstrate_cicd_integration() 
{
  println!("\n🏗️ CI/CD INTEGRATION DEMONSTRATION");
  println!("==================================");
  println!("How benchkit should integrate with CI/CD pipelines: \n");
  
  println!("📋 GITHUB ACTIONS WORKFLOW: ");
  println!("---------------------------");
  println!(r#"
name: Performance Benchmarks

on :
  push :
  branches: [ main ]
  pull_request :
  branches: [ main ]

jobs :
  benchmarks :
  runs-on: ubuntu-latest
  steps :
  - uses: actions/checkout@v3
  
  - name: Setup Rust
   uses: actions-rs/toolchain@v1
   with :
  toolchain: stable
  
  # This should work out of the box!
  - name: Run benchmarks and update docs
   run: cargo bench
  
  # Documentation is automatically updated by benchkit
  - name: Commit updated documentation
   run: |
  git config --local user.email "action@github.com"
  git config --local user.name "GitHub Action"
  git add README.md PERFORMANCE.md
  git commit -m "docs: Update performance benchmarks" || exit 0
  git push
"#);
  
  println!("📋 REGRESSION DETECTION IN CI: ");
  println!("------------------------------");
  println!("   🚨 Benchkit should automatically: ");
  println!("   - Compare PR performance against main branch");
  println!("   - Block PRs with >5% performance regressions");
  println!("   - Generate regression reports in PR comments");
  println!("   - Update performance documentation automatically");
  
  println!("\n📋 MULTI-ENVIRONMENT SUPPORT: ");
  println!("-----------------------------");
  println!("   🌍 Different thresholds per environment: ");
  println!("   - Development: Lenient (15% regression allowed)");
  println!("   - Staging: Moderate (10% regression allowed)");
  println!("   - Production: Strict (5% regression allowed)");
  
  println!("\n✅ Zero additional CI/CD configuration required!");
}

/// Demonstrate real-world directory structure and file organization
fn demonstrate_project_structure() 
{
  println!("\n📁 REAL-WORLD PROJECT STRUCTURE");
  println!("===============================");
  println!("How benchkit should integrate into typical Rust projects: \n");
  
  println!("📂 STANDARD RUST PROJECT LAYOUT: ");
  println!("--------------------------------");
  println!(r#"
my_rust_project/
├── Cargo.toml                 # Standard Rust project
├── README.md                  # Auto-updated with performance results
├── PERFORMANCE.md            # Detailed performance documentation
├── src/
│   ├── lib.rs
│   ├── algorithms.rs         # Code being benchmarked
│   └── utils.rs
├── tests/                    # Unit tests (unchanged)
│   └── integration_tests.rs
├── benches/                  # Standard Rust benchmark directory
│   ├── performance_suite.rs  # Main benchmark suite
│   ├── algorithm_comparison.rs  # Specific comparisons
│   ├── regression_tracking.rs   # Historical tracking
│   └── memory_benchmarks.rs     # Memory usage benchmarks
├── docs/
│   └── performance/          # Extended performance docs
│       ├── methodology.md
│       ├── historical_data.md
│       └── optimization_guide.md
└── .benchkit/               # Benchkit data directory
  ├── historical_data.json  # Performance history
  ├── baselines.json        # Regression baselines
  └── config.toml          # Benchkit configuration
"#);
  
  println!("📋 CARGO.TOML CONFIGURATION: ");
  println!("----------------------------");
  println!(r#"
[package]
name = "my_rust_project"
version = "0.8.0"

# Standard Rust benchmark configuration
[[bench]]
name = "performance_suite"
harness = false

[[bench]]
name = "algorithm_comparison"
harness = false

[dev-dependencies]
benchkit = {{ version = "0.8.0", features = ["cargo_bench", "regression_analysis"] }}

[features]
# Optional: allow disabling benchmarks in some environments
benchmarks = ["benchkit"]
"#);
  
  println!("📋 EXAMPLE BENCHMARK FILE (benches/performance_suite.rs) : ");
  println!("---------------------------------------------------------");
  println!("use benchkit ::prelude :: *;");
  println!("use my_rust_project ::algorithms :: *;");
  println!();
  println!("fn main() -> Result< (), Box<dyn std ::error ::Error >> {{");
  println!("    let mut suite = BenchmarkSuite ::new(\"Algorithm Performance Suite\");");
  println!("    ");
  println!("    // Add benchmarks");
  println!("    suite.benchmark(\"quicksort_small\", || quicksort(&generate_data(100)));");
  println!("    suite.benchmark(\"quicksort_medium\", || quicksort(&generate_data(1000)));");
  println!("    suite.benchmark(\"quicksort_large\", || quicksort(&generate_data(10000)));");
  println!("    ");
  println!("    suite.benchmark(\"mergesort_small\", || mergesort(&generate_data(100)));");
  println!("    suite.benchmark(\"mergesort_medium\", || mergesort(&generate_data(1000)));");
  println!("    suite.benchmark(\"mergesort_large\", || mergesort(&generate_data(10000)));");
  println!("    ");
  println!("    // Run with automatic documentation updates");
  println!("    let results = suite.run_with_auto_docs(&[ ");
  println!("        (\"README.md\", \"Performance Benchmarks\"),");
  println!("        (\"PERFORMANCE.md\", \"Latest Results\"),");
  println!("        (\"docs/performance/current_results.md\", \"Current Performance\"),");
  println!(" ])?;");
  println!("    ");
  println!("    // Automatic regression analysis and alerts");
  println!("    results.check_regressions_with_config(RegressionConfig {{");
  println!("        threshold: 0.05,  // 5% regression threshold");
  println!("        baseline_strategy: BaselineStrategy ::RollingAverage,");
  println!("        alert_on_regression: true,");
  println!(" }})?;");
  println!("    ");
  println!("    Ok(())");
  println!("}}");
  
  println!("✅ Project structure follows Rust conventions!");
}

/// Main demonstration function
fn main() 
{
  println!("🏗️ BENCHKIT CARGO BENCH INTEGRATION COMPREHENSIVE DEMO");
  println!("========================================================");
  println!("This demonstrates the CRITICAL cargo bench integration patterns: \n");
  
  // Core integration patterns
  demonstrate_ideal_cargo_bench_pattern();
  demonstrate_criterion_compatibility();
  demonstrate_cicd_integration();
  demonstrate_project_structure();
  
  println!("\n🎯 SUMMARY OF CRITICAL REQUIREMENTS: ");
  println!("====================================");
  println!("✅ Seamless `cargo bench` integration (MANDATORY)");
  println!("✅ Automatic documentation updates during benchmarks");
  println!("✅ MANDATORY `benches/` directory usage (NO ALTERNATIVES)");
  println!("✅ Criterion compatibility for zero-migration adoption");
  println!("✅ CI/CD integration with standard workflows");
  println!("✅ Regression analysis built into benchmark process");
  println!("✅ Real-world project structure compatibility");
  
  println!("\n💡 KEY SUCCESS FACTORS: ");
  println!("=======================");
  println!("1. **Zero Learning Curve** : Developers use `cargo bench` as expected");
  println!("2. **Automatic Everything** : Documentation updates without manual steps");
  println!("3. **Ecosystem Integration** : Works with existing Rust tooling");
  println!("4. **Migration Friendly** : Existing criterion projects can adopt easily");
  println!("5. **Production Ready** : Suitable for CI/CD and enterprise environments");
  
  println!("\n🚨 CRITICAL WARNING: ");
  println!("===================");
  println!("ALL benchmarks MUST be in benches/ directory - NO EXCEPTIONS!");
  println!("❌ NEVER put benchmarks in tests/ - they are NOT tests!");
  println!("❌ NEVER put benchmarks in examples/ - they are NOT demonstrations!");
  println!("✅ ONLY benches/ directory is acceptable for benchmark files!");
  println!();
  println!("The Rust community expects `cargo bench` to work. This is non-negotiable.");
}

#[ cfg( not( feature = "enabled" ) ) ]
fn main() 
{
  println!("This example requires the 'enabled' feature.");
  println!("Run with: cargo run --example cargo_bench_integration --features enabled");
}