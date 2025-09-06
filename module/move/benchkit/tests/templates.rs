//! Tests for template system functionality

#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::float_cmp ) ]

#[ cfg( feature = "integration" ) ]
#[ cfg( feature = "markdown_reports" ) ]
mod tests
{
  use benchkit::prelude::*;
  use std::collections::HashMap;
  use std::time::{ Duration, SystemTime };

  fn create_sample_results() -> HashMap< String, BenchmarkResult >
  {
    let mut results = HashMap::new();
    
    // Fast operation with good reliability
    let fast_times = vec![ 
      Duration::from_micros( 100 ), Duration::from_micros( 102 ), Duration::from_micros( 98 ),
      Duration::from_micros( 101 ), Duration::from_micros( 99 ), Duration::from_micros( 100 ),
      Duration::from_micros( 103 ), Duration::from_micros( 97 ), Duration::from_micros( 101 ),
      Duration::from_micros( 100 ), Duration::from_micros( 102 ), Duration::from_micros( 99 )
    ];
    results.insert( "fast_operation".to_string(), BenchmarkResult::new( "fast_operation", fast_times ) );
    
    // Slow operation with poor reliability
    let slow_times = vec![ 
      Duration::from_millis( 10 ), Duration::from_millis( 15 ), Duration::from_millis( 8 ),
      Duration::from_millis( 12 ), Duration::from_millis( 20 ), Duration::from_millis( 9 )
    ];
    results.insert( "slow_operation".to_string(), BenchmarkResult::new( "slow_operation", slow_times ) );
    
    results
  }

  #[ test ]
  fn test_performance_report_basic()
  {
    let results = create_sample_results();
    let template = PerformanceReport::new()
      .title( "Test Performance Analysis" )
      .add_context( "Comparing fast vs slow operations" );
    
    let report = template.generate( &results ).unwrap();
    
    // Check structure
    assert!( report.contains( "# Test Performance Analysis" ) );
    assert!( report.contains( "Comparing fast vs slow operations" ) );
    assert!( report.contains( "## Executive Summary" ) );
    assert!( report.contains( "## Performance Results" ) );
    assert!( report.contains( "## Statistical Analysis" ) );
    assert!( report.contains( "## Methodology" ) );
    
    // Check content
    assert!( report.contains( "fast_operation" ) );
    assert!( report.contains( "slow_operation" ) );
    
    assert!( report.contains( "**Total operations benchmarked**: 2" ) );
  }

  #[ test ]
  fn test_performance_report_with_options()
  {
    let results = create_sample_results();
    let template = PerformanceReport::new()
      .title( "Custom Report" )
      .include_statistical_analysis( false )
      .include_regression_analysis( true )
      .add_custom_section( CustomSection::new( "Custom Analysis", "This is custom content." ) );
    
    let report = template.generate( &results ).unwrap();
    
    // Statistical analysis should be excluded
    assert!( !report.contains( "## Statistical Analysis" ) );
    
    // Regression analysis should be included
    assert!( report.contains( "## Regression Analysis" ) );
    
    // Custom section should be included
    assert!( report.contains( "## Custom Analysis" ) );
    assert!( report.contains( "This is custom content." ) );
  }

  #[ test ]
  fn test_comparison_report_basic()
  {
    let results = create_sample_results();
    let template = ComparisonReport::new()
      .title( "Fast vs Slow Comparison" )
      .baseline( "slow_operation" )
      .candidate( "fast_operation" )
      .significance_threshold( 0.05 )
      .practical_significance_threshold( 0.10 );
    
    let report = template.generate( &results ).unwrap();
    
    // Check structure
    assert!( report.contains( "# Fast vs Slow Comparison" ) );
    assert!( report.contains( "## Comparison Summary" ) );
    assert!( report.contains( "## Detailed Comparison" ) );
    assert!( report.contains( "## Statistical Analysis" ) );
    assert!( report.contains( "## Reliability Assessment" ) );
    assert!( report.contains( "## Methodology" ) );
    
    // Should detect improvement
    assert!( report.contains( "faster" ) );
    
    // Check that both algorithms are in the table
    assert!( report.contains( "fast_operation" ) );
    assert!( report.contains( "slow_operation" ) );
  }

  #[ test ]
  fn test_comparison_report_missing_baseline()
  {
    let results = create_sample_results();
    let template = ComparisonReport::new()
      .baseline( "nonexistent_operation" )
      .candidate( "fast_operation" );
    
    let result = template.generate( &results );
    assert!( result.is_err() );
    assert!( result.unwrap_err().to_string().contains( "nonexistent_operation" ) );
  }

  #[ test ]
  fn test_comparison_report_missing_candidate()
  {
    let results = create_sample_results();
    let template = ComparisonReport::new()
      .baseline( "fast_operation" )
      .candidate( "nonexistent_operation" );
    
    let result = template.generate( &results );
    assert!( result.is_err() );
    assert!( result.unwrap_err().to_string().contains( "nonexistent_operation" ) );
  }

