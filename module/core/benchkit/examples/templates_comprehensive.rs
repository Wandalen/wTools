//! Comprehensive Documentation Template Examples
//!
//! This example demonstrates EVERY use case of the Template System:
//! - Performance Report templates with all customization options
//! - Comparison Report templates for A/B testing scenarios
//! - Custom sections and content generation
//! - Template composition and advanced formatting
//! - Integration with validation and statistical analysis
//! - Error handling and template validation

#![ cfg( feature = "enabled" ) ]
#![ cfg( feature = "markdown_reports" ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::format_push_string ) ]
#![ allow( clippy::cast_lossless ) ]
#![ allow( clippy::cast_possible_truncation ) ]
#![ allow( clippy::cast_precision_loss ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::needless_raw_string_hashes ) ]

use benchkit::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

/// Create diverse benchmark results for template demonstrations
fn create_comprehensive_results() -> HashMap< String, BenchmarkResult >
{
  let mut results = HashMap::new();
  
  // Highly optimized algorithm - very fast and consistent
  let optimized_times = vec![
    Duration::from_nanos( 50 ), Duration::from_nanos( 52 ), Duration::from_nanos( 48 ),
    Duration::from_nanos( 51 ), Duration::from_nanos( 49 ), Duration::from_nanos( 50 ),
    Duration::from_nanos( 53 ), Duration::from_nanos( 47 ), Duration::from_nanos( 51 ),
    Duration::from_nanos( 50 ), Duration::from_nanos( 52 ), Duration::from_nanos( 49 ),
    Duration::from_nanos( 50 ), Duration::from_nanos( 48 ), Duration::from_nanos( 52 )
  ];
  results.insert( "optimized_algorithm".to_string(), BenchmarkResult::new( "optimized_algorithm", optimized_times ) );
  
  // Standard algorithm - good performance, reliable
  let standard_times = vec![
    Duration::from_micros( 100 ), Duration::from_micros( 105 ), Duration::from_micros( 95 ),
    Duration::from_micros( 102 ), Duration::from_micros( 98 ), Duration::from_micros( 100 ),
    Duration::from_micros( 107 ), Duration::from_micros( 93 ), Duration::from_micros( 101 ),
    Duration::from_micros( 99 ), Duration::from_micros( 104 ), Duration::from_micros( 96 ),
    Duration::from_micros( 100 ), Duration::from_micros( 102 ), Duration::from_micros( 98 )
  ];
  results.insert( "standard_algorithm".to_string(), BenchmarkResult::new( "standard_algorithm", standard_times ) );
  
  // Legacy algorithm - slower but stable
  let legacy_times = vec![
    Duration::from_micros( 500 ), Duration::from_micros( 510 ), Duration::from_micros( 490 ),
    Duration::from_micros( 505 ), Duration::from_micros( 495 ), Duration::from_micros( 500 ),
    Duration::from_micros( 515 ), Duration::from_micros( 485 ), Duration::from_micros( 502 ),
    Duration::from_micros( 498 ), Duration::from_micros( 508 ), Duration::from_micros( 492 )
  ];
  results.insert( "legacy_algorithm".to_string(), BenchmarkResult::new( "legacy_algorithm", legacy_times ) );
  
  // Experimental algorithm - fast but highly variable
  let experimental_times = vec![
    Duration::from_micros( 80 ), Duration::from_micros( 120 ), Duration::from_micros( 60 ),
    Duration::from_micros( 90 ), Duration::from_micros( 150 ), Duration::from_micros( 70 ),
    Duration::from_micros( 110 ), Duration::from_micros( 85 ), Duration::from_micros( 130 )
  ];
  results.insert( "experimental_algorithm".to_string(), BenchmarkResult::new( "experimental_algorithm", experimental_times ) );
  
  // Memory-intensive algorithm - consistently slow
  let memory_intensive_times = vec![
    Duration::from_millis( 2 ), Duration::from_millis( 2 ) + Duration::from_micros( 100 ),
    Duration::from_millis( 2 ) - Duration::from_micros( 50 ), Duration::from_millis( 2 ) + Duration::from_micros( 80 ),
    Duration::from_millis( 2 ) - Duration::from_micros( 30 ), Duration::from_millis( 2 ) + Duration::from_micros( 120 ),
    Duration::from_millis( 2 ) - Duration::from_micros( 70 ), Duration::from_millis( 2 ) + Duration::from_micros( 90 ),
    Duration::from_millis( 2 ), Duration::from_millis( 2 ) + Duration::from_micros( 60 )
  ];
  results.insert( "memory_intensive_algorithm".to_string(), BenchmarkResult::new( "memory_intensive_algorithm", memory_intensive_times ) );
  
  results
}

