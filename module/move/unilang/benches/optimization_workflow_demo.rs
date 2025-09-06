//! Optimization workflow demonstration
//!
//! Shows the 3-step systematic optimization process:
//! 1. Establish performance baseline
//! 2. Implement optimization changes  
//! 3. Measure optimization impact with regression detection

#[ cfg( feature = "benchmarks" ) ]
#[allow(unused_imports)]
use unilang::
{
  OptimizationWorkflow,
  BenchmarkResult,
  CoefficientsOfVariationAnalysis,
  ComparativeBenchmark,
  BenchmarkDataSize,
  RealisticDataGenerator,
};

#[ cfg( feature = "benchmarks" ) ]
fn main() -> std::io::Result< () >
{
  println!( "🚀 Optimization Workflow Demonstration" );
  
  // Create optimization workflow manager
  let workflow = OptimizationWorkflow::new( 
    "target/optimization_baselines", 
    "string_processing_optimization".to_string() 
  )?;
  
  // Demo Step 1: Establish baseline (if not exists)
  #[allow(clippy::if_not_else)]
  if !workflow.has_baseline()
  {
    println!( "\n=== Step 1: Establishing Baseline ===" );
    demonstrate_baseline_establishment( &workflow )?;
  }
  else
  {
    println!( "\n✅ Baseline already exists, skipping establishment" );
  }
  
  // Demo Step 2: Simulate optimization work
  println!( "\n=== Step 2: Optimization Implementation ===" );
  println!( "⚡ Implementing string processing optimizations..." );
  println!( "   - Added early termination conditions" );
  println!( "   - Implemented SIMD-optimized operations" );
  println!( "   - Introduced memory pool allocation" );
  
  // Demo Step 3: Measure impact
  println!( "\n=== Step 3: Measuring Optimization Impact ===" );
  demonstrate_optimization_measurement( workflow )?;
  
  println!( "\n🎯 Optimization workflow complete!" );
  println!( "📄 Documentation generated in: target/optimization_baselines/" );
  
  Ok( () )
}

#[ cfg( feature = "benchmarks" ) ]
fn demonstrate_baseline_establishment( workflow : &OptimizationWorkflow ) -> std::io::Result< () >
{
  // Create baseline benchmark results
  let baseline_results = generate_baseline_results();
  
  // Create CV analysis for baseline quality
  let cv_analysis = CoefficientsOfVariationAnalysis::new( 
    baseline_results.iter().map( unilang::BenchmarkResult::coefficient_of_variation ).collect(),
    "String Processing Baseline".to_string()
  );
  
  // Establish baseline
  let _baseline = workflow.establish_baseline(
    baseline_results,
    cv_analysis,
    "Intel i7-9700K, 32GB RAM, NVMe SSD".to_string(),
    vec![ 
      "Cold CPU cache measurements".to_string(),
      "Low system load".to_string(),
      "Development environment".to_string()
    ]
  )?;
  
  Ok( () )
}

#[ cfg( feature = "benchmarks" ) ]
fn demonstrate_optimization_measurement( mut workflow : OptimizationWorkflow ) -> std::io::Result< () >
{
  // Generate "optimized" results (simulated improvements)
  let optimized_results = generate_optimized_results();
  
  // Create CV analysis for optimized measurements
  let current_cv = CoefficientsOfVariationAnalysis::new(
    optimized_results.iter().map( unilang::BenchmarkResult::coefficient_of_variation ).collect(),
    "String Processing Optimized".to_string()
  );
  
  // Measure optimization impact
  let _impact = workflow.measure_optimization_impact( optimized_results, current_cv )?;
  
  Ok( () )
}

#[ cfg( feature = "benchmarks" ) ]
fn generate_baseline_results() -> Vec< BenchmarkResult >
{
  // Simulate baseline benchmark results
  vec![
    BenchmarkResult
    {
      algorithm_name : "string_split_operations".to_string(),
      average_time_nanos : 2_500_000.0,  // 2.5ms
      std_dev_nanos : 125_000.0,         // 5% CV
      min_time_nanos : 2_300_000,
      max_time_nanos : 2_700_000,
      sample_count : 100,
    },
    BenchmarkResult
    {
      algorithm_name : "string_validation".to_string(),
      average_time_nanos : 1_800_000.0,  // 1.8ms  
      std_dev_nanos : 90_000.0,          // 5% CV
      min_time_nanos : 1_650_000,
      max_time_nanos : 1_950_000,
      sample_count : 100,
    },
    BenchmarkResult
    {
      algorithm_name : "string_transformation".to_string(),
      average_time_nanos : 3_200_000.0,  // 3.2ms
      std_dev_nanos : 160_000.0,         // 5% CV
      min_time_nanos : 2_900_000,
      max_time_nanos : 3_500_000,
      sample_count : 100,
    },
    BenchmarkResult
    {
      algorithm_name : "string_pattern_matching".to_string(),
      average_time_nanos : 1_200_000.0,  // 1.2ms
      std_dev_nanos : 60_000.0,          // 5% CV
      min_time_nanos : 1_100_000,
      max_time_nanos : 1_300_000,
      sample_count : 100,
    },
  ]
}

#[ cfg( feature = "benchmarks" ) ]
fn generate_optimized_results() -> Vec< BenchmarkResult >
{
  // Simulate post-optimization results with mixed outcomes
  vec![
    // Significant improvement (30% faster)
    BenchmarkResult
    {
      algorithm_name : "string_split_operations".to_string(),
      average_time_nanos : 1_750_000.0,  // Improved from 2.5ms to 1.75ms  
      std_dev_nanos : 87_500.0,
      min_time_nanos : 1_600_000,
      max_time_nanos : 1_900_000,
      sample_count : 100,
    },
    // Moderate improvement (15% faster)
    BenchmarkResult
    {
      algorithm_name : "string_validation".to_string(), 
      average_time_nanos : 1_530_000.0,  // Improved from 1.8ms to 1.53ms
      std_dev_nanos : 76_500.0,
      min_time_nanos : 1_400_000,
      max_time_nanos : 1_660_000,
      sample_count : 100,
    },
    // No significant change (baseline performance)
    BenchmarkResult
    {
      algorithm_name : "string_transformation".to_string(),
      average_time_nanos : 3_150_000.0,  // Slight improvement from 3.2ms to 3.15ms (~2%)
      std_dev_nanos : 157_500.0,
      min_time_nanos : 2_850_000,
      max_time_nanos : 3_450_000,
      sample_count : 100,
    },
    // Performance regression (10% slower) 
    BenchmarkResult
    {
      algorithm_name : "string_pattern_matching".to_string(),
      average_time_nanos : 1_320_000.0,  // Regression from 1.2ms to 1.32ms
      std_dev_nanos : 66_000.0,
      min_time_nanos : 1_200_000,
      max_time_nanos : 1_440_000,
      sample_count : 100,
    },
  ]
}

#[ cfg( not( feature = "benchmarks" ) ) ]
fn main()
{
  eprintln!( "Error: Benchmarks not enabled!" );
  eprintln!( "Run with: cargo run --bin optimization_workflow_demo --features benchmarks" );
  std::process::exit( 1 );
}