//! Optimization workflow demonstration
//!
//! Shows the 3-step systematic optimization process:
//! 1. Establish performance baseline
//! 2. Implement optimization changes  
//! 3. Measure optimization impact with regression detection

#[ cfg( feature = "benchmarks" ) ]
#[ allow( unused_imports ) ]
use unilang::
{
  optimization_workflow::{ OptimizationWorkflow, OptimizationStep, OptimizationTargets },
  cv_analysis::{ BenchmarkResult, CvAnalyzer },
  comparative_benchmark_structure::ComparativeBenchmark,
};

#[ cfg( feature = "benchmarks" ) ]
fn main() -> std::io::Result< () >
{
  println!( "🚀 Optimization Workflow Demonstration" );

  // Create optimization workflow manager
  let mut workflow = OptimizationWorkflow::new(
    "string_processing_optimization",
    "String processing performance optimization workflow"
  );

  // Demo Step 1: Establish baseline
  println!( "\n=== Step 1: Establishing Baseline ===" );
  demonstrate_baseline_establishment( &mut workflow );

  // Demo Step 2: Simulate optimization work
  println!( "\n=== Step 2: Optimization Implementation ===" );
  println!( "⚡ Implementing string processing optimizations..." );
  println!( "   - Added early termination conditions" );
  println!( "   - Implemented SIMD-optimized operations" );
  println!( "   - Introduced memory pool allocation" );

  // Demo Step 3: Measure impact
  println!( "\n=== Step 3: Measuring Optimization Impact ===" );
  demonstrate_optimization_measurement( &mut workflow );

  println!( "\n🎯 Optimization workflow complete!" );

  Ok( () )
}

#[ cfg( feature = "benchmarks" ) ]
fn demonstrate_baseline_establishment( workflow: &mut OptimizationWorkflow )
{
  // Create baseline benchmark results
  let baseline_result = generate_baseline_results();

  // Set baseline in workflow
  workflow.set_baseline( baseline_result );

  println!( "✅ Baseline established with string processing benchmark" );
}

#[ cfg( feature = "benchmarks" ) ]
fn demonstrate_optimization_measurement( workflow: &mut OptimizationWorkflow )
{
  // Generate "optimized" results (simulated improvements)
  let optimized_result = generate_optimized_results();

  // Add optimization step to workflow
  let optimization_step = workflow.add_optimization_step(
    "SIMD and early termination optimizations",
    optimized_result
  );

  match optimization_step
  {
    Ok( step ) =>
    {
      let improvement = step.improvement_percentage();
      if improvement > 0.0
      {
        println!( "✅ Optimization successful: {:.1}% improvement", improvement );
      }
      else
      {
        println!( "⚠️  Performance regression: {:.1}% slower", -improvement );
      }
    },
    Err( e ) =>
    {
      println!( "❌ Failed to add optimization step: {}", e );
    }
  }

  // Generate optimization report
  let report = workflow.generate_report();
  println!( "\n📊 Optimization Report:\n{}", report );
}

#[ cfg( feature = "benchmarks" ) ]
fn generate_baseline_results() -> BenchmarkResult
{
  use std::time::Duration;

  // Simulate baseline benchmark results for string processing
  let times = vec![
    Duration::from_nanos( 2_300_000 ),
    Duration::from_nanos( 2_500_000 ),
    Duration::from_nanos( 2_400_000 ),
    Duration::from_nanos( 2_600_000 ),
    Duration::from_nanos( 2_450_000 ),
    Duration::from_nanos( 2_550_000 ),
    Duration::from_nanos( 2_350_000 ),
    Duration::from_nanos( 2_700_000 ),
    Duration::from_nanos( 2_500_000 ),
    Duration::from_nanos( 2_400_000 ),
  ];

  BenchmarkResult::new( "string_processing_baseline", times, 1000 )
}

#[ cfg( feature = "benchmarks" ) ]
fn generate_optimized_results() -> BenchmarkResult
{
  use std::time::Duration;

  // Simulate optimized results (30% improvement)
  let times = vec![
    Duration::from_nanos( 1_610_000 ), // ~30% faster than baseline
    Duration::from_nanos( 1_750_000 ),
    Duration::from_nanos( 1_680_000 ),
    Duration::from_nanos( 1_820_000 ),
    Duration::from_nanos( 1_715_000 ),
    Duration::from_nanos( 1_785_000 ),
    Duration::from_nanos( 1_645_000 ),
    Duration::from_nanos( 1_890_000 ),
    Duration::from_nanos( 1_750_000 ),
    Duration::from_nanos( 1_680_000 ),
  ];

  BenchmarkResult::new( "string_processing_optimized", times, 1000 )
}

#[ cfg( not( feature = "benchmarks" ) ) ]
fn main()
{
  eprintln!( "Error: Benchmarks not enabled!" );
  eprintln!( "Run with: cargo run --bin optimization_workflow_demo --features benchmarks" );
  std::process::exit( 1 );
}