/// Example 1: Basic Performance Report Template
fn example_basic_performance_report()
{
  println!( "=== Example 1: Basic Performance Report Template ===" );
  
  let results = create_comprehensive_results();
  
  // Minimal performance report
  let basic_template = PerformanceReport::new();
  let basic_report = basic_template.generate( &results ).unwrap();
  
  println!( "Basic report generated: {} characters", basic_report.len() );
  println!( "Contains default title: {}", basic_report.contains( "# Performance Analysis" ) );
  println!( "Contains executive summary: {}", basic_report.contains( "## Executive Summary" ) );
  println!( "Contains statistical analysis: {}", basic_report.contains( "## Statistical Analysis" ) );
  println!( "Does NOT contain regression: {}", !basic_report.contains( "## Regression Analysis" ) );
  
  // Write to temporary file for inspection
  let temp_file = std::env::temp_dir().join( "basic_performance_report.md" );
  std::fs::write( &temp_file, &basic_report ).unwrap();
  println!( "Report saved to: {}", temp_file.display() );
  
  println!();
}

/// Example 2: Fully Customized Performance Report
fn example_customized_performance_report()
{
  println!( "=== Example 2: Fully Customized Performance Report ===" );
  
  let results = create_comprehensive_results();
  
  // Fully customized performance report
  let custom_template = PerformanceReport::new()
    .title( "Advanced Algorithm Performance Analysis" )
    .add_context( "Comprehensive comparison of 5 different algorithmic approaches for data processing" )
    .include_statistical_analysis( true )
    .include_regression_analysis( true )
    .add_custom_section( CustomSection::new(
      "Implementation Details",
      r#"### Algorithm Implementations

- **Optimized**: Hand-tuned assembly optimizations with SIMD instructions
- **Standard**: Idiomatic Rust implementation following best practices  
- **Legacy**: Original implementation maintained for compatibility
- **Experimental**: Research prototype with novel approach (⚠️ unstable)
- **Memory-Intensive**: Optimized for memory bandwidth over compute speed

### Hardware Configuration

- CPU: AMD Ryzen 9 5950X (16 cores @ 3.4GHz)
- RAM: 64GB DDR4-3600 CL16
- Storage: NVMe SSD (Samsung 980 PRO)
- OS: Ubuntu 22.04 LTS with performance governor"#
    ))
    .add_custom_section( CustomSection::new(
      "Optimization Recommendations",
      r#"### Priority Optimizations

1. **Replace Legacy Algorithm**: 5x performance improvement available
2. **Stabilize Experimental**: High potential but needs reliability work
3. **Memory-Intensive Tuning**: Consider NUMA-aware allocation
4. **SIMD Expansion**: Apply optimized approach to more operations

### Performance Targets

- Target latency: < 100μs (currently: 100.5μs average)
- Target throughput: > 10,000 ops/sec (currently: 9,950 ops/sec)
- Reliability threshold: CV < 10% (currently: 8.2%)"#
    ));
  
  let custom_report = custom_template.generate( &results ).unwrap();
  
  let report_len = custom_report.len();
  println!( "Customized report generated: {report_len} characters" );
  println!( "Contains custom title: {}", custom_report.contains( "Advanced Algorithm Performance Analysis" ) );
  println!( "Contains context: {}", custom_report.contains( "Comprehensive comparison of 5 different" ) );
  println!( "Contains implementation details: {}", custom_report.contains( "Implementation Details" ) );
  println!( "Contains optimization recommendations: {}", custom_report.contains( "Optimization Recommendations" ) );
  println!( "Contains regression analysis: {}", custom_report.contains( "## Regression Analysis" ) );
  
  // Save customized report
  let temp_file = std::env::temp_dir().join( "customized_performance_report.md" );
  std::fs::write( &temp_file, &custom_report ).unwrap();
  println!( "Customized report saved to: {}", temp_file.display() );
  
  println!();
}

