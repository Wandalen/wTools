//! Tests for context-rich benchmark documentation functionality

#![ cfg( feature = "benchmarks" ) ]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::float_cmp)]

use unilang::{
  ContextRichDocGenerator,
  BenchmarkMeasurementContext,
  EnvironmentContext,
  BeforeAfterComparison,
  OptimizationStatus,
  BenchmarkDataSize,
  ComparativeResults,
  BenchmarkResult,
};

#[ cfg( not( feature = "benchmarks" ) ) ]
fn main() {}

#[ test ]
fn test_environment_context_creation()
{
  let env = EnvironmentContext
  {
    cpu : "Test CPU".to_string(),
    ram : "16GB".to_string(),
    storage : "SSD".to_string(),
    load_characteristics : "test load".to_string(),
    notes : vec![ "test note".to_string() ],
  };
  
  assert_eq!( env.cpu, "Test CPU" );
  assert_eq!( env.ram, "16GB" );
  assert_eq!( env.storage, "SSD" );
  assert!( env.notes.contains( &"test note".to_string() ) );
}

#[ test ]
fn test_optimization_status_indicators()
{
  assert_eq!( OptimizationStatus::Optimized.indicator(), "✅" );
  assert_eq!( OptimizationStatus::NeedsWork.indicator(), "⚠️" );
  assert_eq!( OptimizationStatus::ProductionReady.indicator(), "🚀" );
  assert_eq!( OptimizationStatus::Baseline.indicator(), "📊" );
  assert_eq!( OptimizationStatus::Regression.indicator(), "❌" );
  
  assert_eq!( OptimizationStatus::Optimized.description(), "Optimized" );
  assert_eq!( OptimizationStatus::NeedsWork.description(), "Needs work" );
  assert_eq!( OptimizationStatus::ProductionReady.description(), "Production ready" );
}

#[ test ]
fn test_before_after_comparison()
{
  let comparison = BeforeAfterComparison
  {
    algorithm_name : "test_algorithm".to_string(),
    before_nanos : 2_000_000.0, // 2ms
    after_nanos : 1_500_000.0,  // 1.5ms
    status : OptimizationStatus::Optimized,
  };
  
  assert_eq!( comparison.improvement_percentage(), 25.0 );
  assert_eq!( comparison.format_improvement(), "25.0% faster" );
  assert_eq!( BeforeAfterComparison::format_time( 2_000_000.0 ), "2.00ms" );
  assert_eq!( BeforeAfterComparison::format_time( 1_500.0 ), "1.50µs" );
}

#[ test ]
fn test_before_after_regression()
{
  let comparison = BeforeAfterComparison
  {
    algorithm_name : "regression_test".to_string(),
    before_nanos : 1_000_000.0, // 1ms
    after_nanos : 1_200_000.0,  // 1.2ms (slower)
    status : OptimizationStatus::Regression,
  };
  
  assert_eq!( comparison.improvement_percentage(), -20.0 );
  assert_eq!( comparison.format_improvement(), "20.0% slower" );
}

#[ test ]
fn test_before_after_no_change()
{
  let comparison = BeforeAfterComparison
  {
    algorithm_name : "no_change_test".to_string(),
    before_nanos : 1_000_000.0, // 1ms
    after_nanos : 1_000_000.0,  // 1ms (same)
    status : OptimizationStatus::Baseline,
  };
  
  assert_eq!( comparison.improvement_percentage(), 0.0 );
  assert_eq!( comparison.format_improvement(), "No change" );
}

#[ test ]
fn test_doc_generator_creation()
{
  let env = EnvironmentContext
  {
    cpu : "Test CPU".to_string(),
    ram : "8GB".to_string(),
    storage : "HDD".to_string(),
    load_characteristics : "low load".to_string(),
    notes : vec![],
  };
  
  let generator = ContextRichDocGenerator::new( env.clone() );
  assert_eq!( generator.environment().cpu, env.cpu );
  assert_eq!( generator.environment().ram, env.ram );
  assert_eq!( generator.section_count(), 0 );
}

#[ test ]
fn test_default_environment_generator()
{
  let generator = ContextRichDocGenerator::default_environment();
  assert!( generator.environment().cpu.contains( "CPU" ) );
  assert!( generator.environment().ram.contains( "GB" ) );
  assert!( generator.environment().storage.contains( "SSD" ) );
}

