//! String Interning Performance Microbenchmarks
//!
//! This benchmark validates the performance improvement from implementing
//! string interning in command name construction. Tests both cache hit
//! and cache miss scenarios to validate the 5-10x performance target.
//!
//! Expected improvements:
//! - Command name construction: 5-10x faster (38K → 190K-380K cmd/sec)
//! - Memory allocation reduction: ~90% fewer allocations for repeated commands
//! - P99 latency: Under 500μs for command resolution

#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::useless_vec)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::doc_comment_double_space_linebreaks)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::used_underscore_binding)]

#[ cfg( feature = "benchmarks" ) ]
use std::time::Instant;
#[ cfg( feature = "benchmarks" ) ]
use unilang::interner::{ StringInterner, intern_command_name };
#[ cfg( feature = "benchmarks" ) ]
use benchkit::prelude::*;
#[ cfg( feature = "benchmarks" ) ]
use unilang::benchmark_config::BenchmarkConfig;

#[ derive( Debug, Clone ) ]
#[ cfg( feature = "benchmarks" ) ]
struct StringInterningResult
{
  test_name : String,
  iterations : usize,
  total_time_ns : u128,
  avg_time_ns : f64,
  p50_time_ns : u64,
  p95_time_ns : u64,
  p99_time_ns : u64,
  operations_per_second : f64,
  memory_allocations : usize, // Estimated based on new string constructions
}

/// Benchmark traditional string construction (current hot path)
#[ cfg( feature = "benchmarks" ) ]
/// What is measured: fn benchmark_string_construction( command_slices : &[ &[ &str ] ], iterations : usize ) -> StringInterningResult
/// How to measure: cargo bench --bench string_interning_benchmark --features benchmarks
/// Measuring: Baseline string construction performance without interning (worst-case scenario)
fn benchmark_string_construction( command_slices : &[ &[ &str ] ], iterations : usize ) -> StringInterningResult
{
  let mut times = Vec::with_capacity( iterations );
  let mut total_allocations = 0;
  
  let start_time = Instant::now();
  
  for _ in 0..iterations
  {
    for slices in command_slices
    {
      let iter_start = Instant::now();
      
      // Replicate the current hot path logic
      let _command_name = if slices[ 0 ].is_empty() && slices.len() > 1
      {
        total_allocations += 2; // format!() + join()
        format!( ".{}", slices[ 1.. ].join( "." ) )
      }
      else
      {
        total_allocations += 2; // format!() + join()
        format!( ".{}", slices.join( "." ) )
      };
      
      times.push( iter_start.elapsed().as_nanos() as u64 );
    }
  }
  
  let total_time = start_time.elapsed();
  times.sort_unstable();
  
  StringInterningResult
  {
    test_name : "String Construction".to_string(),
    iterations : iterations * command_slices.len(),
    total_time_ns : total_time.as_nanos(),
    avg_time_ns : total_time.as_nanos() as f64 / ( iterations * command_slices.len() ) as f64,
    p50_time_ns : times[ times.len() / 2 ],
    p95_time_ns : times[ ( times.len() as f64 * 0.95 ) as usize ],
    p99_time_ns : times[ ( times.len() as f64 * 0.99 ) as usize ],
    operations_per_second : ( iterations * command_slices.len() ) as f64 / total_time.as_secs_f64(),
    memory_allocations : total_allocations,
  }
}

