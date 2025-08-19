//! Comprehensive Regression Analysis Examples
//!
//! This example demonstrates EVERY aspect of the new Regression Analysis system:
//! - RegressionAnalyzer with all baseline strategies (Fixed, Rolling Average, Previous Run)
//! - HistoricalResults management and TimestampedResults creation
//! - Performance trend detection (Improving, Degrading, Stable)  
//! - Statistical significance testing with configurable thresholds
//! - Professional markdown report generation with regression insights
//! - Integration with PerformanceReport templates
//! - Real-world scenarios: code optimization, library upgrades, performance monitoring

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

/// Create current benchmark results showing performance improvements
fn create_current_results() -> HashMap< String, BenchmarkResult >
{
  let mut results = HashMap::new();
  
  // Fast sort algorithm - recently optimized, showing improvement
  let fast_sort_times = vec![
    Duration::from_micros( 85 ), Duration::from_micros( 88 ), Duration::from_micros( 82 ),
    Duration::from_micros( 87 ), Duration::from_micros( 84 ), Duration::from_micros( 86 ),
    Duration::from_micros( 89 ), Duration::from_micros( 81 ), Duration::from_micros( 88 ),
    Duration::from_micros( 85 ), Duration::from_micros( 87 ), Duration::from_micros( 83 ),
    Duration::from_micros( 86 ), Duration::from_micros( 84 ), Duration::from_micros( 88 )
  ];
  results.insert( "fast_sort".to_string(), BenchmarkResult::new( "fast_sort", fast_sort_times ) );
  
  // Hash function - stable performance
  let hash_times = vec![
    Duration::from_nanos( 150 ), Duration::from_nanos( 152 ), Duration::from_nanos( 148 ),
    Duration::from_nanos( 151 ), Duration::from_nanos( 149 ), Duration::from_nanos( 150 ),
    Duration::from_nanos( 153 ), Duration::from_nanos( 147 ), Duration::from_nanos( 151 ),
    Duration::from_nanos( 150 ), Duration::from_nanos( 152 ), Duration::from_nanos( 149 )
  ];
  results.insert( "hash_function".to_string(), BenchmarkResult::new( "hash_function", hash_times ) );
  
  // Memory allocator - performance regression after system update
  let allocator_times = vec![
    Duration::from_micros( 320 ), Duration::from_micros( 335 ), Duration::from_micros( 315 ),
    Duration::from_micros( 330 ), Duration::from_micros( 325 ), Duration::from_micros( 340 ),
    Duration::from_micros( 310 ), Duration::from_micros( 345 ), Duration::from_micros( 318 ),
    Duration::from_micros( 332 ), Duration::from_micros( 327 ), Duration::from_micros( 338 )
  ];
  results.insert( "memory_allocator".to_string(), BenchmarkResult::new( "memory_allocator", allocator_times ) );
  
  results
}

/// Create historical baseline data for fixed baseline strategy
fn create_baseline_historical_data() -> HistoricalResults
{
  let mut baseline_data = HashMap::new();
  
  // Baseline: fast_sort before optimization (slower performance)
  let baseline_fast_sort = vec![
    Duration::from_micros( 110 ), Duration::from_micros( 115 ), Duration::from_micros( 108 ),
    Duration::from_micros( 112 ), Duration::from_micros( 117 ), Duration::from_micros( 111 ),
    Duration::from_micros( 114 ), Duration::from_micros( 107 ), Duration::from_micros( 113 ),
    Duration::from_micros( 109 ), Duration::from_micros( 116 ), Duration::from_micros( 106 )
  ];
  baseline_data.insert( "fast_sort".to_string(), BenchmarkResult::new( "fast_sort", baseline_fast_sort ) );
  
  // Baseline: hash_function (similar performance)
  let baseline_hash = vec![
    Duration::from_nanos( 148 ), Duration::from_nanos( 152 ), Duration::from_nanos( 146 ),
    Duration::from_nanos( 150 ), Duration::from_nanos( 154 ), Duration::from_nanos( 147 ),
    Duration::from_nanos( 151 ), Duration::from_nanos( 149 ), Duration::from_nanos( 153 ),
    Duration::from_nanos( 148 ), Duration::from_nanos( 152 ), Duration::from_nanos( 150 )
  ];
  baseline_data.insert( "hash_function".to_string(), BenchmarkResult::new( "hash_function", baseline_hash ) );
  
  // Baseline: memory_allocator before system update (better performance)
  let baseline_allocator = vec![
    Duration::from_micros( 280 ), Duration::from_micros( 285 ), Duration::from_micros( 275 ),
    Duration::from_micros( 282 ), Duration::from_micros( 287 ), Duration::from_micros( 278 ),
    Duration::from_micros( 284 ), Duration::from_micros( 276 ), Duration::from_micros( 283 ),
    Duration::from_micros( 279 ), Duration::from_micros( 286 ), Duration::from_micros( 277 )
  ];
  baseline_data.insert( "memory_allocator".to_string(), BenchmarkResult::new( "memory_allocator", baseline_allocator ) );
  
  HistoricalResults::new().with_baseline( baseline_data )
}

