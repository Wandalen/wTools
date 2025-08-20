#![ allow( clippy::needless_raw_string_hashes ) ]
//! Comprehensive Benchmark Validation Examples
//!
//! This example demonstrates EVERY use case of the Validation Framework:
//! - Validator configuration with all criteria options
//! - Individual result validation with detailed warnings
//! - Bulk validation of multiple results
//! - Validation report generation and interpretation
//! - Integration with templates and update chains
//! - Custom validation criteria and thresholds
//! - Performance impact analysis and recommendations

#![ cfg( feature = "enabled" ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::format_push_string ) ]
#![ allow( clippy::cast_lossless ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::if_not_else ) ]

use benchkit::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

/// Create benchmark results with various quality characteristics
fn create_diverse_quality_results() -> HashMap< String, BenchmarkResult >
{
  let mut results = HashMap::new();
  
  // Perfect quality - many samples, low variability
  let perfect_times = vec![
    Duration::from_micros( 100 ), Duration::from_micros( 102 ), Duration::from_micros( 98 ),
    Duration::from_micros( 101 ), Duration::from_micros( 99 ), Duration::from_micros( 100 ),
    Duration::from_micros( 103 ), Duration::from_micros( 97 ), Duration::from_micros( 101 ),
    Duration::from_micros( 100 ), Duration::from_micros( 102 ), Duration::from_micros( 99 ),
    Duration::from_micros( 100 ), Duration::from_micros( 98 ), Duration::from_micros( 102 ),
    Duration::from_micros( 101 ), Duration::from_micros( 99 ), Duration::from_micros( 100 )
  ];
  results.insert( "perfect_quality".to_string(), BenchmarkResult::new( "perfect_quality", perfect_times ) );
  
  // Good quality - adequate samples, reasonable variability
  let good_times = vec![
    Duration::from_micros( 200 ), Duration::from_micros( 210 ), Duration::from_micros( 190 ),
    Duration::from_micros( 205 ), Duration::from_micros( 195 ), Duration::from_micros( 200 ),
    Duration::from_micros( 215 ), Duration::from_micros( 185 ), Duration::from_micros( 202 ),
    Duration::from_micros( 198 ), Duration::from_micros( 208 ), Duration::from_micros( 192 )
  ];
  results.insert( "good_quality".to_string(), BenchmarkResult::new( "good_quality", good_times ) );
  
  // Insufficient samples
  let few_samples_times = vec![
    Duration::from_micros( 150 ), Duration::from_micros( 155 ), Duration::from_micros( 145 ),
    Duration::from_micros( 152 ), Duration::from_micros( 148 )
  ];
  results.insert( "insufficient_samples".to_string(), BenchmarkResult::new( "insufficient_samples", few_samples_times ) );
  
  // High variability
  let high_variability_times = vec![
    Duration::from_micros( 100 ), Duration::from_micros( 200 ), Duration::from_micros( 50 ),
    Duration::from_micros( 150 ), Duration::from_micros( 80 ), Duration::from_micros( 180 ),
    Duration::from_micros( 120 ), Duration::from_micros( 170 ), Duration::from_micros( 60 ),
    Duration::from_micros( 140 ), Duration::from_micros( 90 ), Duration::from_micros( 160 ),
    Duration::from_micros( 110 ), Duration::from_micros( 190 ), Duration::from_micros( 70 )
  ];
  results.insert( "high_variability".to_string(), BenchmarkResult::new( "high_variability", high_variability_times ) );
  
  // Very short measurement times (nanoseconds)
  let short_measurement_times = vec![
    Duration::from_nanos( 10 ), Duration::from_nanos( 12 ), Duration::from_nanos( 8 ),
    Duration::from_nanos( 11 ), Duration::from_nanos( 9 ), Duration::from_nanos( 10 ),
    Duration::from_nanos( 13 ), Duration::from_nanos( 7 ), Duration::from_nanos( 11 ),
    Duration::from_nanos( 10 ), Duration::from_nanos( 12 ), Duration::from_nanos( 9 ),
    Duration::from_nanos( 10 ), Duration::from_nanos( 8 ), Duration::from_nanos( 12 )
  ];
  results.insert( "short_measurements".to_string(), BenchmarkResult::new( "short_measurements", short_measurement_times ) );
  
  // Wide performance range
  let wide_range_times = vec![
    Duration::from_micros( 50 ), Duration::from_micros( 55 ), Duration::from_micros( 250 ),
    Duration::from_micros( 60 ), Duration::from_micros( 200 ), Duration::from_micros( 52 ),
    Duration::from_micros( 180 ), Duration::from_micros( 58 ), Duration::from_micros( 220 ),
    Duration::from_micros( 65 ), Duration::from_micros( 240 ), Duration::from_micros( 48 )
  ];
  results.insert( "wide_range".to_string(), BenchmarkResult::new( "wide_range", wide_range_times ) );
  
  // No obvious warmup pattern (all measurements similar)
  let no_warmup_times = vec![
    Duration::from_micros( 300 ), Duration::from_micros( 302 ), Duration::from_micros( 298 ),
    Duration::from_micros( 301 ), Duration::from_micros( 299 ), Duration::from_micros( 300 ),
    Duration::from_micros( 303 ), Duration::from_micros( 297 ), Duration::from_micros( 301 ),
    Duration::from_micros( 300 ), Duration::from_micros( 302 ), Duration::from_micros( 298 )
  ];
  results.insert( "no_warmup".to_string(), BenchmarkResult::new( "no_warmup", no_warmup_times ) );
  
  results
}