/// Benchmark string interning (cache miss scenario)
#[ cfg( feature = "benchmarks" ) ]
/// What is measured: fn benchmark_string_interning_miss( command_slices : &[ &[ &str ] ], iterations : usize ) -> StringInterningResult
/// How to measure: cargo bench --bench string_interning_benchmark --features benchmarks
/// Measuring: String interning performance with cache misses (first-time string caching)
fn benchmark_string_interning_miss( command_slices : &[ &[ &str ] ], iterations : usize ) -> StringInterningResult
{
  let mut times = Vec::with_capacity( iterations );
  let mut total_allocations = 0;
  
  let start_time = Instant::now();
  
  for _i in 0..iterations
  {
    // Create unique interner for each iteration to simulate cache miss
    let interner = StringInterner::new();
    
    for slices in command_slices
    {
      let iter_start = Instant::now();
      
      let _command_name = interner.intern_command_name( slices );
      // Cache miss = 1 allocation (Box::leak), then cached
      total_allocations += 1;
      
      times.push( iter_start.elapsed().as_nanos() as u64 );
    }
  }
  
  let total_time = start_time.elapsed();
  times.sort_unstable();
  
  StringInterningResult
  {
    test_name : "String Interning (Cache Miss)".to_string(),
    iterations : iterations * command_slices.len(),
    total_time_ns : total_time.as_nanos(),
    avg_time_ns : total_time.as_nanos() as f64 / ( iterations * command_slices.len() ) as f64,
    p50_time_ns : times[ times.len() / 2 ],
    p95_time_ns : times[ ( times.len() as f64 * 0.95 ) as usize ],
    p99_time_ns : times[ ( times.len() as f64 * 0.99 ) as usize ],
    operations_per_second : ( iterations * command_slices.len() ) as f64 / total_time.as_secs_f64(),
    memory_allocations : total_allocations,
  }
}

/// Benchmark string interning (cache hit scenario)
#[ cfg( feature = "benchmarks" ) ]
/// What is measured: fn benchmark_string_interning_hit( command_slices : &[ &[ &str ] ], iterations : usize ) -> StringInterningResult
/// How to measure: cargo bench --bench string_interning_benchmark --features benchmarks  
/// Measuring: String interning performance with cache hits (optimal scenario - strings already cached)
fn benchmark_string_interning_hit( command_slices : &[ &[ &str ] ], iterations : usize ) -> StringInterningResult
{
  let mut times = Vec::with_capacity( iterations );
  let interner = StringInterner::new();
  
  // Pre-populate cache
  for slices in command_slices
  {
    interner.intern_command_name( slices );
  }
  
  let start_time = Instant::now();
  
  for _ in 0..iterations
  {
    for slices in command_slices
    {
      let iter_start = Instant::now();
      
      let _command_name = interner.intern_command_name( slices );
      // Cache hit = 0 allocations, just hash lookup
      
      times.push( iter_start.elapsed().as_nanos() as u64 );
    }
  }
  
  let total_time = start_time.elapsed();
  times.sort_unstable();
  
  StringInterningResult
  {
    test_name : "String Interning (Cache Hit)".to_string(),
    iterations : iterations * command_slices.len(),
    total_time_ns : total_time.as_nanos(),
    avg_time_ns : total_time.as_nanos() as f64 / ( iterations * command_slices.len() ) as f64,
    p50_time_ns : times[ times.len() / 2 ],
    p95_time_ns : times[ ( times.len() as f64 * 0.95 ) as usize ],
    p99_time_ns : times[ ( times.len() as f64 * 0.99 ) as usize ],
    operations_per_second : ( iterations * command_slices.len() ) as f64 / total_time.as_secs_f64(),
    memory_allocations : 0, // All cache hits
  }
}

/// Benchmark global string interning convenience functions
#[ cfg( feature = "benchmarks" ) ]
fn benchmark_global_interner( command_slices : &[ &[ &str ] ], iterations : usize ) -> StringInterningResult
{
  let mut times = Vec::with_capacity( iterations );
  
  // Pre-populate global cache
  for slices in command_slices
  {
    let _ = intern_command_name( slices );
  }
  
  let start_time = Instant::now();
  
  for _ in 0..iterations
  {
    for slices in command_slices
    {
      let iter_start = Instant::now();
      
      let _command_name = intern_command_name( slices );
      
      times.push( iter_start.elapsed().as_nanos() as u64 );
    }
  }
  
  let total_time = start_time.elapsed();
  times.sort_unstable();
  
  StringInterningResult
  {
    test_name : "Global String Interner".to_string(),
    iterations : iterations * command_slices.len(),
    total_time_ns : total_time.as_nanos(),
    avg_time_ns : total_time.as_nanos() as f64 / ( iterations * command_slices.len() ) as f64,
    p50_time_ns : times[ times.len() / 2 ],
    p95_time_ns : times[ ( times.len() as f64 * 0.95 ) as usize ],
    p99_time_ns : times[ ( times.len() as f64 * 0.99 ) as usize ],
    operations_per_second : ( iterations * command_slices.len() ) as f64 / total_time.as_secs_f64(),
    memory_allocations : 0, // Pre-cached
  }
}