/// Example 3: Basic Comparison Report Template
fn example_basic_comparison_report()
{
  println!( "=== Example 3: Basic Comparison Report Template ===" );
  
  let results = create_comprehensive_results();
  
  // Basic A/B comparison
  let basic_comparison = ComparisonReport::new()
    .baseline( "standard_algorithm" )
    .candidate( "optimized_algorithm" );
  
  let comparison_report = basic_comparison.generate( &results ).unwrap();
  
  println!( "Basic comparison report generated: {} characters", comparison_report.len() );
  println!( "Contains comparison summary: {}", comparison_report.contains( "## Comparison Summary" ) );
  println!( "Contains performance improvement: {}", comparison_report.contains( "faster" ) );
  println!( "Contains detailed comparison: {}", comparison_report.contains( "## Detailed Comparison" ) );
  println!( "Contains statistical analysis: {}", comparison_report.contains( "## Statistical Analysis" ) );
  println!( "Contains reliability assessment: {}", comparison_report.contains( "## Reliability Assessment" ) );
  
  // Check if it correctly identifies the performance improvement
  let improvement_detected = comparison_report.contains( "✅" ) && comparison_report.contains( "faster" );
  println!( "Correctly detected improvement: {}", improvement_detected );
  
  let temp_file = std::env::temp_dir().join( "basic_comparison_report.md" );
  std::fs::write( &temp_file, &comparison_report ).unwrap();
  println!( "Basic comparison saved to: {}", temp_file.display() );
  
  println!();
}

/// Example 4: Advanced Comparison Report with Custom Thresholds
fn example_advanced_comparison_report()
{
  println!( "=== Example 4: Advanced Comparison Report with Custom Thresholds ===" );
  
  let results = create_comprehensive_results();
  
  // Advanced comparison with custom thresholds
  let advanced_comparison = ComparisonReport::new()
    .title( "Legacy vs Optimized Algorithm Migration Analysis" )
    .baseline( "legacy_algorithm" )
    .candidate( "optimized_algorithm" )
    .significance_threshold( 0.01 )  // Very strict statistical requirement
    .practical_significance_threshold( 0.05 );  // 5% minimum improvement needed
  
  let advanced_report = advanced_comparison.generate( &results ).unwrap();
  
  println!( "Advanced comparison report generated: {} characters", advanced_report.len() );
  println!( "Contains custom title: {}", advanced_report.contains( "Legacy vs Optimized Algorithm Migration Analysis" ) );
  
  // Check significance thresholds
  let has_strict_threshold = advanced_report.contains( "0.01" ) || advanced_report.contains( "1%" );
  let has_practical_threshold = advanced_report.contains( "5.0%" ) || advanced_report.contains( "5%" );
  println!( "Shows strict statistical threshold: {}", has_strict_threshold );
  println!( "Shows practical significance threshold: {}", has_practical_threshold );
  
  // Should show massive improvement (legacy vs optimized)
  let shows_improvement = advanced_report.contains( "faster" );
  println!( "Correctly shows improvement: {}", shows_improvement );
  
  let temp_file = std::env::temp_dir().join( "advanced_comparison_report.md" );
  std::fs::write( &temp_file, &advanced_report ).unwrap();
  println!( "Advanced comparison saved to: {}", temp_file.display() );
  
  println!();
}

/// Example 5: Multiple Comparison Reports
fn example_multiple_comparisons()
{
  println!( "=== Example 5: Multiple Comparison Reports ===" );
  
  let results = create_comprehensive_results();
  
  // Create multiple comparison scenarios
  let comparisons = vec![
    ( "Standard vs Optimized", "standard_algorithm", "optimized_algorithm" ),
    ( "Legacy vs Standard", "legacy_algorithm", "standard_algorithm" ),
    ( "Experimental vs Standard", "standard_algorithm", "experimental_algorithm" ),
    ( "Memory vs Standard", "standard_algorithm", "memory_intensive_algorithm" ),
  ];
  
  let mut all_reports = Vec::new();
  
  for ( title, baseline, candidate ) in comparisons
  {
    let comparison = ComparisonReport::new()
      .title( title )
      .baseline( baseline )
      .candidate( candidate )
      .practical_significance_threshold( 0.10 );  // 10% threshold
    
    match comparison.generate( &results )
    {
      Ok( report ) =>
      {
        println!( "✅ {}: {} characters", title, report.len() );
        all_reports.push( ( title.to_string(), report ) );
      },
      Err( e ) =>
      {
        println!( "❌ {} failed: {}", title, e );
      }
    }
  }
  
  // Combine all comparison reports
  let combined_report = format!(
    "# Comprehensive Algorithm Comparison Analysis\n\n{}\n",
    all_reports.iter()
      .map( | ( title, report ) | format!( "## {}\n\n{}", title, report ) )
      .collect::< Vec< _ > >()
      .join( "\n---\n\n" )
  );
  
  let temp_file = std::env::temp_dir().join( "multiple_comparisons_report.md" );
  std::fs::write( &temp_file, &combined_report ).unwrap();
  
  println!( "Combined report: {} characters across {} comparisons", 
           combined_report.len(), all_reports.len() );
  println!( "Multiple comparisons saved to: {}", temp_file.display() );
  
  println!();
}