/// Example 1: Default Validator Configuration
fn example_default_validator()
{
  println!( "=== Example 1: Default Validator Configuration ===" );
  
  let results = create_diverse_quality_results();
  let validator = BenchmarkValidator::new();
  
  println!( "Default validator criteria:" );
  println!( "- Minimum samples: 10 (default)" );
  println!( "- Maximum CV: 10% (default)" );
  println!( "- Requires warmup: true (default)" );
  println!( "- Maximum time ratio: 3.0x (default)" );
  println!( "- Minimum measurement time: 1μs (default)" );
  
  // Validate each result individually
  for ( name, result ) in &results
  {
    let warnings = validator.validate_result( result );
    let is_reliable = validator.is_reliable( result );
    
    println!( "\n📊 {}: {} warnings, reliable: {}", 
             name, warnings.len(), is_reliable );
    
    for warning in warnings
    {
      println!( "   ⚠️ {}", warning );
    }
  }
  
  // Overall statistics
  let reliable_count = results.values()
    .filter( | result | validator.is_reliable( result ) )
    .count();
  
  println!( "\n📈 Overall validation summary:" );
  println!( "   Total benchmarks: {}", results.len() );
  println!( "   Reliable benchmarks: {}", reliable_count );
  println!( "   Reliability rate: {:.1}%", 
           ( reliable_count as f64 / results.len() as f64 ) * 100.0 );
  
  println!();
}

/// Example 2: Custom Validator Configuration
fn example_custom_validator()
{
  println!( "=== Example 2: Custom Validator Configuration ===" );
  
  let results = create_diverse_quality_results();
  
  // Strict validator for production use
  let strict_validator = BenchmarkValidator::new()
    .min_samples( 20 )
    .max_coefficient_variation( 0.05 )  // 5% maximum CV
    .require_warmup( true )
    .max_time_ratio( 2.0 )  // Tighter range requirement
    .min_measurement_time( Duration::from_micros( 10 ) );  // Longer minimum time
  
  println!( "Strict validator criteria:" );
  println!( "- Minimum samples: 20" );
  println!( "- Maximum CV: 5%" );
  println!( "- Requires warmup: true" );
  println!( "- Maximum time ratio: 2.0x" );
  println!( "- Minimum measurement time: 10μs" );
  
  let strict_results = ValidatedResults::new( results.clone(), strict_validator );
  
  println!( "\n📊 Strict validation results:" );
  println!( "   Reliable benchmarks: {}/{} ({:.1}%)", 
           strict_results.reliable_count(),
           strict_results.results.len(),
           strict_results.reliability_rate() );
  
  if let Some( warnings ) = strict_results.reliability_warnings()
  {
    println!( "\n⚠️ Quality issues detected with strict criteria:" );
    for warning in warnings
    {
      println!( "   - {}", warning );
    }
  }
  
  // Lenient validator for development/debugging
  let lenient_validator = BenchmarkValidator::new()
    .min_samples( 5 )
    .max_coefficient_variation( 0.25 )  // 25% maximum CV
    .require_warmup( false )
    .max_time_ratio( 10.0 )  // Very loose range requirement
    .min_measurement_time( Duration::from_nanos( 1 ) );  // Accept any duration
  
  println!( "\nLenient validator criteria:" );
  println!( "- Minimum samples: 5" );
  println!( "- Maximum CV: 25%" );
  println!( "- Requires warmup: false" );
  println!( "- Maximum time ratio: 10.0x" );
  println!( "- Minimum measurement time: 1ns" );
  
  let lenient_results = ValidatedResults::new( results, lenient_validator );
  
  println!( "\n📊 Lenient validation results:" );
  println!( "   Reliable benchmarks: {}/{} ({:.1}%)", 
           lenient_results.reliable_count(),
           lenient_results.results.len(),
           lenient_results.reliability_rate() );
  
  if lenient_results.reliability_rate() < 100.0
  {
    println!( "   Note: Even lenient criteria found issues!" );
  }
  else
  {
    println!( "   ✅ All benchmarks pass lenient criteria" );
  }
  
  println!();
}

