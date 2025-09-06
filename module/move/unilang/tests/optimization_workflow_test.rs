//! Tests for before/after optimization workflow functionality

#![ cfg( feature = "benchmarks" ) ]
#![allow(clippy::too_many_lines)]

use std::fs;
use unilang::
{
  OptimizationWorkflow,
  BenchmarkResult,
  CoefficientsOfVariationAnalysis,
  OptimizationStatus,
};

#[ cfg( not( feature = "benchmarks" ) ) ]
fn main() {}

#[ test ]
fn test_workflow_creation()
{
  let temp_dir = std::env::temp_dir().join( "test_optimization_workflow" );
  
  let workflow = OptimizationWorkflow::new( 
    &temp_dir, 
    "test_benchmark".to_string() 
  ).expect( "Failed to create workflow" );
  
  // Directory should be created
  assert!( temp_dir.exists() );
  
  // Should have no baseline initially
  assert!( !workflow.has_baseline() );
  
  // Cleanup
  let _ = fs::remove_dir_all( temp_dir );
}

#[ test ] 
fn test_baseline_establishment_and_loading()
{
  let temp_dir = std::env::temp_dir().join( "test_baseline_workflow" );
  
  let workflow = OptimizationWorkflow::new(
    &temp_dir,
    "baseline_test".to_string()
  ).expect( "Failed to create workflow" );
  
  // Create test results
  let test_results = vec![
    BenchmarkResult
    {
      algorithm_name : "test_algorithm".to_string(),
      average_time_nanos : 1_000_000.0,
      std_dev_nanos : 50_000.0,
      min_time_nanos : 950_000,
      max_time_nanos : 1_050_000,
      sample_count : 100,
    }
  ];
  
  let cv_analysis = CoefficientsOfVariationAnalysis::new(
    vec![ 5.0 ], // 5% CV
    "Test Analysis".to_string()
  );
  
  // Establish baseline
  let baseline = workflow.establish_baseline(
    test_results.clone(),
    cv_analysis,
    "Test environment".to_string(),
    vec![ "Test note".to_string() ]
  ).expect( "Failed to establish baseline" );
  
  // Verify baseline properties
  assert_eq!( baseline.benchmark_name, "baseline_test" );
  assert_eq!( baseline.results.len(), 1 );
  assert_eq!( baseline.results[ 0 ].algorithm_name, "test_algorithm" );
  assert!( baseline.timestamp.contains( "UTC" ) );
  
  // Should now have baseline
  assert!( workflow.has_baseline() );
  
  // Should be able to load baseline
  let loaded = workflow.load_baseline_results().expect( "Failed to load baseline" );
  assert_eq!( loaded.benchmark_name, baseline.benchmark_name );
  assert_eq!( loaded.results.len(), baseline.results.len() );
  
  // Cleanup
  let _ = fs::remove_dir_all( temp_dir );
}

#[ test ]
fn test_optimization_impact_measurement()
{
  let temp_dir = std::env::temp_dir().join( "test_impact_workflow" );
  
  let mut workflow = OptimizationWorkflow::new(
    &temp_dir,
    "impact_test".to_string() 
  ).expect( "Failed to create workflow" );
  
  // Establish baseline
  let baseline_results = vec![
    BenchmarkResult
    {
      algorithm_name : "improved_algo".to_string(),
      average_time_nanos : 2_000_000.0, // 2ms
      std_dev_nanos : 100_000.0,
      min_time_nanos : 1_900_000,
      max_time_nanos : 2_100_000,
      sample_count : 100,
    },
    BenchmarkResult
    {
      algorithm_name : "regressed_algo".to_string(),
      average_time_nanos : 1_000_000.0, // 1ms  
      std_dev_nanos : 50_000.0,
      min_time_nanos : 950_000,
      max_time_nanos : 1_050_000,
      sample_count : 100,
    }
  ];
  
  let baseline_cv = CoefficientsOfVariationAnalysis::new(
    vec![ 5.0, 5.0 ],
    "Baseline Analysis".to_string()
  );
  
  let _baseline = workflow.establish_baseline(
    baseline_results,
    baseline_cv,
    "Test environment".to_string(),
    vec![]
  ).expect( "Failed to establish baseline" );
  
  // Create optimized results
  let optimized_results = vec![
    BenchmarkResult
    {
      algorithm_name : "improved_algo".to_string(), 
      average_time_nanos : 1_500_000.0, // Improved: 2ms -> 1.5ms (25% faster)
      std_dev_nanos : 75_000.0,
      min_time_nanos : 1_400_000,
      max_time_nanos : 1_600_000,
      sample_count : 100,
    },
    BenchmarkResult
    {
      algorithm_name : "regressed_algo".to_string(),
      average_time_nanos : 1_200_000.0, // Regression: 1ms -> 1.2ms (20% slower)
      std_dev_nanos : 60_000.0,
      min_time_nanos : 1_150_000,
      max_time_nanos : 1_250_000,
      sample_count : 100,
    }
  ];
  
  let current_cv = CoefficientsOfVariationAnalysis::new(
    vec![ 5.0, 5.0 ],
    "Current Analysis".to_string()
  );
  
  // Measure impact
  let impact = workflow.measure_optimization_impact(
    optimized_results,
    current_cv
  ).expect( "Failed to measure impact" );
  
  // Verify impact analysis
  assert_eq!( impact.comparisons.len(), 2 );
  
  // Check improved algorithm
  let improved = impact.comparisons.iter()
    .find( | c | c.algorithm_name == "improved_algo" )
    .expect( "Missing improved_algo comparison" );
  assert!( improved.improvement_percentage() > 20.0 );
  assert_eq!( improved.status, OptimizationStatus::ProductionReady );
  
  // Check regressed algorithm  
  let regressed = impact.comparisons.iter()
    .find( | c | c.algorithm_name == "regressed_algo" )
    .expect( "Missing regressed_algo comparison" );
  assert!( regressed.improvement_percentage() < -15.0 );
  assert_eq!( regressed.status, OptimizationStatus::NeedsWork );
  
  // Verify significance analysis
  assert_eq!( impact.significance_analysis.significant_improvements, 1 );
  assert_eq!( impact.significance_analysis.regressions, 1 );
  assert_eq!( impact.significance_analysis.total_algorithms, 2 );
  
  // Should not be considered successful due to regression
  assert!( !impact.summary.success );
  
  // Cleanup
  let _ = fs::remove_dir_all( temp_dir );
}