/// Example 6: Custom Sections and Advanced Formatting
fn example_custom_sections()
{
  println!( "=== Example 6: Custom Sections and Advanced Formatting ===" );
  
  let results = create_comprehensive_results();
  
  // Performance report with multiple custom sections
  let custom_template = PerformanceReport::new()
    .title( "Production Performance Audit" )
    .add_context( "Monthly performance review for algorithmic trading system" )
    .include_statistical_analysis( true )
    .include_regression_analysis( false )
    .add_custom_section( CustomSection::new(
      "Risk Assessment",
      r#"### Performance Risk Analysis

| Algorithm | Latency Risk | Throughput Risk | Stability Risk | Overall Risk |
|-----------|--------------|-----------------|----------------|--------------|
| Optimized | 🟢 Low | 🟢 Low | 🟢 Low | 🟢 **Low** |
| Standard | 🟡 Medium | 🟡 Medium | 🟢 Low | 🟡 **Medium** |
| Legacy | 🔴 High | 🔴 High | 🟡 Medium | 🔴 **High** |
| Experimental | 🔴 High | 🟡 Medium | 🔴 High | 🔴 **Critical** |
| Memory-Intensive | 🔴 High | 🔴 High | 🟢 Low | 🔴 **High** |

**Recommendations:**
- ⚠️ **Immediate**: Phase out experimental algorithm in production
- 🔄 **Q1 2024**: Migrate legacy systems to standard algorithm  
- 🚀 **Q2 2024**: Deploy optimized algorithm for critical paths"#
    ))
    .add_custom_section( CustomSection::new(
      "Business Impact",
      r#"### Performance Impact on Business Metrics

**Latency Improvements:**
- Customer satisfaction: +12% (sub-100μs response times)
- API SLA compliance: 99.9% → 99.99% uptime
- Revenue impact: ~$2.3M annually from improved user experience

**Throughput Gains:**
- Peak capacity: 8,500 → 12,000 requests/second
- Infrastructure savings: -30% server instances needed
- Cost reduction: ~$400K annually in cloud compute costs

**Risk Mitigation:**
- Reduced tail latency incidents: 95% → 5% of deployment cycles
- Improved system predictability enables better capacity planning
- Enhanced monitoring and alerting from statistical reliability metrics"#
    ))
    .add_custom_section( CustomSection::new(
      "Technical Debt Assessment",
      r#"### Code Quality and Maintenance Impact

**Current Technical Debt:**
- Legacy algorithm: 2,500 lines of unmaintained code
- Experimental algorithm: 15 open security vulnerabilities
- Memory-intensive: Poor test coverage (34% line coverage)

**Optimization Benefits:**
- Optimized algorithm: 98% test coverage, zero security issues
- Standard algorithm: Well-documented, idiomatic Rust code
- Reduced maintenance burden: -60% time spent on performance bugs

**Migration Effort Estimate:**
- Legacy replacement: 40 developer-days
- Experimental deprecation: 15 developer-days  
- Documentation updates: 10 developer-days
- **Total effort**: ~13 weeks for 1 developer"#
    ));
  
  let comprehensive_report = custom_template.generate( &results ).unwrap();
  
  println!( "Comprehensive report with custom sections: {} characters", comprehensive_report.len() );
  println!( "Contains risk assessment: {}", comprehensive_report.contains( "Risk Assessment" ) );
  println!( "Contains business impact: {}", comprehensive_report.contains( "Business Impact" ) );
  println!( "Contains technical debt: {}", comprehensive_report.contains( "Technical Debt Assessment" ) );
  println!( "Contains markdown tables: {}", comprehensive_report.contains( "| Algorithm |" ) );
  println!( "Contains emoji indicators: {}", comprehensive_report.contains( "🟢" ) );
  
  let temp_file = std::env::temp_dir().join( "comprehensive_custom_report.md" );
  std::fs::write( &temp_file, &comprehensive_report ).unwrap();
  println!( "Comprehensive report saved to: {}", temp_file.display() );
  
  println!();
}

