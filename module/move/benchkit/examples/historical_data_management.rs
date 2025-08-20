//! Historical Data Management Examples
//!
//! This example demonstrates EVERY aspect of managing historical benchmark data:
//! - Creating and managing `HistoricalResults` with multiple data sources
//! - `TimestampedResults` creation and manipulation
//! - Data persistence patterns for long-term storage
//! - Historical data validation and cleanup
//! - Performance trend tracking across time periods
//! - Data migration and format evolution scenarios

#![ cfg( feature = "enabled" ) ]
#![ cfg( feature = "markdown_reports" ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::format_push_string ) ]
#![ allow( clippy::cast_lossless ) ]
#![ allow( clippy::cast_possible_truncation ) ]
#![ allow( clippy::cast_precision_loss ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::needless_raw_string_hashes ) ]
#![ allow( clippy::too_many_lines ) ]

use benchkit::prelude::*;
use std::collections::HashMap;
use std::time::{ Duration, SystemTime };

/// Simulate realistic benchmark results for different time periods
fn generate_realistic_benchmark_data( base_performance_micros : u64, variation_factor : f64, sample_count : usize ) -> Vec< Duration >
{
  let mut times = Vec::new();
  let base_nanos = base_performance_micros * 1000;
  
  for i in 0..sample_count
  {
    // Add realistic variation with some consistency
    #[allow(clippy::cast_sign_loss)]
    let variation = ( ( i as f64 * 0.1 ).sin() * variation_factor * base_nanos as f64 ) as u64;
    let time_nanos = base_nanos + variation;
    times.push( Duration::from_nanos( time_nanos ) );
  }
  
  times
}

/// Create a complete historical dataset spanning multiple months
fn create_comprehensive_historical_dataset() -> HistoricalResults
{
  let mut historical_runs = Vec::new();
  let now = SystemTime::now();
  
  // Algorithm performance evolution over 6 months
  let algorithms = vec![
    ( "quicksort", 100_u64 ),      // Started at 100μs, gradually optimized
    ( "mergesort", 150_u64 ),      // Started at 150μs, remained stable  
    ( "heapsort", 200_u64 ),       // Started at 200μs, slight degradation
    ( "bubblesort", 5000_u64 ),    // Started at 5ms, major optimization in month 3
  ];
  
  // Generate 6 months of weekly data (26 data points)
  for week in 0..26
  {
    let mut week_results = HashMap::new();
    #[allow(clippy::cast_sign_loss)]
    let timestamp = now - Duration::from_secs( ( week * 7 * 24 * 3600 ) as u64 );
    
    for ( algo_name, base_perf ) in &algorithms
    {
      let performance_factor = match *algo_name
      {
        "quicksort" =>
        {
          // Gradual optimization: 20% improvement over 6 months
          1.0 - ( week as f64 * 0.008 )
        },
        "mergesort" =>
        {
          // Stable performance with minor fluctuations
          1.0 + ( ( week as f64 * 0.5 ).sin() * 0.02 )
        },
        "heapsort" =>
        {
          // Slight degradation due to system changes
          1.0 + ( week as f64 * 0.005 )
        },
        "bubblesort" =>
        {
          // Major optimization at week 13 (3 months ago)
          if week <= 13 { 0.4 } else { 1.0 }  // 60% improvement
        },
        _ => 1.0,
      };
      
      #[allow(clippy::cast_sign_loss)]
      let adjusted_perf = ( *base_perf as f64 * performance_factor ) as u64;
      let times = generate_realistic_benchmark_data( adjusted_perf, 0.1, 15 );
      
      week_results.insert( (*algo_name).to_string(), BenchmarkResult::new( *algo_name, times ) );
    }
    
    historical_runs.push( TimestampedResults::new( timestamp, week_results ) );
  }
  
  // Create baseline data from the oldest measurement (6 months ago)
  let mut baseline_data = HashMap::new();
  for ( algo_name, base_perf ) in &algorithms
  {
    let baseline_times = generate_realistic_benchmark_data( *base_perf, 0.05, 20 );
    baseline_data.insert( (*algo_name).to_string(), BenchmarkResult::new( *algo_name, baseline_times ) );
  }
  
  HistoricalResults::new()
    .with_baseline( baseline_data )
    .with_historical_runs( historical_runs )
}

