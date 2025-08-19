//! Tests for template system functionality

#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::float_cmp ) ]

#[ cfg( feature = "integration" ) ]
#[ cfg( feature = "markdown_reports" ) ]
mod tests
{
  use benchkit::prelude::*;
  use std::collections::HashMap;
  use std::time::Duration;

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
}