/// Example 7: Error Handling and Edge Cases
fn example_error_handling()
{
  println!( "=== Example 7: Error Handling and Edge Cases ===" );
  
  let results = create_comprehensive_results();
  
  // Test with empty results
  println!( "Testing with empty results..." );
  let empty_results = HashMap::new();
  let empty_template = PerformanceReport::new().title( "Empty Results Test" );
  
  match empty_template.generate( &empty_results )
  {
    Ok( report ) =>
    {
      println!( "✅ Empty results handled: {} characters", report.len() );
      println!( "   Contains 'No benchmark results': {}", report.contains( "No benchmark results available" ) );
    },
    Err( e ) => println!( "❌ Empty results failed: {}", e ),
  }
  
  // Test comparison with missing baseline
  println!( "\nTesting comparison with missing baseline..." );
  let missing_baseline = ComparisonReport::new()
    .baseline( "nonexistent_algorithm" )
    .candidate( "standard_algorithm" );
  
  match missing_baseline.generate( &results )
  {
    Ok( _report ) => println!( "❌ Should have failed with missing baseline" ),
    Err( e ) =>
    {
      println!( "✅ Correctly caught missing baseline: {}", e );
      println!( "   Error mentions baseline name: {}", e.to_string().contains( "nonexistent_algorithm" ) );
    }
  }
  
  // Test comparison with missing candidate
  println!( "\nTesting comparison with missing candidate..." );
  let missing_candidate = ComparisonReport::new()
    .baseline( "standard_algorithm" )
    .candidate( "nonexistent_algorithm" );
  
  match missing_candidate.generate( &results )
  {
    Ok( _report ) => println!( "❌ Should have failed with missing candidate" ),
    Err( e ) =>
    {
      println!( "✅ Correctly caught missing candidate: {}", e );
      println!( "   Error mentions candidate name: {}", e.to_string().contains( "nonexistent_algorithm" ) );
    }
  }
  
  // Test with single result (edge case for statistics)
  println!( "\nTesting with single benchmark result..." );
  let mut single_result = HashMap::new();
  single_result.insert( "lonely_algorithm".to_string(), 
                       BenchmarkResult::new( "lonely_algorithm", vec![ Duration::from_micros( 100 ) ] ) );
  
  let single_template = PerformanceReport::new().title( "Single Result Test" );
  match single_template.generate( &single_result )
  {
    Ok( report ) =>
    {
      println!( "✅ Single result handled: {} characters", report.len() );
      println!( "   Contains algorithm name: {}", report.contains( "lonely_algorithm" ) );
      println!( "   Handles statistics gracefully: {}", report.contains( "## Statistical Analysis" ) );
    },
    Err( e ) => println!( "❌ Single result failed: {}", e ),
  }
  
  println!();
}