  #[ test ]
  fn test_performance_report_empty_results()
  {
    let results = HashMap::new();
    let template = PerformanceReport::new();
    
    let report = template.generate( &results ).unwrap();
    
    assert!( report.contains( "No benchmark results available." ) );
    assert!( report.contains( "# Performance Analysis" ) );
  }

  #[ test ]
  fn test_custom_section()
  {
    let section = CustomSection::new( "Test Section", "Test content with *markdown*." );
    
    assert_eq!( section.title, "Test Section" );
    assert_eq!( section.content, "Test content with *markdown*." );
  }

  #[ test ]
  fn test_performance_report_reliability_analysis()
  {
    let results = create_sample_results();
    let template = PerformanceReport::new()
      .include_statistical_analysis( true );
    
    let report = template.generate( &results ).unwrap();
    
    // Should have reliability analysis sections
    assert!( report.contains( "Reliable Results" ) || report.contains( "Measurements Needing Attention" ) );
    
    // Should contain reliability indicators
    assert!( report.contains( "✅" ) || report.contains( "⚠️" ) );
  }

  #[ test ]
  fn test_comparison_report_confidence_intervals()
  {
    let results = create_sample_results();
    let template = ComparisonReport::new()
      .baseline( "slow_operation" )
      .candidate( "fast_operation" );
    
    let report = template.generate( &results ).unwrap();
    
    // Should mention confidence intervals
    assert!( report.contains( "95% CI" ) );
    assert!( report.contains( "Confidence intervals" ) || report.contains( "confidence interval" ) );
    
    // Should have statistical analysis
    assert!( report.contains( "Performance ratio" ) );
    assert!( report.contains( "Improvement" ) );
  }

  #[ test ]
  fn test_performance_report_default_values()
  {
    let template = PerformanceReport::default();
    let results = create_sample_results();
    
    let report = template.generate( &results ).unwrap();
    
    // Should use default title
    assert!( report.contains( "# Performance Analysis" ) );
    
    // Should include statistical analysis by default
    assert!( report.contains( "## Statistical Analysis" ) );
    
    // Should not include regression analysis by default
    assert!( !report.contains( "## Regression Analysis" ) );
  }

  #[ test ]
  fn test_comparison_report_default_values()
  {
    let template = ComparisonReport::default();
    
    // Check default values
    assert_eq!( template.baseline_name(), "Baseline" );
    assert_eq!( template.candidate_name(), "Candidate" );
    assert_eq!( template.significance_threshold_value(), 0.05 );
    assert_eq!( template.practical_significance_threshold_value(), 0.10 );
  }

  #[ test ]
  fn test_performance_report_with_regression_analysis()
  {
    let results = create_sample_results();
    
    // Create historical data for regression analysis
    let mut baseline_data = HashMap::new();
    let baseline_times = vec![ 
      Duration::from_micros( 120 ), Duration::from_micros( 118 ), Duration::from_micros( 122 ),
      Duration::from_micros( 119 ), Duration::from_micros( 121 ), Duration::from_micros( 120 ),
      Duration::from_micros( 123 ), Duration::from_micros( 117 ), Duration::from_micros( 121 ),
      Duration::from_micros( 120 ), Duration::from_micros( 122 ), Duration::from_micros( 119 )
    ];
    baseline_data.insert( "fast_operation".to_string(), BenchmarkResult::new( "fast_operation", baseline_times ) );
    
    let historical = HistoricalResults::new()
      .with_baseline( baseline_data );
    
    let template = PerformanceReport::new()
      .title( "Performance Report with Regression Analysis" )
      .include_regression_analysis( true )
      .with_historical_data( historical );
    
    let report = template.generate( &results ).unwrap();
    
    // Should include regression analysis section
    assert!( report.contains( "## Regression Analysis" ) );
    
    // Should detect performance improvement (100μs current vs 120μs baseline)
    assert!( report.contains( "Performance improvement detected" ) || report.contains( "faster than baseline" ) );
    
    // Should not show placeholder message when historical data is available
    assert!( !report.contains( "Not yet implemented" ) );
  }

  #[ test ]
  fn test_regression_analyzer_fixed_baseline_strategy()
  {
    let results = create_sample_results();
    
    // Create baseline with slower performance
    let mut baseline_data = HashMap::new();
    let baseline_times = vec![ 
      Duration::from_micros( 150 ), Duration::from_micros( 148 ), Duration::from_micros( 152 ),
      Duration::from_micros( 149 ), Duration::from_micros( 151 ), Duration::from_micros( 150 )
    ];
    baseline_data.insert( "fast_operation".to_string(), BenchmarkResult::new( "fast_operation", baseline_times ) );
    
    let historical = HistoricalResults::new()
      .with_baseline( baseline_data );
    
    let analyzer = RegressionAnalyzer::new()
      .with_baseline_strategy( BaselineStrategy::FixedBaseline )
      .with_significance_threshold( 0.05 );
    
    let regression_report = analyzer.analyze( &results, &historical );
    
    // Should detect significant improvement
    assert!( regression_report.has_significant_changes() );
    assert!( regression_report.get_trend_for( "fast_operation" ) == Some( PerformanceTrend::Improving ) );
    
    // Should include statistical significance
    assert!( regression_report.is_statistically_significant( "fast_operation" ) );
  }