#[ cfg( feature = "benchmarks" ) ]
fn print_result( result : &StringInterningResult )
{
  println!( "=== {} ===" , result.test_name );
  println!( "Iterations: {}", result.iterations );
  println!( "Total Time: {:.2} ms", result.total_time_ns as f64 / 1_000_000.0 );
  println!( "Average Time: {:.0} ns", result.avg_time_ns );
  println!( "P50 Latency: {} ns", result.p50_time_ns );
  println!( "P95 Latency: {} ns", result.p95_time_ns );
  println!( "P99 Latency: {} ns", result.p99_time_ns );
  println!( "Operations/sec: {:.0}", result.operations_per_second );
  println!( "Memory Allocations: {}", result.memory_allocations );
  println!();
}

/// Run statistical analysis benchmarks using benchkit
#[ cfg( feature = "benchmarks" ) ]
#[ allow( dead_code ) ]
fn run_statistical_analysis_benchmarks()
{
  println!( "📊 String Interning Statistical Analysis (Environment-Adaptive)" );
  println!( "================================================================\n" );
  
  // Get environment-specific benchmark configuration
  let benchmark_config = BenchmarkConfig::from_environment();
  println!( "🌍 Environment: {}", benchmark_config.environment );
  println!( "📏 CV Tolerance: {:.1}%", benchmark_config.cv_tolerance * 100.0 );
  println!( "📊 Sample Range: {}-{}", benchmark_config.min_sample_size, benchmark_config.max_sample_size );
  println!( "🎯 Regression Threshold: {:.1}%\n", benchmark_config.regression_threshold * 100.0 );
  
  // Realistic command patterns from typical usage
  let test_commands = vec![
    vec![ "file", "create" ],
    vec![ "file", "delete" ],
    vec![ "user", "login" ],
    vec![ "user", "logout" ],
    vec![ "system", "status" ],
    vec![ "database", "migrate" ],
    vec![ "cache", "clear" ],
    vec![ "config", "get", "value" ],
    vec![ "config", "set", "key" ],
    vec![ "deploy", "production", "service" ],
  ];
  
  let command_slices : Vec< &[ &str ] > = test_commands.iter().map( std::vec::Vec::as_slice ).collect();
  
  // Use environment-specific measurement configuration
  println!( "📈 Running statistical analysis with {} samples per algorithm...\n", 
           benchmark_config.min_sample_size );
  
  // Convert to benchkit MeasurementConfig
  let config: MeasurementConfig = benchmark_config.to_measurement_config().into();
  
  // Benchmark 1: String construction (baseline)
  let baseline_result = bench_function_with_config("string_construction", &config, || {
    for slices in &command_slices {
      let _command_name = slices.join(".");  // String allocation per call
    }
  });
  
  // Benchmark 2: String interning (cache miss)
  let interner_miss_result = bench_function_with_config("string_interning_miss", &config, || {
    let interner = StringInterner::new();
    for slices in &command_slices {
      let _interned = interner.intern_command_name(slices);
    }
  });
  
  // Benchmark 3: String interning (cache hit - pre-warm cache)
  let interner_hit_result = bench_function_with_config("string_interning_hit", &config, || {
    let interner = StringInterner::new();
    // Pre-warm cache
    for slices in &command_slices {
      let _interned = interner.intern_command_name(slices);
    }
    // Now measure cache hits
    for slices in &command_slices {
      let _interned = interner.intern_command_name(slices);
    }
  });
  
  // Benchmark 4: Global interner  
  let global_interner_result = bench_function_with_config("global_interner", &config, || {
    for slices in &command_slices {
      let _interned = intern_command_name(slices);
    }
  });
  
  println!( "🔬 Statistical Analysis Results" );
  println!( "==============================\n" );
  
  // Analyze each result with statistical significance testing
  let algorithms = vec![
    ("String Construction (Baseline)", &baseline_result),
    ("String Interning (Cache Miss)", &interner_miss_result),  
    ("String Interning (Cache Hit)", &interner_hit_result),
    ("Global Interner", &global_interner_result),
  ];
  
  let mut reliable_results: Vec<(&str, &BenchmarkResult, StatisticalAnalysis)> = Vec::new();
  
  for (name, result) in &algorithms {
    println!( "📊 {name}" );
    
    if let Ok(analysis) = StatisticalAnalysis::analyze(result, SignificanceLevel::Standard) {
      println!( "  Mean Time: {:.2?} ± {:.2?} (95% confidence)", 
               analysis.mean_confidence_interval.point_estimate,
               analysis.mean_confidence_interval.margin_of_error );
      println!( "  Coefficient of Variation: {:.1}%", analysis.coefficient_of_variation * 100.0 );
      println!( "  Statistical Power: {:.3}", analysis.statistical_power );
      println!( "  Sample Size: {}", result.times.len() );
      
      // Use environment-specific CV requirements
      let cv_meets_env = benchmark_config.cv_meets_requirements(analysis.coefficient_of_variation);
      let is_reliable = analysis.is_reliable() && cv_meets_env;
      
      if is_reliable {
        println!( "  Quality: ✅ Meets {} environment requirements", benchmark_config.environment );
        reliable_results.push((name, result, analysis));
      } else if !cv_meets_env {
        println!( "  Quality: ⚠️  CV {:.1}% exceeds {:.1}% tolerance for {} environment", 
                 analysis.coefficient_of_variation * 100.0,
                 benchmark_config.cv_tolerance * 100.0,
                 benchmark_config.environment );
        let recommended_size = benchmark_config.adaptive_sample_size(analysis.coefficient_of_variation);
        println!( "  Recommendation: Use {} samples for this variance level", recommended_size );
      } else {
        println!( "  Quality: ⚠️  Not statistically reliable - need more samples" );
        println!( "  Recommendation: Increase sample size to at least {}", 
                 benchmark_config.adaptive_sample_size(analysis.coefficient_of_variation) );
      }
    } else {
      println!( "  Quality: ❌ Statistical analysis failed" );
    }
    println!();
  }
  
  // Comparative analysis for reliable results only
  if reliable_results.len() >= 2 {
    println!( "🎯 Performance Comparison (Reliable Results Only)" );
    println!( "================================================\n" );
    
    let baseline_analysis = reliable_results.iter()
      .find(|(name, _, _)| name.contains("Baseline"))
      .map(|(_, _, analysis)| analysis);
      
    if let Some(baseline) = baseline_analysis {
      for (name, _result, analysis) in &reliable_results {
        if !name.contains("Baseline") {
          // Compare with baseline using statistical comparison
          if let Ok(comparison) = StatisticalAnalysis::compare(
            &baseline_result, 
            _result, 
            SignificanceLevel::Standard
          ) {
            let improvement = baseline.mean_confidence_interval.point_estimate.as_nanos() as f64 
                            / analysis.mean_confidence_interval.point_estimate.as_nanos() as f64;
            let change_ratio = (improvement - 1.0).abs();
            
            let env_significant = benchmark_config.is_significant_change(change_ratio);
            let statistically_significant = comparison.is_significant;
            
            if statistically_significant && env_significant {
              println!( "✅ {name}: {:.1}x faster than baseline (significant for {} environment)", 
                       improvement, benchmark_config.environment );
            } else if statistically_significant && !env_significant {
              println!( "📈 {name}: {:.1}x faster than baseline (statistically significant but < {:.1}% threshold)", 
                       improvement, benchmark_config.regression_threshold * 100.0 );
            } else {
              println!( "🔍 {name}: {:.1}x faster than baseline (not significant)", improvement );
            }
          }
        }
      }
    }
  } else {
    println!( "⚠️  Not enough reliable results for performance comparison" );
    println!( "   Increase sample sizes and rerun for statistical analysis" );
  }
}