#[ test ]
fn test_optimization_status_determination()
{
  let temp_dir = std::env::temp_dir().join( "test_status_workflow" );
  
  let workflow = OptimizationWorkflow::new(
    &temp_dir,
    "status_test".to_string()
  ).expect( "Failed to create workflow" );
  
  // Test through impact measurement since status determination is private
  
  // Establish baseline
  let baseline_results = vec![
    BenchmarkResult
    {
      algorithm_name : "production_ready".to_string(),
      average_time_nanos : 2_000_000.0,
      std_dev_nanos : 100_000.0,
      min_time_nanos : 1_900_000,
      max_time_nanos : 2_100_000,
      sample_count : 100,
    },
    BenchmarkResult
    {
      algorithm_name : "optimized".to_string(),
      average_time_nanos : 1_000_000.0,
      std_dev_nanos : 50_000.0,
      min_time_nanos : 950_000,
      max_time_nanos : 1_050_000,
      sample_count : 100,
    },
    BenchmarkResult
    {
      algorithm_name : "baseline".to_string(),
      average_time_nanos : 500_000.0,
      std_dev_nanos : 25_000.0,
      min_time_nanos : 475_000,
      max_time_nanos : 525_000,
      sample_count : 100,
    },
    BenchmarkResult
    {
      algorithm_name : "needs_work".to_string(),
      average_time_nanos : 800_000.0,
      std_dev_nanos : 40_000.0,
      min_time_nanos : 760_000,
      max_time_nanos : 840_000,
      sample_count : 100,
    },
    BenchmarkResult
    {
      algorithm_name : "regression".to_string(),
      average_time_nanos : 600_000.0,
      std_dev_nanos : 30_000.0,
      min_time_nanos : 570_000,
      max_time_nanos : 630_000,
      sample_count : 100,
    }
  ];
  
  let cv_analysis = CoefficientsOfVariationAnalysis::new(
    vec![ 5.0; 5 ],
    "Status Test".to_string()
  );
  
  let _baseline = workflow.establish_baseline(
    baseline_results,
    cv_analysis.clone(),
    "Test environment".to_string(),
    vec![]
  ).expect( "Failed to establish baseline" );
  
  // Create results with different performance changes
  let optimized_results = vec![
    BenchmarkResult // 25% improvement -> ProductionReady
    {
      algorithm_name : "production_ready".to_string(),
      average_time_nanos : 1_500_000.0,
      std_dev_nanos : 75_000.0,
      min_time_nanos : 1_400_000,
      max_time_nanos : 1_600_000,
      sample_count : 100,
    },
    BenchmarkResult // 10% improvement -> Optimized
    {
      algorithm_name : "optimized".to_string(),
      average_time_nanos : 900_000.0,
      std_dev_nanos : 45_000.0,
      min_time_nanos : 855_000,
      max_time_nanos : 945_000,
      sample_count : 100,
    },
    BenchmarkResult // 2% improvement -> Baseline
    {
      algorithm_name : "baseline".to_string(),
      average_time_nanos : 490_000.0,
      std_dev_nanos : 24_500.0,
      min_time_nanos : 465_000,
      max_time_nanos : 515_000,
      sample_count : 100,
    },
    BenchmarkResult // 10% regression -> NeedsWork  
    {
      algorithm_name : "needs_work".to_string(),
      average_time_nanos : 880_000.0,
      std_dev_nanos : 44_000.0,
      min_time_nanos : 836_000,
      max_time_nanos : 924_000,
      sample_count : 100,
    },
    BenchmarkResult // 30% regression -> Regression
    {
      algorithm_name : "regression".to_string(),
      average_time_nanos : 780_000.0,
      std_dev_nanos : 39_000.0,
      min_time_nanos : 741_000,
      max_time_nanos : 819_000,
      sample_count : 100,
    }
  ];
  
  let mut workflow_mut = workflow;
  let impact = workflow_mut.measure_optimization_impact(
    optimized_results,
    cv_analysis
  ).expect( "Failed to measure impact" );
  
  // Verify status assignments
  let statuses : std::collections::HashMap< String, OptimizationStatus > = impact.comparisons.iter()
    .map( | c | ( c.algorithm_name.clone(), c.status.clone() ) )
    .collect();
    
  assert_eq!( statuses[ "production_ready" ], OptimizationStatus::ProductionReady );
  assert_eq!( statuses[ "optimized" ], OptimizationStatus::Optimized );
  assert_eq!( statuses[ "baseline" ], OptimizationStatus::Baseline );
  assert_eq!( statuses[ "needs_work" ], OptimizationStatus::NeedsWork );
  assert_eq!( statuses[ "regression" ], OptimizationStatus::Regression );
  
  // Cleanup
  let _ = fs::remove_dir_all( temp_dir );
}

