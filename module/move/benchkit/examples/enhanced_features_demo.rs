#![ allow( clippy::similar_names ) ]
#![ allow( clippy::needless_raw_string_hashes ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::if_not_else ) ]
//! Demonstration of enhanced benchkit features
//! 
//! This example showcases the new practical usage features:
//! - Safe Update Chain Pattern for atomic markdown updates
//! - Documentation templates for consistent reporting
//! - Benchmark validation for quality assessment

#![ cfg( feature = "enabled" ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::needless_borrows_for_generic_args ) ]

use benchkit::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

fn simulate_algorithm_a() -> Duration
{
  // Simulate fast, consistent algorithm
  std::thread::sleep( Duration::from_micros( 100 ) );
  Duration::from_micros( 100 )
}

fn simulate_algorithm_b() -> Duration
{
  // Simulate slower, more variable algorithm  
  let base = Duration::from_micros( 200 );
  let variance = Duration::from_micros( 50 );
  std::thread::sleep( base );
  base + variance
}

fn simulate_unreliable_algorithm() -> Duration
{
  // Simulate highly variable algorithm
  let base = Duration::from_millis( 1 );
  use std::collections::hash_map::DefaultHasher;
  use std::hash::{Hash, Hasher};
  let mut hasher = DefaultHasher::new();
  std::thread::current().id().hash(&mut hasher);
  let variance_micros = hasher.finish() % 500;
  std::thread::sleep( base );
  base + Duration::from_micros( variance_micros )
}

fn create_benchmark_results() -> HashMap< String, BenchmarkResult >
{
  let mut results = HashMap::new();

  // Create reliable benchmark result
  let algorithm_a_times : Vec< Duration > = ( 0..15 )
    .map( | _ | simulate_algorithm_a() )
    .collect();
  results.insert( "algorithm_a".to_string(), BenchmarkResult::new( "algorithm_a", algorithm_a_times ) );

  // Create moderately reliable result
  let algorithm_b_times : Vec< Duration > = ( 0..12 )
    .map( | _ | simulate_algorithm_b() )
    .collect();
  results.insert( "algorithm_b".to_string(), BenchmarkResult::new( "algorithm_b", algorithm_b_times ) );

  // Create unreliable result (for validation demonstration)
  let unreliable_times : Vec< Duration > = ( 0..6 )
    .map( | _ | simulate_unreliable_algorithm() )
    .collect();
  results.insert( "unreliable_algorithm".to_string(), BenchmarkResult::new( "unreliable_algorithm", unreliable_times ) );

  results
}

fn demonstrate_validation_framework()
{
  println!( "=== Benchmark Validation Framework Demo ===" );
  
  let results = create_benchmark_results();
  
  // Create validator with custom criteria
  let validator = BenchmarkValidator::new()
    .min_samples( 10 )
    .max_coefficient_variation( 0.15 )
    .require_warmup( false ) // Disabled for demo
    .max_time_ratio( 3.0 )
    .min_measurement_time( Duration::from_micros( 50 ) );

  // Validate all results
  let validated_results = ValidatedResults::new( results, validator );
  
  println!( "Total benchmarks: {}", validated_results.results.len() );
  println!( "Reliable benchmarks: {}", validated_results.reliable_count() );
  println!( "Reliability rate: {:.1}%", validated_results.reliability_rate() );
  
  // Show warnings if any
  if let Some( warnings ) = validated_results.reliability_warnings()
  {
    println!( "\n⚠️ Quality concerns detected:" );
    for warning in warnings
    {
      println!( "  - {}", warning );
    }
  }
  else
  {
    println!( "\n✅ All benchmarks meet quality criteria!" );
  }
  
  println!( "\n" );
}

fn demonstrate_template_system()
{
  println!( "=== Template System Demo ===" );
  
  let results = create_benchmark_results();
  
  // Performance report template
  let performance_template = PerformanceReport::new()
    .title( "Algorithm Performance Analysis" )
    .add_context( "Comparing three different algorithmic approaches" )
    .include_statistical_analysis( true )
    .include_regression_analysis( false )
    .add_custom_section( CustomSection::new( 
      "Implementation Notes", 
      "- Algorithm A: Optimized for consistency\n- Algorithm B: Balanced approach\n- Unreliable: Experimental implementation" 
    ) );
  
  let performance_report = performance_template.generate( &results ).unwrap();
  println!( "Performance Report Generated ({} characters)", performance_report.len() );
  
  // Comparison report template
  let comparison_template = ComparisonReport::new()
    .title( "Algorithm A vs Algorithm B Comparison" )
    .baseline( "algorithm_b" )
    .candidate( "algorithm_a" )
    .significance_threshold( 0.05 )
    .practical_significance_threshold( 0.10 );
  
  let comparison_report = comparison_template.generate( &results ).unwrap();
  println!( "Comparison Report Generated ({} characters)", comparison_report.len() );
  
  println!( "\n" );
}