#[ cfg( feature = "benchmarks" ) ]
#[ allow( dead_code ) ]
fn run_string_interning_benchmarks()
{
  println!( "🚀 String Interning Performance Benchmarks" );
  println!( "============================================\n" );
  
  // Realistic command patterns from typical usage
  let test_commands = vec![
    vec![ "file", "create" ],
    vec![ "file", "delete" ],
    vec![ "user", "login" ],
    vec![ "user", "logout" ],
    vec![ "system", "status" ],
    vec![ "database", "migrate" ],
    vec![ "cache", "clear" ],
    vec![ "config", "get", "value" ],
    vec![ "config", "set", "key" ],
    vec![ "deploy", "production", "service" ],
  ];
  
  let command_slices : Vec< &[ &str ] > = test_commands.iter().map( std::vec::Vec::as_slice ).collect();
  
  use unilang::benchmark_data_sizes::BenchmarkDataSize;
  let iterations = BenchmarkDataSize::Huge.value(); // 10,000 iterations for statistical significance
  
  println!( "Test Configuration:" );
  println!( "- Command patterns: {}", command_slices.len() );
  println!( "- Iterations per pattern: {} ({})", iterations, BenchmarkDataSize::Huge.description() );
  println!( "- Total operations: {}", command_slices.len() * iterations );
  println!( "{}", unilang::benchmark_data_sizes::BenchmarkDataUtils::document_sizes() );
  println!();
  
  // Benchmark 1: Traditional string construction (baseline)
  println!( "Running baseline string construction benchmark..." );
  let baseline = benchmark_string_construction( &command_slices, iterations );
  print_result( &baseline );
  
  // Benchmark 2: String interning cache miss
  println!( "Running string interning (cache miss) benchmark..." );
  let interner_miss = benchmark_string_interning_miss( &command_slices, iterations );
  print_result( &interner_miss );
  
  // Benchmark 3: String interning cache hit
  println!( "Running string interning (cache hit) benchmark..." );
  let interner_hit = benchmark_string_interning_hit( &command_slices, iterations );
  print_result( &interner_hit );
  
  // Benchmark 4: Global interner
  println!( "Running global interner benchmark..." );
  let global_interner = benchmark_global_interner( &command_slices, iterations );
  print_result( &global_interner );
  
  // Performance Analysis
  println!( "🎯 Performance Analysis" );
  println!( "======================" );
  
  let baseline_ops = baseline.operations_per_second;
  let miss_improvement = interner_miss.operations_per_second / baseline_ops;
  let hit_improvement = interner_hit.operations_per_second / baseline_ops;
  let global_improvement = global_interner.operations_per_second / baseline_ops;
  
  println!( "Improvement vs String Construction:" );
  println!( "- Cache Miss: {:.1}x faster ({:.0} vs {:.0} ops/sec)", 
           miss_improvement, interner_miss.operations_per_second, baseline_ops );
  println!( "- Cache Hit: {:.1}x faster ({:.0} vs {:.0} ops/sec)",
           hit_improvement, interner_hit.operations_per_second, baseline_ops );
  println!( "- Global Interner: {:.1}x faster ({:.0} vs {:.0} ops/sec)",
           global_improvement, global_interner.operations_per_second, baseline_ops );
  println!();
  
  let alloc_reduction = ( ( baseline.memory_allocations - interner_hit.memory_allocations ) as f64 
                         / baseline.memory_allocations as f64 ) * 100.0;
  println!( "Memory Allocation Reduction (Cache Hit): {alloc_reduction:.0}%" );
  
  // Success criteria validation
  let target_met = hit_improvement >= 5.0;
  println!();
  println!( "✅ Target Validation (5x minimum improvement): {}", 
           if target_met { "PASSED" } else { "FAILED" } );
  
  if hit_improvement >= 10.0
  {
    println!( "🎉 Exceeded stretch goal of 10x improvement!" );
  }
  
  // Latency analysis
  println!();
  println!( "Latency Analysis:" );
  println!( "- Baseline P99: {} ns", baseline.p99_time_ns );
  println!( "- Interner P99: {} ns", interner_hit.p99_time_ns );
  let target_latency_met = interner_hit.p99_time_ns <= 500_000; // 500μs
  println!( "- P99 under 500μs target: {}", if target_latency_met { "PASSED" } else { "FAILED" } );
}