#[ test ]
fn test_load_baseline_without_establishment()
{
  let temp_dir = std::env::temp_dir().join( "test_no_baseline_workflow" );
  
  let workflow = OptimizationWorkflow::new(
    &temp_dir,
    "no_baseline_test".to_string()
  ).expect( "Failed to create workflow" );
  
  // Should fail to load non-existent baseline
  let result = workflow.load_baseline_results();
  assert!( result.is_err() );
  
  let error = result.unwrap_err();
  assert_eq!( error.kind(), std::io::ErrorKind::NotFound );
  assert!( error.to_string().contains( "No baseline found" ) );
  
  // Cleanup
  let _ = fs::remove_dir_all( temp_dir );
}

#[ test ]
fn test_significance_analysis_calculations()
{
  let temp_dir = std::env::temp_dir().join( "test_significance_workflow" );
  
  let mut workflow = OptimizationWorkflow::new(
    &temp_dir,
    "significance_test".to_string()
  ).expect( "Failed to create workflow" );
  
  // Establish baseline with multiple algorithms
  let baseline_results = vec![
    BenchmarkResult
    {
      algorithm_name : "fast".to_string(),
      average_time_nanos : 1_000_000.0,
      std_dev_nanos : 50_000.0,
      min_time_nanos : 950_000,
      max_time_nanos : 1_050_000,
      sample_count : 100,
    },
    BenchmarkResult
    {
      algorithm_name : "medium".to_string(),
      average_time_nanos : 2_000_000.0,
      std_dev_nanos : 100_000.0,
      min_time_nanos : 1_900_000,
      max_time_nanos : 2_100_000,
      sample_count : 100,
    },
    BenchmarkResult
    {
      algorithm_name : "slow".to_string(),
      average_time_nanos : 3_000_000.0,
      std_dev_nanos : 150_000.0,
      min_time_nanos : 2_850_000,
      max_time_nanos : 3_150_000,
      sample_count : 100,
    }
  ];
  
  let cv_analysis = CoefficientsOfVariationAnalysis::new(
    vec![ 5.0, 5.0, 5.0 ],
    "Significance Test".to_string()
  );
  
  let _baseline = workflow.establish_baseline(
    baseline_results,
    cv_analysis.clone(),
    "Test environment".to_string(),
    vec![]
  ).expect( "Failed to establish baseline" );
  
  // Create mixed results: 1 improvement, 1 regression, 1 no change
  let optimized_results = vec![
    BenchmarkResult // 20% improvement (significant)
    {
      algorithm_name : "fast".to_string(),
      average_time_nanos : 800_000.0,
      std_dev_nanos : 40_000.0,
      min_time_nanos : 760_000,
      max_time_nanos : 840_000,
      sample_count : 100,
    },
    BenchmarkResult // 10% regression (significant)
    {
      algorithm_name : "medium".to_string(),
      average_time_nanos : 2_200_000.0,
      std_dev_nanos : 110_000.0,
      min_time_nanos : 2_090_000,
      max_time_nanos : 2_310_000,
      sample_count : 100,
    },
    BenchmarkResult // 2% improvement (not significant)
    {
      algorithm_name : "slow".to_string(),
      average_time_nanos : 2_940_000.0,
      std_dev_nanos : 147_000.0,
      min_time_nanos : 2_793_000,
      max_time_nanos : 3_087_000,
      sample_count : 100,
    }
  ];
  
  let impact = workflow.measure_optimization_impact(
    optimized_results,
    cv_analysis
  ).expect( "Failed to measure impact" );
  
  // Verify significance analysis
  let sig = &impact.significance_analysis;
  assert_eq!( sig.significant_improvements, 1 ); // Only "fast" algorithm
  assert_eq!( sig.regressions, 1 ); // Only "medium" algorithm
  assert_eq!( sig.total_algorithms, 3 );
  
  // Average should be positive due to one large improvement offsetting one regression
  let expected_avg = ( 20.0 - 10.0 + 2.0 ) / 3.0; // ~4%
  assert!( ( sig.average_improvement - expected_avg ).abs() < 1.0 );
  
  // Cleanup
  let _ = fs::remove_dir_all( temp_dir );
}