/// Create historical runs for rolling average strategy
fn create_rolling_average_historical_data() -> HistoricalResults
{
  let mut historical_runs = Vec::new();
  
  // Historical run 1: 2 weeks ago
  let mut run1_results = HashMap::new();
  let run1_fast_sort = vec![ Duration::from_micros( 120 ), Duration::from_micros( 125 ), Duration::from_micros( 118 ) ];
  let run1_hash = vec![ Duration::from_nanos( 155 ), Duration::from_nanos( 160 ), Duration::from_nanos( 150 ) ];
  let run1_allocator = vec![ Duration::from_micros( 290 ), Duration::from_micros( 295 ), Duration::from_micros( 285 ) ];
  
  run1_results.insert( "fast_sort".to_string(), BenchmarkResult::new( "fast_sort", run1_fast_sort ) );
  run1_results.insert( "hash_function".to_string(), BenchmarkResult::new( "hash_function", run1_hash ) );
  run1_results.insert( "memory_allocator".to_string(), BenchmarkResult::new( "memory_allocator", run1_allocator ) );
  
  historical_runs.push( TimestampedResults::new(
    SystemTime::now() - Duration::from_secs( 1_209_600 ), // 2 weeks ago
    run1_results
  ) );
  
  // Historical run 2: 1 week ago
  let mut run2_results = HashMap::new();
  let run2_fast_sort = vec![ Duration::from_micros( 100 ), Duration::from_micros( 105 ), Duration::from_micros( 98 ) ];
  let run2_hash = vec![ Duration::from_nanos( 150 ), Duration::from_nanos( 155 ), Duration::from_nanos( 145 ) ];
  let run2_allocator = vec![ Duration::from_micros( 285 ), Duration::from_micros( 290 ), Duration::from_micros( 280 ) ];
  
  run2_results.insert( "fast_sort".to_string(), BenchmarkResult::new( "fast_sort", run2_fast_sort ) );
  run2_results.insert( "hash_function".to_string(), BenchmarkResult::new( "hash_function", run2_hash ) );
  run2_results.insert( "memory_allocator".to_string(), BenchmarkResult::new( "memory_allocator", run2_allocator ) );
  
  historical_runs.push( TimestampedResults::new(
    SystemTime::now() - Duration::from_secs( 604_800 ), // 1 week ago  
    run2_results
  ) );
  
  // Historical run 3: 3 days ago
  let mut run3_results = HashMap::new();
  let run3_fast_sort = vec![ Duration::from_micros( 95 ), Duration::from_micros( 98 ), Duration::from_micros( 92 ) ];
  let run3_hash = vec![ Duration::from_nanos( 148 ), Duration::from_nanos( 153 ), Duration::from_nanos( 147 ) ];
  let run3_allocator = vec![ Duration::from_micros( 305 ), Duration::from_micros( 310 ), Duration::from_micros( 300 ) ];
  
  run3_results.insert( "fast_sort".to_string(), BenchmarkResult::new( "fast_sort", run3_fast_sort ) );
  run3_results.insert( "hash_function".to_string(), BenchmarkResult::new( "hash_function", run3_hash ) );
  run3_results.insert( "memory_allocator".to_string(), BenchmarkResult::new( "memory_allocator", run3_allocator ) );
  
  historical_runs.push( TimestampedResults::new(
    SystemTime::now() - Duration::from_secs( 259_200 ), // 3 days ago
    run3_results
  ) );
  
  HistoricalResults::new().with_historical_runs( historical_runs )
}