fn demonstrate_update_chain()
{
  println!( "=== Update Chain Demo ===" );
  
  let results = create_benchmark_results();
  
  // Create temporary file for demonstration
  let temp_file = std::env::temp_dir().join( "benchkit_demo.md" );
  
  // Initial content
  let initial_content = r#"# Benchkit Enhanced Features Demo

## Introduction

This document demonstrates the new enhanced features of benchkit.

## Conclusion

More sections will be added automatically."#;
  
  std::fs::write( &temp_file, initial_content ).unwrap();
  
  // Generate reports using templates
  let performance_template = PerformanceReport::new()
    .title( "Performance Analysis Results" )
    .include_statistical_analysis( true );
  let performance_content = performance_template.generate( &results ).unwrap();
  
  let comparison_template = ComparisonReport::new()
    .baseline( "algorithm_b" )
    .candidate( "algorithm_a" );
  let comparison_content = comparison_template.generate( &results ).unwrap();
  
  let validator = BenchmarkValidator::new().require_warmup( false );
  let validation_report = validator.generate_validation_report( &results );
  
  // Use update chain for atomic updates
  let chain = MarkdownUpdateChain::new( &temp_file ).unwrap()
    .add_section( "Performance Analysis", &performance_content )
    .add_section( "Algorithm Comparison", &comparison_content )
    .add_section( "Quality Assessment", &validation_report );
  
  // Check for conflicts
  let conflicts = chain.check_all_conflicts().unwrap();
  if !conflicts.is_empty()
  {
    println!( "⚠️ Potential conflicts detected: {:?}", conflicts );
  }
  else
  {
    println!( "✅ No conflicts detected" );
  }
  
  // Execute atomic update
  match chain.execute()
  {
    Ok( () ) =>
    {
      println!( "✅ Successfully updated {} sections atomically", chain.len() );
      
      let final_content = std::fs::read_to_string( &temp_file ).unwrap();
      println!( "Final document size: {} characters", final_content.len() );
      
      // Count sections
      let section_count = final_content.matches( "## " ).count();
      println!( "Total sections in document: {}", section_count );
    },
    Err( e ) =>
    {
      println!( "❌ Update failed: {}", e );
    }
  }
  
  // Cleanup
  let _ = std::fs::remove_file( &temp_file );
  
  println!( "\n" );
}

fn demonstrate_practical_workflow()
{
  println!( "=== Practical Workflow Demo ===" );
  
  // Step 1: Run benchmarks and collect results
  println!( "1. Running benchmarks..." );
  let results = create_benchmark_results();
  
  // Step 2: Validate results for quality
  println!( "2. Validating benchmark quality..." );
  let validator = BenchmarkValidator::new().require_warmup( false );
  let validated_results = ValidatedResults::new( results.clone(), validator );
  
  if validated_results.reliability_rate() < 50.0
  {
    println!( "   ⚠️ Low reliability rate: {:.1}%", validated_results.reliability_rate() );
    println!( "   Consider increasing sample sizes or reducing measurement noise" );
  }
  else
  {
    println!( "   ✅ Good reliability rate: {:.1}%", validated_results.reliability_rate() );
  }
  
  // Step 3: Generate professional reports
  println!( "3. Generating reports..." );
  let template = PerformanceReport::new()
    .title( "Production Performance Analysis" )
    .add_context( "Automated benchmark analysis with quality validation" )
    .include_statistical_analysis( true );
  
  let report = template.generate( &results ).unwrap();
  println!( "   📄 Generated {} character report", report.len() );
  
  // Step 4: Update documentation atomically
  println!( "4. Updating documentation..." );
  let temp_doc = std::env::temp_dir().join( "production_report.md" );
  
  let chain = MarkdownUpdateChain::new( &temp_doc ).unwrap()
    .add_section( "Latest Performance Results", &report )
    .add_section( "Quality Assessment", &validated_results.validation_report() );
  
  match chain.execute()
  {
    Ok( () ) => println!( "   ✅ Documentation updated successfully" ),
    Err( e ) => println!( "   ❌ Documentation update failed: {}", e ),
  }
  
  // Cleanup
  let _ = std::fs::remove_file( &temp_doc );
  
  println!( "\n✅ Practical workflow demonstration complete!" );
}

fn main()
{
  println!( "🚀 Benchkit Enhanced Features Demonstration\n" );
  
  demonstrate_validation_framework();
  demonstrate_template_system();
  demonstrate_update_chain();
  demonstrate_practical_workflow();
  
  println!( "📋 Summary of New Features:" );
  println!( "• Safe Update Chain Pattern - Atomic markdown section updates" );
  println!( "• Documentation Templates - Consistent, professional reporting" );
  println!( "• Benchmark Validation - Quality assessment and recommendations" );
  println!( "• Integrated Workflow - Seamless validation → templating → documentation" );
}