/// Example 8: Template Integration with Validation
fn example_template_validation_integration()
{
  println!( "=== Example 8: Template Integration with Validation ===" );
  
  let results = create_comprehensive_results();
  
  // Create validator with specific criteria
  let validator = BenchmarkValidator::new()
    .min_samples( 10 )
    .max_coefficient_variation( 0.15 )
    .require_warmup( false )
    .max_time_ratio( 2.0 );
  
  let validated_results = ValidatedResults::new( results.clone(), validator );
  
  // Create performance report that incorporates validation insights
  let integrated_template = PerformanceReport::new()
    .title( "Validated Performance Analysis" )
    .add_context( format!( 
      "Analysis of {} algorithms with {:.1}% reliability rate", 
      validated_results.results.len(),
      validated_results.reliability_rate()
    ))
    .include_statistical_analysis( true )
    .add_custom_section( CustomSection::new(
      "Reliability Assessment",
      {
        let reliable_count = validated_results.reliable_count();
        let total_count = validated_results.results.len();
        let reliability_rate = validated_results.reliability_rate();
        
        let mut assessment = format!(
          "### Statistical Reliability Summary\n\n- **Reliable algorithms**: {}/{} ({:.1}%)\n",
          reliable_count, total_count, reliability_rate
        );
        
        if let Some( warnings ) = validated_results.reliability_warnings()
        {
          assessment.push_str( "\n### Quality Concerns\n\n" );
          for warning in warnings
          {
            assessment.push_str( &format!( "- {}\n", warning ) );
          }
        }
        
        if reliable_count > 0
        {
          assessment.push_str( "\n### Recommended Algorithms\n\n" );
          let reliable_results = validated_results.reliable_results();
          for ( name, result ) in reliable_results
          {
            assessment.push_str( &format!(
              "- **{}**: {:.2?} mean time, {:.1}% CV, {} samples\n",
              name, 
              result.mean_time(),
              result.coefficient_of_variation() * 100.0,
              result.times.len()
            ));
          }
        }
        
        assessment
      }
    ));
  
  let integrated_report = integrated_template.generate( &results ).unwrap();
  
  println!( "Validation-integrated report: {} characters", integrated_report.len() );
  println!( "Contains reliability rate: {}", integrated_report.contains( &format!( "{:.1}%", validated_results.reliability_rate() ) ) );
  println!( "Contains quality concerns: {}", integrated_report.contains( "Quality Concerns" ) );
  println!( "Contains recommended algorithms: {}", integrated_report.contains( "Recommended Algorithms" ) );
  
  // Also create a comparison using only reliable results
  let reliable_results = validated_results.reliable_results();
  if reliable_results.len() >= 2
  {
    let reliable_names : Vec< &String > = reliable_results.keys().collect();
    let validated_comparison = ComparisonReport::new()
      .title( "Validated Algorithm Comparison" )
      .baseline( reliable_names[ 0 ] )
      .candidate( reliable_names[ 1 ] );
    
    match validated_comparison.generate( &reliable_results )
    {
      Ok( comparison_report ) =>
      {
        println!( "✅ Validated comparison report: {} characters", comparison_report.len() );
        
        let combined_report = format!(
          "{}\n\n---\n\n{}", 
          integrated_report, 
          comparison_report
        );
        
        let temp_file = std::env::temp_dir().join( "validated_integrated_report.md" );
        std::fs::write( &temp_file, &combined_report ).unwrap();
        println!( "Integrated validation report saved to: {}", temp_file.display() );
      },
      Err( e ) => println!( "❌ Validated comparison failed: {}", e ),
    }
  }
  else
  {
    println!( "⚠️ Not enough reliable results for comparison (need ≥2, have {})", reliable_results.len() );
    
    let temp_file = std::env::temp_dir().join( "validation_only_report.md" );
    std::fs::write( &temp_file, &integrated_report ).unwrap();
    println!( "Validation report saved to: {}", temp_file.display() );
  }
  
  println!();
}

fn main()
{
  println!( "🚀 Comprehensive Documentation Template Examples\n" );
  
  example_basic_performance_report();
  example_customized_performance_report();
  example_basic_comparison_report();
  example_advanced_comparison_report();
  example_multiple_comparisons();
  example_custom_sections();
  example_error_handling();
  example_template_validation_integration();
  
  println!( "📋 Template System Use Cases Covered:" );
  println!( "✅ Basic and customized Performance Report templates" );
  println!( "✅ Basic and advanced Comparison Report templates" );
  println!( "✅ Multiple comparison scenarios and batch processing" );
  println!( "✅ Custom sections with advanced markdown formatting" );
  println!( "✅ Comprehensive error handling for edge cases" );
  println!( "✅ Full integration with validation framework" );
  println!( "✅ Business impact analysis and risk assessment" );
  println!( "✅ Technical debt assessment and migration planning" );
  println!( "\n🎯 The Template System provides professional, customizable reports" );
  println!( "   with statistical rigor and business-focused insights." );
  
  println!( "\n📁 Generated reports saved to temporary directory:" );
  println!( "   {}", std::env::temp_dir().display() );
}