/// Demonstrate building historical data incrementally
fn demonstrate_incremental_data_building()
{
  println!( "🏗️ INCREMENTAL HISTORICAL DATA BUILDING" );
  println!( "=======================================" );
  println!( "Demonstrating how to build historical datasets incrementally over time.\n" );
  
  // Start with empty historical data
  let mut historical = HistoricalResults::new();
  println!( "📊 Starting with empty historical dataset..." );
  
  // Add initial baseline
  let mut baseline_data = HashMap::new();
  let baseline_times = vec![ Duration::from_micros( 100 ), Duration::from_micros( 105 ), Duration::from_micros( 95 ) ];
  baseline_data.insert( "algorithm_v1".to_string(), BenchmarkResult::new( "algorithm_v1", baseline_times ) );
  
  historical = historical.with_baseline( baseline_data );
  println!( "✅ Added baseline measurement (algorithm_v1: ~100μs)" );
  
  // Simulate adding measurements over time
  let mut runs = Vec::new();
  let timestamps = vec![
    ( "1 month ago", SystemTime::now() - Duration::from_secs( 30 * 24 * 3600 ), 90_u64 ),
    ( "2 weeks ago", SystemTime::now() - Duration::from_secs( 14 * 24 * 3600 ), 85_u64 ),
    ( "1 week ago", SystemTime::now() - Duration::from_secs( 7 * 24 * 3600 ), 80_u64 ),
    ( "Yesterday", SystemTime::now() - Duration::from_secs( 24 * 3600 ), 75_u64 ),
  ];
  
  for ( description, timestamp, perf_micros ) in timestamps
  {
    let mut run_results = HashMap::new();
    let times = vec![ 
      Duration::from_micros( perf_micros ),
      Duration::from_micros( perf_micros + 2 ),
      Duration::from_micros( perf_micros - 2 )
    ];
    run_results.insert( "algorithm_v1".to_string(), BenchmarkResult::new( "algorithm_v1", times ) );
    
    runs.push( TimestampedResults::new( timestamp, run_results ) );
    println!( "📈 Added measurement from {} (~{}μs)", description, perf_micros );
  }
  
  let runs_count = runs.len();  // Store count before moving
  historical = historical.with_historical_runs( runs );
  
  // Add most recent measurement as previous run
  let mut previous_results = HashMap::new();
  let previous_times = vec![ Duration::from_micros( 72 ), Duration::from_micros( 74 ), Duration::from_micros( 70 ) ];
  previous_results.insert( "algorithm_v1".to_string(), BenchmarkResult::new( "algorithm_v1", previous_times ) );
  
  let previous_run = TimestampedResults::new(
    SystemTime::now() - Duration::from_secs( 3600 ), // 1 hour ago
    previous_results
  );
  historical = historical.with_previous_run( previous_run );
  
  println!( "⏮️ Added previous run measurement (~72μs)" );
  println!( "\n✨ Complete historical dataset built with {} data points!", runs_count + 2 );
  
  // Analyze the trend
  let current_results = {
    let mut current = HashMap::new();
    let current_times = vec![ Duration::from_micros( 70 ), Duration::from_micros( 72 ), Duration::from_micros( 68 ) ];
    current.insert( "algorithm_v1".to_string(), BenchmarkResult::new( "algorithm_v1", current_times ) );
    current
  };
  
  let analyzer = RegressionAnalyzer::new()
    .with_baseline_strategy( BaselineStrategy::RollingAverage )
    .with_trend_window( 4 );
  
  let regression_report = analyzer.analyze( &current_results, &historical );
  
  if let Some( trend ) = regression_report.get_trend_for( "algorithm_v1" )
  {
    println!( "📊 DETECTED TREND: {:?}", trend );
    println!( "    Performance has improved ~30% from baseline (100μs → 70μs)" );
  }
  
  println!( "\n" );
}