/// Example 3: Individual Warning Types
fn example_individual_warnings()
{
  println!( "=== Example 3: Individual Warning Types ===" );
  
  let results = create_diverse_quality_results();
  let validator = BenchmarkValidator::new();
  
  // Demonstrate each type of warning
  println!( "🔍 Analyzing specific warning types:\n" );
  
  for ( name, result ) in &results
  {
    let warnings = validator.validate_result( result );
    
    println!( "📊 {}:", name );
    println!( "   Samples: {}", result.times.len() );
    println!( "   Mean time: {:.2?}", result.mean_time() );
    println!( "   CV: {:.1}%", result.coefficient_of_variation() * 100.0 );
    
    if !warnings.is_empty()
    {
      println!( "   ⚠️ Issues:" );
      for warning in &warnings
      {
        match warning
        {
          ValidationWarning::InsufficientSamples { actual, minimum } =>
          {
            println!( "     - Insufficient samples: {} < {} required", actual, minimum );
          },
          ValidationWarning::HighVariability { actual, maximum } =>
          {
            println!( "     - High variability: {:.1}% > {:.1}% maximum", actual * 100.0, maximum * 100.0 );
          },
          ValidationWarning::NoWarmup =>
          {
            println!( "     - No warmup detected (all measurements similar)" );
          },
          ValidationWarning::WidePerformanceRange { ratio } =>
          {
            println!( "     - Wide performance range: {:.1}x difference", ratio );
          },
          ValidationWarning::ShortMeasurementTime { duration } =>
          {
            println!( "     - Short measurement time: {:.2?} may be inaccurate", duration );
          },
        }
      }
    }
    else
    {
      println!( "   ✅ No issues detected" );
    }
    
    println!();
  }
}

/// Example 4: Validation Report Generation
fn example_validation_reports()
{
  println!( "=== Example 4: Validation Report Generation ===" );
  
  let results = create_diverse_quality_results();
  let validator = BenchmarkValidator::new();
  
  // Generate comprehensive validation report
  let validation_report = validator.generate_validation_report( &results );
  
  println!( "Generated validation report: {} characters", validation_report.len() );
  println!( "Contains validation summary: {}", validation_report.contains( "## Summary" ) );
  println!( "Contains recommendations: {}", validation_report.contains( "## Recommendations" ) );
  println!( "Contains methodology: {}", validation_report.contains( "## Validation Criteria" ) );
  
  // Save validation report
  let temp_file = std::env::temp_dir().join( "validation_report.md" );
  std::fs::write( &temp_file, &validation_report ).unwrap();
  println!( "Validation report saved to: {}", temp_file.display() );
  
  // Create ValidatedResults and get its report
  let validated_results = ValidatedResults::new( results, validator );
  let validated_report = validated_results.validation_report();
  
  println!( "\nValidatedResults report: {} characters", validated_report.len() );
  println!( "Reliability rate: {:.1}%", validated_results.reliability_rate() );
  
  let temp_file2 = std::env::temp_dir().join( "validated_results_report.md" );
  std::fs::write( &temp_file2, &validated_report ).unwrap();
  println!( "ValidatedResults report saved to: {}", temp_file2.display() );
  
  println!();
}