/// Create previous run data for previous run strategy
fn create_previous_run_historical_data() -> HistoricalResults
{
  let mut previous_results = HashMap::new();
  
  // Previous run: yesterday's results
  let prev_fast_sort = vec![ Duration::from_micros( 90 ), Duration::from_micros( 95 ), Duration::from_micros( 88 ) ];
  let prev_hash = vec![ Duration::from_nanos( 149 ), Duration::from_nanos( 154 ), Duration::from_nanos( 146 ) ];
  let prev_allocator = vec![ Duration::from_micros( 295 ), Duration::from_micros( 300 ), Duration::from_micros( 290 ) ];
  
  previous_results.insert( "fast_sort".to_string(), BenchmarkResult::new( "fast_sort", prev_fast_sort ) );
  previous_results.insert( "hash_function".to_string(), BenchmarkResult::new( "hash_function", prev_hash ) );
  previous_results.insert( "memory_allocator".to_string(), BenchmarkResult::new( "memory_allocator", prev_allocator ) );
  
  let previous_run = TimestampedResults::new(
    SystemTime::now() - Duration::from_secs( 86_400 ), // 1 day ago
    previous_results
  );
  
  HistoricalResults::new().with_previous_run( previous_run )
}

/// Demonstrate Fixed Baseline Strategy
fn demonstrate_fixed_baseline_strategy()
{
  println!( "🎯 FIXED BASELINE STRATEGY DEMONSTRATION" );
  println!( "=========================================" );
  println!( "Comparing current performance against a fixed baseline measurement." );
  println!( "Use case: Long-term performance tracking against a stable reference point.\n" );
  
  let current_results = create_current_results();
  let historical = create_baseline_historical_data();
  
  // Create analyzer with strict significance threshold
  let analyzer = RegressionAnalyzer::new()
    .with_baseline_strategy( BaselineStrategy::FixedBaseline )
    .with_significance_threshold( 0.01 )  // 1% significance level (very strict)
    .with_trend_window( 5 );
  
  let regression_report = analyzer.analyze( &current_results, &historical );
  
  // Display analysis results
  println!( "📊 REGRESSION ANALYSIS RESULTS:" );
  println!( "--------------------------------" );
  
  for operation in [ "fast_sort", "hash_function", "memory_allocator" ]
  {
    if let Some( trend ) = regression_report.get_trend_for( operation )
    {
      let significance = if regression_report.is_statistically_significant( operation )
      {
        "✓ Statistically Significant"
      }
      else
      {
        "- Not Significant"
      };
      
      let trend_emoji = match trend
      {
        PerformanceTrend::Improving => "🟢 IMPROVING",
        PerformanceTrend::Degrading => "🔴 DEGRADING", 
        PerformanceTrend::Stable => "🟡 STABLE",
      };
      
      println!( "  {} - {} ({})", operation, trend_emoji, significance );
    }
  }
  
  // Generate markdown report
  let markdown_report = regression_report.format_markdown();
  println!( "\n📝 GENERATED MARKDOWN REPORT:" );
  println!( "------------------------------" );
  println!( "{}", markdown_report );
  println!( "\n" );
}

/// Demonstrate Rolling Average Strategy  
fn demonstrate_rolling_average_strategy()
{
  println!( "📈 ROLLING AVERAGE STRATEGY DEMONSTRATION" );
  println!( "==========================================" );
  println!( "Comparing current performance against rolling average of recent runs." );
  println!( "Use case: Detecting gradual performance trends over time.\n" );
  
  let current_results = create_current_results();
  let historical = create_rolling_average_historical_data();
  
  // Create analyzer optimized for trend detection
  let analyzer = RegressionAnalyzer::new()
    .with_baseline_strategy( BaselineStrategy::RollingAverage )
    .with_significance_threshold( 0.05 )  // 5% significance level (moderate)
    .with_trend_window( 3 );  // Look at last 3 runs for trend analysis
  
  let regression_report = analyzer.analyze( &current_results, &historical );
  
  // Display comprehensive analysis
  println!( "📊 TREND ANALYSIS RESULTS:" );
  println!( "--------------------------" );
  
  for operation in [ "fast_sort", "hash_function", "memory_allocator" ]
  {
    if regression_report.has_historical_data( operation )
    {
      let trend = regression_report.get_trend_for( operation ).unwrap();
      let significance = regression_report.is_statistically_significant( operation );
      
      println!( "  🔍 {} Analysis:", operation );
      println!( "     Trend: {:?}", trend );
      println!( "     Statistical Significance: {}", if significance { "Yes" } else { "No" } );
      println!( "     Historical Data Points: Available" );
      println!();
    }
  }
  
  // Check overall report status
  if regression_report.has_significant_changes()
  {
    println!( "⚠️  ALERT: Significant performance changes detected!" );
  }
  else
  {
    println!( "✅ STATUS: Performance within normal variation ranges" );
  }
  
  println!( "\n" );
}