/// Demonstrate data validation and cleanup
fn demonstrate_data_validation_and_cleanup()
{
  println!( "🧹 HISTORICAL DATA VALIDATION AND CLEANUP" );
  println!( "==========================================" );
  println!( "Demonstrating validation of historical data quality and cleanup procedures.\n" );
  
  // Create dataset with quality issues
  let mut problematic_runs = Vec::new();
  let now = SystemTime::now();
  
  // Good data point
  let mut good_results = HashMap::new();
  let good_times = generate_realistic_benchmark_data( 100, 0.05, 15 );
  good_results.insert( "stable_algo".to_string(), BenchmarkResult::new( "stable_algo", good_times ) );
  problematic_runs.push( TimestampedResults::new( now - Duration::from_secs( 7 * 24 * 3600 ), good_results ) );
  
  // Noisy data point (high variance)
  let mut noisy_results = HashMap::new();
  let noisy_times = vec![
    Duration::from_micros( 80 ), Duration::from_micros( 200 ), Duration::from_micros( 90 ),
    Duration::from_micros( 300 ), Duration::from_micros( 85 ), Duration::from_micros( 150 ),
  ];
  noisy_results.insert( "stable_algo".to_string(), BenchmarkResult::new( "stable_algo", noisy_times ) );
  problematic_runs.push( TimestampedResults::new( now - Duration::from_secs( 6 * 24 * 3600 ), noisy_results ) );
  
  // Insufficient samples
  let mut sparse_results = HashMap::new();
  let sparse_times = vec![ Duration::from_micros( 95 ), Duration::from_micros( 105 ) ];  // Only 2 samples
  sparse_results.insert( "stable_algo".to_string(), BenchmarkResult::new( "stable_algo", sparse_times ) );
  problematic_runs.push( TimestampedResults::new( now - Duration::from_secs( 5 * 24 * 3600 ), sparse_results ) );
  
  // Another good data point
  let mut good_results2 = HashMap::new();
  let good_times2 = generate_realistic_benchmark_data( 98, 0.08, 12 );
  good_results2.insert( "stable_algo".to_string(), BenchmarkResult::new( "stable_algo", good_times2 ) );
  problematic_runs.push( TimestampedResults::new( now - Duration::from_secs( 4 * 24 * 3600 ), good_results2 ) );
  
  let historical = HistoricalResults::new().with_historical_runs( problematic_runs );
  
  println!( "📋 ORIGINAL DATASET: {} historical runs", historical.historical_runs().len() );
  
  // Create validator for quality assessment
  let validator = BenchmarkValidator::new()
    .min_samples( 10 )
    .max_coefficient_variation( 0.15 )
    .max_time_ratio( 2.0 );
  
  // Validate each historical run
  let mut quality_report = Vec::new();
  for ( i, timestamped_run ) in historical.historical_runs().iter().enumerate()
  {
    let run_validation = ValidatedResults::new( timestamped_run.results().clone(), validator.clone() );
    let reliability = run_validation.reliability_rate();
    
    quality_report.push( ( i, reliability, run_validation.reliability_warnings() ) );
    
    println!( "📊 Run {} - Reliability: {:.1}%", i + 1, reliability );
    if let Some( warnings ) = run_validation.reliability_warnings()
    {
      for warning in warnings
      {
        println!( "    ⚠️ {}", warning );
      }
    }
  }
  
  // Filter out low-quality runs
  let quality_threshold = 80.0;
  let high_quality_indices : Vec< usize > = quality_report.iter()
    .filter_map( | ( i, reliability, _ ) | if *reliability >= quality_threshold { Some( *i ) } else { None } )
    .collect();
  
  println!( "\n🔍 QUALITY FILTERING RESULTS:" );
  println!( "    Runs meeting quality threshold ({}%): {}/{}", quality_threshold, high_quality_indices.len(), quality_report.len() );
  println!( "    High-quality run indices: {:?}", high_quality_indices );
  
  // Demonstrate cleanup procedure
  println!( "\n🧹 CLEANUP RECOMMENDATIONS:" );
  if high_quality_indices.len() < quality_report.len()
  {
    println!( "    ❌ Remove {} low-quality runs", quality_report.len() - high_quality_indices.len() );
    println!( "    ✅ Retain {} high-quality runs", high_quality_indices.len() );
    println!( "    💡 Consider re-running benchmarks for removed time periods" );
  }
  else
  {
    println!( "    ✅ All historical runs meet quality standards" );
    println!( "    💡 Dataset ready for regression analysis" );
  }
  
  println!( "\n" );
}