#[ test ]
fn test_comparative_results_documentation()
{
  let mut generator = ContextRichDocGenerator::default_environment();
  
  // Create sample comparative results
  let results = vec![
    BenchmarkResult
    {
      algorithm_name : "fast_algo".to_string(),
      average_time_nanos : 1_000_000.0, // 1ms
      std_dev_nanos : 50_000.0,
      min_time_nanos : 950_000,
      max_time_nanos : 1_050_000,
      sample_count : 100,
    },
    BenchmarkResult
    {
      algorithm_name : "slow_algo".to_string(),
      average_time_nanos : 2_000_000.0, // 2ms
      std_dev_nanos : 100_000.0,
      min_time_nanos : 1_900_000,
      max_time_nanos : 2_100_000,
      sample_count : 100,
    },
  ];
  
  let comparative_results = ComparativeResults::new(
    "Algorithm Comparison".to_string(),
    "Testing algorithm performance".to_string(),
    BenchmarkDataSize::Small,
    results
  );
  
  let context = BenchmarkMeasurementContext
  {
    what_is_measured : "Algorithm performance comparison".to_string(),
    how_to_measure : "cargo test".to_string(),
    environment : generator.environment().clone(),
    purpose : "Test documentation generation".to_string(),
  };
  
  generator.add_comparative_results( context, &comparative_results );
  
  assert_eq!( generator.section_count(), 1 );
  
  let report = generator.generate_report( "Test Report" );
  assert!( report.contains( "Algorithm Comparison Performance Analysis" ) );
  assert!( report.contains( "fast_algo" ) );
  assert!( report.contains( "slow_algo" ) );
  assert!( report.contains( "1.00x (baseline)" ) );
  assert!( report.contains( "2.00x slower" ) );
  assert!( report.contains( "Key Findings" ) );
  assert!( report.contains( "Environment Specification" ) );
}

#[ test ]
fn test_before_after_documentation()
{
  let mut generator = ContextRichDocGenerator::default_environment();
  
  let comparisons = vec![
    BeforeAfterComparison
    {
      algorithm_name : "optimized_function".to_string(),
      before_nanos : 3_000_000.0, // 3ms
      after_nanos : 2_000_000.0,  // 2ms
      status : OptimizationStatus::Optimized,
    },
    BeforeAfterComparison
    {
      algorithm_name : "needs_work_function".to_string(),
      before_nanos : 1_000_000.0, // 1ms
      after_nanos : 1_100_000.0,  // 1.1ms (regression)
      status : OptimizationStatus::NeedsWork,
    },
  ];
  
  let context = BenchmarkMeasurementContext
  {
    what_is_measured : "Function optimization impact".to_string(),
    how_to_measure : "cargo bench".to_string(),
    environment : generator.environment().clone(),
    purpose : "Validate optimization effectiveness".to_string(),
  };
  
  generator.add_before_after_comparison(
    "Optimization Results",
    context,
    &comparisons
  );
  
  assert_eq!( generator.section_count(), 1 );
  
  let report = generator.generate_report( "Optimization Report" );
  assert!( report.contains( "Optimization Results" ) );
  assert!( report.contains( "optimized_function" ) );
  assert!( report.contains( "needs_work_function" ) );
  assert!( report.contains( "33.3% faster" ) );
  assert!( report.contains( "10.0% slower" ) );
  assert!( report.contains( "Analysis & Recommendations" ) );
  assert!( report.contains( "Action Required" ) );
  assert!( report.contains( "Next Steps" ) );
}

#[ test ]
fn test_measurement_context()
{
  let context = BenchmarkMeasurementContext
  {
    what_is_measured : "Test measurement".to_string(),
    how_to_measure : "test command".to_string(),
    environment : EnvironmentContext
    {
      cpu : "Test CPU".to_string(),
      ram : "8GB".to_string(),
      storage : "SSD".to_string(),
      load_characteristics : "test load".to_string(),
      notes : vec![],
    },
    purpose : "Testing context".to_string(),
  };
  
  assert_eq!( context.what_is_measured, "Test measurement" );
  assert_eq!( context.how_to_measure, "test command" );
  assert_eq!( context.purpose, "Testing context" );
}

#[ test ]
fn test_time_formatting()
{
  assert_eq!( BeforeAfterComparison::format_time( 1_500_000_000.0 ), "1.50s" );
  assert_eq!( BeforeAfterComparison::format_time( 5_000_000.0 ), "5.00ms" );
  assert_eq!( BeforeAfterComparison::format_time( 800_000.0 ), "0.80ms" );
  assert_eq!( BeforeAfterComparison::format_time( 250_000.0 ), "0.25ms" );
  assert_eq!( BeforeAfterComparison::format_time( 1_500.0 ), "1.50µs" );
}

#[ test ]
fn test_generator_section_management()
{
  let mut generator = ContextRichDocGenerator::default_environment();
  
  // Initially empty
  assert_eq!( generator.section_count(), 0 );
  
  // Add some mock data to create a section
  let comparisons = vec![
    BeforeAfterComparison
    {
      algorithm_name : "test".to_string(),
      before_nanos : 1000.0,
      after_nanos : 900.0,
      status : OptimizationStatus::Optimized,
    },
  ];
  
  let context = BenchmarkMeasurementContext
  {
    what_is_measured : "Test".to_string(),
    how_to_measure : "test".to_string(),
    environment : generator.environment().clone(),
    purpose : "Test".to_string(),
  };
  
  generator.add_before_after_comparison( "Test", context, &comparisons );
  assert_eq!( generator.section_count(), 1 );
  
  // Clear sections
  generator.clear_sections();
  assert_eq!( generator.section_count(), 0 );
}

#[ test ]
fn test_report_generation_metadata()
{
  let generator = ContextRichDocGenerator::default_environment();
  let report = generator.generate_report( "Metadata Test Report" );
  
  assert!( report.contains( "Metadata Test Report" ) );
  assert!( report.contains( "Generated on" ) );
  assert!( report.contains( "context-rich benchmark documentation" ) );
  assert!( report.contains( "benchkit standards" ) );
}