/// Example 5: Reliable Results Filtering
fn example_reliable_results_filtering()
{
  println!( "=== Example 5: Reliable Results Filtering ===" );
  
  let results = create_diverse_quality_results();
  let validator = BenchmarkValidator::new().require_warmup( false );  // Disable warmup for demo
  
  let validated_results = ValidatedResults::new( results, validator );
  
  println!( "Original results: {} benchmarks", validated_results.results.len() );
  println!( "Reliable results: {} benchmarks", validated_results.reliable_count() );
  
  // Get only reliable results
  let reliable_only = validated_results.reliable_results();
  
  println!( "\n✅ Reliable benchmarks:" );
  for ( name, result ) in &reliable_only
  {
    println!( "   - {}: {:.2?} mean, {:.1}% CV, {} samples",
             name, 
             result.mean_time(),
             result.coefficient_of_variation() * 100.0,
             result.times.len() );
  }
  
  // Demonstrate using reliable results for further analysis
  if reliable_only.len() >= 2
  {
    println!( "\n🔍 Using only reliable results for comparison analysis..." );
    
    let reliable_names : Vec< &String > = reliable_only.keys().collect();
    let comparison_template = ComparisonReport::new()
      .title( "Reliable Algorithm Comparison" )
      .baseline( reliable_names[ 0 ] )
      .candidate( reliable_names[ 1 ] );
    
    match comparison_template.generate( &reliable_only )
    {
      Ok( comparison_report ) =>
      {
        println!( "✅ Comparison report generated: {} characters", comparison_report.len() );
        
        let temp_file = std::env::temp_dir().join( "reliable_comparison.md" );
        std::fs::write( &temp_file, &comparison_report ).unwrap();
        println!( "Reliable comparison saved to: {}", temp_file.display() );
      },
      Err( e ) => println!( "❌ Comparison failed: {}", e ),
    }
  }
  else
  {
    println!( "⚠️ Not enough reliable results for comparison (need ≥2)" );
  }
  
  println!();
}

/// Example 6: Custom Validation Criteria
fn example_custom_validation_scenarios()
{
  println!( "=== Example 6: Custom Validation Scenarios ===" );
  
  let results = create_diverse_quality_results();
  
  // Scenario 1: Research-grade validation (very strict)
  println!( "🔬 Research-grade validation (publication quality):" );
  let research_validator = BenchmarkValidator::new()
    .min_samples( 30 )
    .max_coefficient_variation( 0.02 )  // 2% maximum CV
    .require_warmup( true )
    .max_time_ratio( 1.5 )  // Very tight range
    .min_measurement_time( Duration::from_micros( 100 ) );  // Long measurements
  
  let research_results = ValidatedResults::new( results.clone(), research_validator );
  println!( "   Reliability rate: {:.1}%", research_results.reliability_rate() );
  
  // Scenario 2: Quick development validation (very lenient)
  println!( "\n⚡ Quick development validation (rapid iteration):" );
  let dev_validator = BenchmarkValidator::new()
    .min_samples( 3 )
    .max_coefficient_variation( 0.50 )  // 50% maximum CV
    .require_warmup( false )
    .max_time_ratio( 20.0 )  // Very loose range
    .min_measurement_time( Duration::from_nanos( 1 ) );
  
  let dev_results = ValidatedResults::new( results.clone(), dev_validator );
  println!( "   Reliability rate: {:.1}%", dev_results.reliability_rate() );
  
  // Scenario 3: Production monitoring validation (balanced)
  println!( "\n🏭 Production monitoring validation (CI/CD pipelines):" );
  let production_validator = BenchmarkValidator::new()
    .min_samples( 15 )
    .max_coefficient_variation( 0.10 )  // 10% maximum CV
    .require_warmup( true )
    .max_time_ratio( 2.5 )
    .min_measurement_time( Duration::from_micros( 50 ) );
  
  let production_results = ValidatedResults::new( results.clone(), production_validator );
  println!( "   Reliability rate: {:.1}%", production_results.reliability_rate() );
  
  // Scenario 4: Microbenchmark validation (for very fast operations)
  println!( "\n🔬 Microbenchmark validation (nanosecond measurements):" );
  let micro_validator = BenchmarkValidator::new()
    .min_samples( 100 )  // Many samples for statistical power
    .max_coefficient_variation( 0.15 )  // 15% CV (noise is expected)
    .require_warmup( true )  // Critical for micro operations
    .max_time_ratio( 5.0 )  // Allow more variation
    .min_measurement_time( Duration::from_nanos( 10 ) );  // Accept nano measurements
  
  let micro_results = ValidatedResults::new( results, micro_validator );
  println!( "   Reliability rate: {:.1}%", micro_results.reliability_rate() );
  
  // Summary comparison
  println!( "\n📊 Validation scenario comparison:" );
  println!( "   Research-grade: {:.1}% reliable", research_results.reliability_rate() );
  println!( "   Development:    {:.1}% reliable", dev_results.reliability_rate() );
  println!( "   Production:     {:.1}% reliable", production_results.reliability_rate() );
  println!( "   Microbenchmark: {:.1}% reliable", micro_results.reliability_rate() );
  
  println!();
}