/// Demonstrate performance trend analysis across different time windows
fn demonstrate_trend_analysis()
{
  println!( "📈 PERFORMANCE TREND ANALYSIS" );
  println!( "==============================" );
  println!( "Analyzing performance trends across different time windows and granularities.\n" );
  
  let historical = create_comprehensive_historical_dataset();
  let runs = historical.historical_runs();
  
  println!( "📊 HISTORICAL DATASET SUMMARY:" );
  println!( "    Total historical runs: {}", runs.len() );
  println!( "    Time span: ~6 months of weekly measurements" );
  println!( "    Algorithms tracked: quicksort, mergesort, heapsort, bubblesort\n" );
  
  // Analyze different algorithms with current results
  let mut current_results = HashMap::new();
  current_results.insert( "quicksort".to_string(), BenchmarkResult::new( "quicksort", vec![ Duration::from_micros( 80 ), Duration::from_micros( 82 ), Duration::from_micros( 78 ) ] ) );
  current_results.insert( "mergesort".to_string(), BenchmarkResult::new( "mergesort", vec![ Duration::from_micros( 155 ), Duration::from_micros( 158 ), Duration::from_micros( 152 ) ] ) );
  current_results.insert( "heapsort".to_string(), BenchmarkResult::new( "heapsort", vec![ Duration::from_micros( 210 ), Duration::from_micros( 215 ), Duration::from_micros( 205 ) ] ) );
  current_results.insert( "bubblesort".to_string(), BenchmarkResult::new( "bubblesort", vec![ Duration::from_micros( 2000 ), Duration::from_micros( 2050 ), Duration::from_micros( 1950 ) ] ) );
  
  // Different trend window analyses
  let trend_windows = vec![ 4, 8, 12, 20 ];
  
  for &window in &trend_windows
  {
    println!( "🔍 TREND ANALYSIS (Last {} weeks):", window );
    
    let analyzer = RegressionAnalyzer::new()
      .with_baseline_strategy( BaselineStrategy::RollingAverage )
      .with_trend_window( window )
      .with_significance_threshold( 0.10 );
    
    let regression_report = analyzer.analyze( &current_results, &historical );
    
    for algorithm in [ "quicksort", "mergesort", "heapsort", "bubblesort" ]
    {
      if let Some( trend ) = regression_report.get_trend_for( algorithm )
      {
        let trend_description = match trend
        {
          PerformanceTrend::Improving => "🟢 Improving",
          PerformanceTrend::Degrading => "🔴 Degrading", 
          PerformanceTrend::Stable => "🟡 Stable",
        };
        
        let significance = if regression_report.is_statistically_significant( algorithm )
        {
          " (Significant)"
        }
        else
        {
          " (Not significant)"
        };
        
        println!( "    {}: {}{}", algorithm, trend_description, significance );
      }
    }
    println!();
  }
  
  // Expected results explanation
  println!( "💡 EXPECTED TREND PATTERNS:" );
  println!( "    quicksort: Should show consistent improvement (20% optimization over 6 months)" );
  println!( "    mergesort: Should show stable performance (minor fluctuations only)" );
  println!( "    heapsort: Should show slight degradation (system changes impact)" );
  println!( "    bubblesort: Should show major improvement (60% optimization 3 months ago)" );
  println!( "\n" );
}