/// Helper benchmark wrappers for standard setup protocol
#[ cfg( feature = "benchmarks" ) ]
fn run_string_construction_benchmark() {
  let test_commands = vec![
    vec![ "file", "create" ], vec![ "file", "delete" ], vec![ "user", "login" ],
    vec![ "user", "logout" ], vec![ "system", "status" ],
  ];
  let command_slices : Vec< &[ &str ] > = test_commands.iter().map( std::vec::Vec::as_slice ).collect();
  let _ = benchmark_string_construction(&command_slices, 1000);
}

#[ cfg( feature = "benchmarks" ) ]
fn run_string_interning_miss_benchmark() {
  let test_commands = vec![
    vec![ "file", "create" ], vec![ "file", "delete" ], vec![ "user", "login" ],
    vec![ "user", "logout" ], vec![ "system", "status" ],
  ];
  let command_slices : Vec< &[ &str ] > = test_commands.iter().map( std::vec::Vec::as_slice ).collect();
  let _ = benchmark_string_interning_miss(&command_slices, 1000);
}

#[ cfg( feature = "benchmarks" ) ]
fn run_string_interning_hit_benchmark() {
  let test_commands = vec![
    vec![ "file", "create" ], vec![ "file", "delete" ], vec![ "user", "login" ],
    vec![ "user", "logout" ], vec![ "system", "status" ],
  ];
  let command_slices : Vec< &[ &str ] > = test_commands.iter().map( std::vec::Vec::as_slice ).collect();
  let _ = benchmark_string_interning_hit(&command_slices, 1000);
}