/// Demonstrate Previous Run Strategy
fn demonstrate_previous_run_strategy()
{
  println!( "⏮️  PREVIOUS RUN STRATEGY DEMONSTRATION" );
  println!( "=======================================" );
  println!( "Comparing current performance against the immediate previous run." );
  println!( "Use case: Detecting immediate impact of recent changes.\n" );
  
  let current_results = create_current_results();
  let historical = create_previous_run_historical_data();
  
  // Create analyzer for immediate change detection  
  let analyzer = RegressionAnalyzer::new()
    .with_baseline_strategy( BaselineStrategy::PreviousRun )
    .with_significance_threshold( 0.10 )  // 10% significance level (lenient)
    .with_trend_window( 2 );  // Only compare current vs previous
  
  let regression_report = analyzer.analyze( &current_results, &historical );
  
  // Display immediate change analysis
  println!( "📊 IMMEDIATE CHANGE ANALYSIS:" );
  println!( "-----------------------------" );
  
  if regression_report.has_previous_run_data()
  {
    for operation in [ "fast_sort", "hash_function", "memory_allocator" ]
    {
      if let Some( trend ) = regression_report.get_trend_for( operation )
      {
        let change_indicator = match trend
        {
          PerformanceTrend::Improving => "↗️ Performance improved since last run",
          PerformanceTrend::Degrading => "↘️ Performance degraded since last run",
          PerformanceTrend::Stable => "➡️ Performance stable since last run",
        };
        
        println!( "  {} - {}", operation, change_indicator );
      }
    }
  }
  else
  {
    println!( "  ❌ No previous run data available for comparison" );
  }
  
  println!( "\n" );
}

/// Demonstrate comprehensive template integration
fn demonstrate_template_integration()
{
  println!( "📋 PERFORMANCE REPORT TEMPLATE INTEGRATION" );
  println!( "===========================================" );
  println!( "Demonstrating full integration with PerformanceReport templates." );
  println!( "Use case: Automated performance documentation with regression insights.\n" );
  
  let current_results = create_current_results();
  let historical = create_rolling_average_historical_data();
  
  // Create comprehensive performance report with regression analysis
  let template = PerformanceReport::new()
    .title( "Algorithm Performance Analysis with Regression Detection" )
    .add_context( "Comprehensive analysis after code optimization and system updates" )
    .include_statistical_analysis( true )
    .include_regression_analysis( true )
    .with_historical_data( historical )
    .add_custom_section( CustomSection::new(
      "Optimization Impact Analysis",
      r#"### Key Changes Made

- **fast_sort**: Applied cache-friendly memory access patterns
- **hash_function**: No changes (stable baseline)  
- **memory_allocator**: System update may have introduced overhead

### Expected Outcomes

- fast_sort should show significant improvement
- hash_function should remain stable
- memory_allocator performance needs investigation"#
    ) );
  
  match template.generate( &current_results )
  {
    Ok( report ) =>
    {
      println!( "✅ GENERATED COMPREHENSIVE PERFORMANCE REPORT:" );
      println!( "----------------------------------------------" );
      
      // Display key sections
      let lines : Vec< &str > = report.lines().collect();
      let mut in_regression_section = false;
      let mut regression_lines = Vec::new();
      
      for line in lines
      {
        if line.contains( "## Regression Analysis" )
        {
          in_regression_section = true;
        }
        else if line.starts_with( "## " ) && in_regression_section
        {
          break;
        }
        
        if in_regression_section
        {
          regression_lines.push( line );
        }
      }
      
      if !regression_lines.is_empty()
      {
        println!( "📊 REGRESSION ANALYSIS SECTION:" );
        for line in regression_lines.iter().take( 15 )  // Show first 15 lines
        {
          println!( "{}", line );
        }
        if regression_lines.len() > 15
        {
          println!( "... ({} more lines)", regression_lines.len() - 15 );
        }
      }
      
      // Report statistics
      let report_size = report.len();
      let line_count = report.matches( '\n' ).count();
      println!( "\n📈 REPORT STATISTICS:" );
      println!( "    Size: {} characters", report_size );
      println!( "    Lines: {} lines", line_count );
      println!( "    Includes: Executive Summary, Performance Results, Statistical Analysis, Regression Analysis, Custom Sections" );
    },
    Err( e ) =>
    {
      println!( "❌ ERROR generating report: {}", e );
    }
  }
  
  println!( "\n" );
}