/// Demonstrate data persistence and serialization patterns
fn demonstrate_data_persistence_patterns()
{
  println!( "💾 DATA PERSISTENCE AND SERIALIZATION PATTERNS" );
  println!( "===============================================" );
  println!( "Demonstrating approaches for persisting historical benchmark data.\n" );
  
  let historical = create_comprehensive_historical_dataset();
  
  // Simulate different persistence strategies
  println!( "📁 PERSISTENCE STRATEGY OPTIONS:" );
  println!( "    1. JSON serialization for human-readable storage" );
  println!( "    2. Binary serialization for compact storage" );
  println!( "    3. Database storage for querying and analysis" );
  println!( "    4. File-per-run for incremental updates\n" );
  
  // Demonstrate JSON-like structure (conceptual)
  println!( "📄 JSON STRUCTURE EXAMPLE (conceptual):" );
  println!( r#"{{
  "baseline_data": {{
    "quicksort": {{
      "measurements": [100, 105, 95, ...],
      "timestamp": "2024-01-01T00:00:00Z"
    }}
  }},
  "historical_runs": [
    {{
      "timestamp": "2024-01-07T00:00:00Z",
      "results": {{
        "quicksort": {{ "measurements": [98, 102, 94, ...] }}
      }}
    }},
    ...
  ],
  "previous_run": {{
    "timestamp": "2024-06-30T00:00:00Z",
    "results": {{ ... }}
  }}
}}"# );
  
  // Analyze storage requirements
  let runs_count = historical.historical_runs().len();
  let algorithms_count = 4;  // quicksort, mergesort, heapsort, bubblesort
  let measurements_per_run = 15;  // average
  
  let estimated_json_size = runs_count * algorithms_count * measurements_per_run * 20;  // ~20 bytes per measurement in JSON
  let estimated_binary_size = runs_count * algorithms_count * measurements_per_run * 8;  // ~8 bytes per measurement in binary
  
  println!( "\n📊 STORAGE REQUIREMENTS ESTIMATE:" );
  println!( "    Historical runs: {}", runs_count );
  println!( "    Algorithms tracked: {}", algorithms_count );
  println!( "    Average measurements per run: {}", measurements_per_run );
  println!( "    Estimated JSON size: ~{} KB", estimated_json_size / 1024 );
  println!( "    Estimated binary size: ~{} KB", estimated_binary_size / 1024 );
  
  // Demonstrate incremental update pattern
  println!( "\n🔄 INCREMENTAL UPDATE PATTERNS:" );
  println!( "    ✅ Append new measurements to existing dataset" );
  println!( "    ✅ Rotate old data beyond retention period" );
  println!( "    ✅ Compress historical data for long-term storage" );
  println!( "    ✅ Maintain separate baseline and rolling data" );
  
  // Data retention recommendations  
  println!( "\n🗂️ DATA RETENTION RECOMMENDATIONS:" );
  println!( "    Development: Keep 3-6 months of daily measurements" );
  println!( "    Production: Keep 1-2 years of weekly measurements" );
  println!( "    Archive: Keep quarterly snapshots indefinitely" );
  println!( "    Cleanup: Remove incomplete or invalid measurements" );
  
  println!( "\n" );
}

/// Main demonstration function
fn main()
{
  println!( "🏛️ BENCHKIT HISTORICAL DATA MANAGEMENT COMPREHENSIVE DEMO" );
  println!( "===========================================================" );
  println!( "This example demonstrates every aspect of managing historical benchmark data:\n" );
  
  // Core data management demonstrations
  demonstrate_incremental_data_building();
  demonstrate_data_validation_and_cleanup();
  demonstrate_trend_analysis();
  demonstrate_data_persistence_patterns();
  
  println!( "✨ SUMMARY OF DEMONSTRATED CAPABILITIES:" );
  println!( "=======================================" );
  println!( "✅ Incremental historical data building and management" );
  println!( "✅ TimestampedResults creation with realistic time spans" );
  println!( "✅ Data quality validation and cleanup procedures" );
  println!( "✅ Performance trend analysis across multiple time windows" );
  println!( "✅ Storage and serialization strategy recommendations" );
  println!( "✅ Data retention and archival best practices" );
  println!( "✅ Integration with RegressionAnalyzer for trend detection" );
  println!( "\n🎯 Ready for production deployment with long-term performance monitoring!" );
}

#[ cfg( not( feature = "enabled" ) ) ]
fn main()
{
  println!( "This example requires the 'enabled' feature." );
  println!( "Run with: cargo run --example historical_data_management --features enabled" );
}