#[ cfg( feature = "benchmarks" ) ]
fn run_global_interner_benchmark() {
  let test_commands = vec![
    vec![ "file", "create" ], vec![ "file", "delete" ], vec![ "user", "login" ],
    vec![ "user", "logout" ], vec![ "system", "status" ],
  ];
  let command_slices : Vec< &[ &str ] > = test_commands.iter().map( std::vec::Vec::as_slice ).collect();
  let _ = benchmark_global_interner(&command_slices, 1000);
}

#[ cfg( feature = "benchmarks" ) ]
fn main()
{
  use benchkit::prelude::*;
  
  // BENCHKIT STANDARD SETUP PROTOCOL - NON-NEGOTIABLE REQUIREMENT
  let mut suite = BenchmarkSuite::new("String Interning Performance");
  
  // Add core string interning benchmarks
  suite.benchmark("string_construction_baseline", || {
    run_string_construction_benchmark();
  });
  
  suite.benchmark("string_interning_cache_miss", || {
    run_string_interning_miss_benchmark();
  });
  
  suite.benchmark("string_interning_cache_hit", || {
    run_string_interning_hit_benchmark();
  });
  
  suite.benchmark("global_interner", || {
    run_global_interner_benchmark();
  });
  
  // Run all benchmarks
  let results = suite.run_all();
  
  // Print results summary
  results.print_summary();
  
  // MANDATORY: Update documentation automatically across multiple files
  use unilang::documentation_updater::DocumentationUpdater;
  let doc_updater = DocumentationUpdater::new();
  let markdown_report = results.generate_markdown_report();
  let comprehensive_report = DocumentationUpdater::generate_report(
      "String Interning Performance",
      &markdown_report.generate()
  );
  
  if let Err(e) = doc_updater.update_documentation("String Interning Performance", &comprehensive_report) {
      eprintln!("⚠️ Documentation update failed: {}", e);
  }
  
  println!("\n✅ Benchkit standard setup protocol completed");
}

#[ cfg( not( feature = "benchmarks" ) ) ]
fn main()
{
  println!( "String interning benchmarks require the 'benchmarks' feature flag." );
  println!( "Run with: cargo run --bin string_interning_benchmark --features benchmarks" );
}