/// Demonstrate statistical significance tuning
fn demonstrate_significance_tuning()
{
  println!( "🎛️  STATISTICAL SIGNIFICANCE TUNING" );
  println!( "===================================" );
  println!( "Demonstrating how different significance thresholds affect regression detection." );
  println!( "Use case: Calibrating sensitivity for different environments.\n" );
  
  let current_results = create_current_results();
  let historical = create_baseline_historical_data();
  
  let thresholds = vec![ 0.01, 0.05, 0.10, 0.20 ];
  
  for &threshold in &thresholds
  {
    println!( "📊 ANALYSIS WITH {}% SIGNIFICANCE THRESHOLD:", ( threshold * 100.0 ) as i32 );
    
    let analyzer = RegressionAnalyzer::new()
      .with_baseline_strategy( BaselineStrategy::FixedBaseline )
      .with_significance_threshold( threshold );
    
    let regression_report = analyzer.analyze( &current_results, &historical );
    
    let mut significant_count = 0;
    let operations = [ "fast_sort", "hash_function", "memory_allocator" ];
    
    for operation in &operations
    {
      if regression_report.is_statistically_significant( operation )
      {
        significant_count += 1;
      }
    }
    
    println!( "    Significant changes detected: {}/{}", significant_count, operations.len() );
    
    // Show specific results for fast_sort (known improvement)
    if regression_report.is_statistically_significant( "fast_sort" )
    {
      println!( "    fast_sort: ✓ Significant improvement detected" );
    }
    else
    {
      println!( "    fast_sort: - Improvement not statistically significant at this level" );
    }
    
    println!();
  }
  
  println!( "💡 TUNING GUIDANCE:" );
  println!( "   - Strict thresholds (1-5%): Production environments, critical systems" );
  println!( "   - Moderate thresholds (5-10%): Development, performance monitoring" );
  println!( "   - Lenient thresholds (10-20%): Early development, noisy environments\n" );
}

/// Main demonstration function
fn main()
{
  println!( "🚀 BENCHKIT REGRESSION ANALYSIS COMPREHENSIVE DEMO" );
  println!( "====================================================" );
  println!( "This example demonstrates every aspect of the new regression analysis system:\n" );
  
  // Core strategy demonstrations
  demonstrate_fixed_baseline_strategy();
  demonstrate_rolling_average_strategy();
  demonstrate_previous_run_strategy();
  
  // Advanced features
  demonstrate_template_integration();
  demonstrate_significance_tuning();
  
  println!( "✨ SUMMARY OF DEMONSTRATED FEATURES:" );
  println!( "=====================================" );
  println!( "✅ All three baseline strategies (Fixed, Rolling Average, Previous Run)" );
  println!( "✅ Performance trend detection (Improving, Degrading, Stable)" );
  println!( "✅ Statistical significance testing with configurable thresholds" );
  println!( "✅ Historical data management (baseline, runs, previous run)" );
  println!( "✅ Professional markdown report generation" );
  println!( "✅ Full PerformanceReport template integration" );
  println!( "✅ Real-world use cases and configuration guidance" );
  println!( "\n🎯 Ready for production use in performance monitoring workflows!" );
}

#[ cfg( not( feature = "enabled" ) ) ]
fn main()
{
  println!( "This example requires the 'enabled' feature." );
  println!( "Run with: cargo run --example regression_analysis_comprehensive --features enabled" );
}