  #[ test ]
  fn test_regression_analyzer_rolling_average_strategy()
  {
    let results = create_sample_results();
    
    // Create historical runs showing gradual improvement
    let mut historical_runs = Vec::new();
    
    // Run 1: Slower performance
    let mut run1_results = HashMap::new();
    let run1_times = vec![ Duration::from_micros( 140 ), Duration::from_micros( 142 ), Duration::from_micros( 138 ) ];
    run1_results.insert( "fast_operation".to_string(), BenchmarkResult::new( "fast_operation", run1_times ) );
    historical_runs.push( TimestampedResults::new( 
      SystemTime::now() - Duration::from_secs( 604_800 ), // 1 week ago
      run1_results 
    ) );
    
    // Run 2: Medium performance  
    let mut run2_results = HashMap::new();
    let run2_times = vec![ Duration::from_micros( 120 ), Duration::from_micros( 122 ), Duration::from_micros( 118 ) ];
    run2_results.insert( "fast_operation".to_string(), BenchmarkResult::new( "fast_operation", run2_times ) );
    historical_runs.push( TimestampedResults::new(
      SystemTime::now() - Duration::from_secs( 86400 ), // 1 day ago
      run2_results
    ) );
    
    let historical = HistoricalResults::new()
      .with_historical_runs( historical_runs );
    
    let analyzer = RegressionAnalyzer::new()
      .with_baseline_strategy( BaselineStrategy::RollingAverage )
      .with_trend_window( 3 );
    
    let regression_report = analyzer.analyze( &results, &historical );
    
    // Should detect improving trend from rolling average
    assert!( regression_report.get_trend_for( "fast_operation" ) == Some( PerformanceTrend::Improving ) );
    assert!( regression_report.has_historical_data( "fast_operation" ) );
  }

  #[ test ]
  fn test_regression_analyzer_previous_run_strategy()
  {
    let results = create_sample_results();
    
    // Create single previous run with worse performance
    let mut previous_results = HashMap::new();
    let previous_times = vec![ Duration::from_micros( 130 ), Duration::from_micros( 132 ), Duration::from_micros( 128 ) ];
    previous_results.insert( "fast_operation".to_string(), BenchmarkResult::new( "fast_operation", previous_times ) );
    
    let historical = HistoricalResults::new()
      .with_previous_run( TimestampedResults::new( 
        SystemTime::now() - Duration::from_secs( 3600 ), // 1 hour ago
        previous_results 
      ) );
    
    let analyzer = RegressionAnalyzer::new()
      .with_baseline_strategy( BaselineStrategy::PreviousRun );
    
    let regression_report = analyzer.analyze( &results, &historical );
    
    // Should detect improvement compared to previous run
    assert!( regression_report.get_trend_for( "fast_operation" ) == Some( PerformanceTrend::Improving ) );
    assert!( regression_report.has_previous_run_data() );
  }

  #[ test ]
  fn test_regression_analyzer_statistical_significance()
  {
    let results = create_sample_results();
    
    // Create baseline with very similar performance (should not be significant)
    let mut baseline_data = HashMap::new();
    let baseline_times = vec![ 
      Duration::from_micros( 101 ), Duration::from_micros( 99 ), Duration::from_micros( 102 ),
      Duration::from_micros( 100 ), Duration::from_micros( 98 ), Duration::from_micros( 101 )
    ];
    baseline_data.insert( "fast_operation".to_string(), BenchmarkResult::new( "fast_operation", baseline_times ) );
    
    let historical = HistoricalResults::new()
      .with_baseline( baseline_data );
    
    let analyzer = RegressionAnalyzer::new()
      .with_significance_threshold( 0.01 ); // Very strict threshold
    
    let regression_report = analyzer.analyze( &results, &historical );
    
    // Should detect that changes are not statistically significant
    assert!( !regression_report.is_statistically_significant( "fast_operation" ) );
    assert!( regression_report.get_trend_for( "fast_operation" ) == Some( PerformanceTrend::Stable ) );
  }

  #[ test ]
  fn test_regression_report_markdown_output()
  {
    let results = create_sample_results();
    
    let mut baseline_data = HashMap::new();
    let baseline_times = vec![ Duration::from_micros( 150 ), Duration::from_micros( 152 ), Duration::from_micros( 148 ) ];
    baseline_data.insert( "fast_operation".to_string(), BenchmarkResult::new( "fast_operation", baseline_times ) );
    
    let historical = HistoricalResults::new()
      .with_baseline( baseline_data );
    
    let analyzer = RegressionAnalyzer::new();
    let regression_report = analyzer.analyze( &results, &historical );
    
    let markdown = regression_report.format_markdown();
    
    // Should include proper markdown sections
    assert!( markdown.contains( "### Performance Comparison Against Baseline" ) );
    assert!( markdown.contains( "### Analysis Summary & Recommendations" ) );
    assert!( markdown.contains( "Performance improvement detected" ) );
    assert!( markdown.contains( "faster than baseline" ) );
  }
}