/// Example 7: Integration with Templates and Update Chains
fn example_validation_integration()
{
  println!( "=== Example 7: Integration with Templates and Update Chains ===" );
  
  let results = create_diverse_quality_results();
  let validator = BenchmarkValidator::new();
  let validated_results = ValidatedResults::new( results, validator );
  
  // Create comprehensive analysis using validation
  let performance_template = PerformanceReport::new()
    .title( "Quality-Validated Performance Analysis" )
    .add_context( format!( 
      "Analysis includes quality validation - {:.1}% of benchmarks meet reliability criteria",
      validated_results.reliability_rate()
    ))
    .include_statistical_analysis( true )
    .add_custom_section( CustomSection::new(
      "Quality Assessment Results",
      {
        let mut assessment = String::new();
        
        assessment.push_str( &format!( 
          "### Validation Summary\n\n- **Total benchmarks**: {}\n- **Reliable benchmarks**: {}\n- **Reliability rate**: {:.1}%\n\n",
          validated_results.results.len(),
          validated_results.reliable_count(),
          validated_results.reliability_rate()
        ));
        
        if let Some( warnings ) = validated_results.reliability_warnings()
        {
          assessment.push_str( "### Quality Issues Detected\n\n" );
          for warning in warnings.iter().take( 10 )  // Limit to first 10 warnings
          {
            assessment.push_str( &format!( "- {}\n", warning ) );
          }
          
          if warnings.len() > 10
          {
            assessment.push_str( &format!( "- ... and {} more issues\n", warnings.len() - 10 ) );
          }
        }
        
        assessment
      }
    ));
  
  // Generate reports
  let full_analysis = performance_template.generate( &validated_results.results ).unwrap();
  let validation_report = validated_results.validation_report();
  
  // Create temporary document for update chain demo
  let temp_file = std::env::temp_dir().join( "validation_integration_demo.md" );
  let initial_content = r#"# Validation Integration Demo

## Introduction

This document demonstrates integration of validation with templates and update chains.

## Performance Analysis

*Performance analysis will be inserted here.*

## Quality Assessment

*Validation results will be inserted here.*

## Recommendations

*Optimization recommendations based on validation.*

## Conclusion

Results and next steps.
"#;
  
  std::fs::write( &temp_file, initial_content ).unwrap();
  
  // Use update chain to atomically update documentation
  let chain = MarkdownUpdateChain::new( &temp_file ).unwrap()
    .add_section( "Performance Analysis", &full_analysis )
    .add_section( "Quality Assessment", &validation_report );
  
  match chain.execute()
  {
    Ok( () ) =>
    {
      println!( "✅ Integrated validation documentation updated successfully" );
      
      let final_content = std::fs::read_to_string( &temp_file ).unwrap();
      println!( "   Final document size: {} characters", final_content.len() );
      println!( "   Contains reliability rate: {}", final_content.contains( &format!( "{:.1}%", validated_results.reliability_rate() ) ) );
      println!( "   Contains validation summary: {}", final_content.contains( "Validation Summary" ) );
      
      println!( "   Integrated document saved to: {}", temp_file.display() );
    },
    Err( e ) => println!( "❌ Integration update failed: {}", e ),
  }
  
  // Cleanup
  // std::fs::remove_file( &temp_file ).unwrap();
  
  println!();
}

fn main()
{
  println!( "🚀 Comprehensive Benchmark Validation Examples\n" );
  
  example_default_validator();
  example_custom_validator();
  example_individual_warnings();
  example_validation_reports();
  example_reliable_results_filtering();
  example_custom_validation_scenarios();
  example_validation_integration();
  
  println!( "📋 Validation Framework Use Cases Covered:" );
  println!( "✅ Default and custom validator configurations" );
  println!( "✅ Individual warning types and detailed analysis" );
  println!( "✅ Validation report generation and formatting" );
  println!( "✅ Reliable results filtering and analysis" );
  println!( "✅ Custom validation scenarios (research, dev, production, micro)" );
  println!( "✅ Full integration with templates and update chains" );
  println!( "✅ Quality assessment and optimization recommendations" );
  println!( "\n🎯 The Validation Framework ensures statistical reliability" );
  println!( "   and provides actionable quality improvement recommendations." );
  
  println!( "\n📁 Generated reports saved to temporary directory:" );
  println!( "   {}", std::env::temp_dir().display() );
}