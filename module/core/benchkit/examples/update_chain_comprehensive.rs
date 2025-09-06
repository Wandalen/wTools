//! Comprehensive Update Chain Pattern Examples
//!
//! This example demonstrates EVERY use case of the Safe Update Chain Pattern:
//! - Single section updates with conflict detection
//! - Multi-section atomic updates with rollback
//! - Error handling and recovery patterns
//! - Integration with validation and templates
//! - Advanced conflict resolution strategies

#![ cfg( feature = "enabled" ) ]
#![ cfg( feature = "markdown_reports" ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::format_push_string ) ]
#![ allow( clippy::needless_borrows_for_generic_args ) ]
#![ allow( clippy::needless_raw_string_hashes ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::permissions_set_readonly_false ) ]
#![ allow( clippy::if_not_else ) ]

use benchkit::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

/// Create sample benchmark results for demonstration
fn create_sample_results() -> HashMap< String, BenchmarkResult >
{
  let mut results = HashMap::new();
  
  // Fast, reliable algorithm
  let fast_times = vec![
    Duration::from_micros( 100 ), Duration::from_micros( 102 ), Duration::from_micros( 98 ),
    Duration::from_micros( 101 ), Duration::from_micros( 99 ), Duration::from_micros( 100 ),
    Duration::from_micros( 103 ), Duration::from_micros( 97 ), Duration::from_micros( 101 ),
    Duration::from_micros( 100 ), Duration::from_micros( 102 ), Duration::from_micros( 99 )
  ];
  results.insert( "fast_algorithm".to_string(), BenchmarkResult::new( "fast_algorithm", fast_times ) );
  
  // Medium performance algorithm
  let medium_times = vec![
    Duration::from_micros( 250 ), Duration::from_micros( 245 ), Duration::from_micros( 255 ),
    Duration::from_micros( 248 ), Duration::from_micros( 252 ), Duration::from_micros( 250 ),
    Duration::from_micros( 247 ), Duration::from_micros( 253 ), Duration::from_micros( 249 ),
    Duration::from_micros( 251 ), Duration::from_micros( 248 ), Duration::from_micros( 252 )
  ];
  results.insert( "medium_algorithm".to_string(), BenchmarkResult::new( "medium_algorithm", medium_times ) );
  
  // Slow algorithm
  let slow_times = vec![
    Duration::from_millis( 1 ), Duration::from_millis( 1 ) + Duration::from_micros( 50 ),
    Duration::from_millis( 1 ) - Duration::from_micros( 30 ), Duration::from_millis( 1 ) + Duration::from_micros( 20 ),
    Duration::from_millis( 1 ) - Duration::from_micros( 10 ), Duration::from_millis( 1 ) + Duration::from_micros( 40 ),
    Duration::from_millis( 1 ) - Duration::from_micros( 20 ), Duration::from_millis( 1 ) + Duration::from_micros( 30 ),
    Duration::from_millis( 1 ), Duration::from_millis( 1 ) - Duration::from_micros( 15 )
  ];
  results.insert( "slow_algorithm".to_string(), BenchmarkResult::new( "slow_algorithm", slow_times ) );
  
  results
}

/// Create test document with multiple sections
fn create_test_document() -> String
{
  r#"# Performance Analysis Document

## Introduction

This document contains automated performance analysis results.

## Summary

Overall performance summary will be updated automatically.

## Algorithm Performance

*This section will be automatically updated with benchmark results.*

## Memory Analysis

*Memory usage analysis will be added here.*

## Comparison Results

*Algorithm comparison results will be inserted automatically.*

## Quality Assessment

*Benchmark quality metrics and validation results.*

## Regression Analysis

*Performance trends and regression detection.*

## Recommendations

*Optimization recommendations based on analysis.*

## Methodology

Technical details about measurement methodology.

## Conclusion

Performance analysis conclusions and next steps.
"#.to_string()
}

/// Example 1: Single Section Update with Conflict Detection
fn example_single_section_update()
{
  println!( "=== Example 1: Single Section Update ===" );
  
  let temp_file = std::env::temp_dir().join( "single_update_example.md" );
  std::fs::write( &temp_file, create_test_document() ).unwrap();
  
  let results = create_sample_results();
  let performance_template = PerformanceReport::new()
    .title( "Single Algorithm Analysis" )
    .add_context( "Demonstrating single section update pattern" );
  
  let report = performance_template.generate( &results ).unwrap();
  
  // Create update chain with single section
  let chain = MarkdownUpdateChain::new( &temp_file ).unwrap()
    .add_section( "Algorithm Performance", &report );
  
  // Check for conflicts before update
  match chain.check_all_conflicts()
  {
    Ok( conflicts ) =>
    {
      if conflicts.is_empty()
      {
        println!( "✅ No conflicts detected for single section update" );
        
        // Execute the update
        match chain.execute()
        {
          Ok( () ) =>
          {
            println!( "✅ Single section updated successfully" );
            let updated_content = std::fs::read_to_string( &temp_file ).unwrap();
            let section_count = updated_content.matches( "## Algorithm Performance" ).count();
            println!( "   Section found {} time(s) in document", section_count );
          },
          Err( e ) => println!( "❌ Update failed: {}", e ),
        }
      }
      else
      {
        println!( "⚠️ Conflicts detected: {:?}", conflicts );
      }
    },
    Err( e ) => println!( "❌ Conflict check failed: {}", e ),
  }
  
  std::fs::remove_file( &temp_file ).unwrap();
  println!();
}

/// Example 2: Multi-Section Atomic Updates
fn example_multi_section_atomic()
{
  println!( "=== Example 2: Multi-Section Atomic Update ===" );
  
  let temp_file = std::env::temp_dir().join( "multi_update_example.md" );
  std::fs::write( &temp_file, create_test_document() ).unwrap();
  
  let results = create_sample_results();
  
  // Generate multiple report sections
  let performance_template = PerformanceReport::new()
    .title( "Multi-Algorithm Performance" )
    .include_statistical_analysis( true );
  let performance_report = performance_template.generate( &results ).unwrap();
  
  let comparison_template = ComparisonReport::new()
    .title( "Fast vs Medium Algorithm Comparison" )
    .baseline( "medium_algorithm" )
    .candidate( "fast_algorithm" );
  let comparison_report = comparison_template.generate( &results ).unwrap();
  
  let validator = BenchmarkValidator::new().require_warmup( false );
  let quality_report = validator.generate_validation_report( &results );
  
  // Create update chain with multiple sections
  let chain = MarkdownUpdateChain::new( &temp_file ).unwrap()
    .add_section( "Algorithm Performance", &performance_report )
    .add_section( "Comparison Results", &comparison_report )
    .add_section( "Quality Assessment", &quality_report );
  
  println!( "Preparing to update {} sections atomically", chain.len() );
  
  // Validate all sections before update
  match chain.check_all_conflicts()
  {
    Ok( conflicts ) =>
    {
      if conflicts.is_empty()
      {
        println!( "✅ All {} sections validated successfully", chain.len() );
        
        // Execute atomic update
        match chain.execute()
        {
          Ok( () ) =>
          {
            println!( "✅ All {} sections updated atomically", chain.len() );
            let updated_content = std::fs::read_to_string( &temp_file ).unwrap();
            println!( "   Final document size: {} characters", updated_content.len() );
            
            // Verify all sections were updated
            let algo_sections = updated_content.matches( "## Algorithm Performance" ).count();
            let comp_sections = updated_content.matches( "## Comparison Results" ).count(); 
            let qual_sections = updated_content.matches( "## Quality Assessment" ).count();
            
            println!( "   Verified sections: algo={}, comp={}, qual={}", 
                     algo_sections, comp_sections, qual_sections );
          },
          Err( e ) =>
          {
            println!( "❌ Atomic update failed: {}", e );
            println!( "   All sections rolled back automatically" );
          },
        }
      }
      else
      {
        println!( "⚠️ Cannot proceed - conflicts detected: {:?}", conflicts );
      }
    },
    Err( e ) => println!( "❌ Validation failed: {}", e ),
  }
  
  std::fs::remove_file( &temp_file ).unwrap();
  println!();
}

/// Example 3: Error Handling and Recovery
fn example_error_handling()
{
  println!( "=== Example 3: Error Handling and Recovery ===" );
  
  let temp_file = std::env::temp_dir().join( "error_handling_example.md" );
  std::fs::write( &temp_file, create_test_document() ).unwrap();
  
  let results = create_sample_results();
  let report = PerformanceReport::new().generate( &results ).unwrap();
  
  // Demonstrate handling of non-existent section
  println!( "Testing update of non-existent section..." );
  let chain = MarkdownUpdateChain::new( &temp_file ).unwrap()
    .add_section( "Non-Existent Section", &report );
  
  match chain.check_all_conflicts()
  {
    Ok( conflicts ) =>
    {
      if !conflicts.is_empty()
      {
        println!( "✅ Correctly detected missing section conflict: {:?}", conflicts );
        
        // Show how to handle the conflict
        println!( "   Recovery strategy: Create section manually or use different section name" );
        
        // Retry with correct section name
        let recovery_chain = MarkdownUpdateChain::new( &temp_file ).unwrap()
          .add_section( "Algorithm Performance", &report );
        
        match recovery_chain.execute()
        {
          Ok( () ) => println!( "✅ Recovery successful with correct section name" ),
          Err( e ) => println!( "❌ Recovery failed: {}", e ),
        }
      }
      else
      {
        println!( "❌ Conflict detection failed - this should not happen" );
      }
    },
    Err( e ) => println!( "✅ Correctly caught validation error: {}", e ),
  }
  
  // Demonstrate file permission error handling
  println!( "\nTesting file permission error handling..." );
  
  // Make file read-only to simulate permission error
  let metadata = std::fs::metadata( &temp_file ).unwrap();
  let mut permissions = metadata.permissions();
  permissions.set_readonly( true );
  std::fs::set_permissions( &temp_file, permissions ).unwrap();
  
  let readonly_chain = MarkdownUpdateChain::new( &temp_file ).unwrap()
    .add_section( "Algorithm Performance", &report );
  
  match readonly_chain.execute()
  {
    Ok( () ) => println!( "❌ Should have failed due to read-only file" ),
    Err( e ) =>
    {
      println!( "✅ Correctly handled permission error: {}", e );
      println!( "   File remains unchanged due to atomic operation" );
    },
  }
  
  // Restore permissions and cleanup
  let mut permissions = std::fs::metadata( &temp_file ).unwrap().permissions();
  permissions.set_readonly( false );
  std::fs::set_permissions( &temp_file, permissions ).unwrap();
  std::fs::remove_file( &temp_file ).unwrap();
  
  println!();
}

/// Example 4: Advanced Conflict Resolution
fn example_conflict_resolution()
{
  println!( "=== Example 4: Advanced Conflict Resolution ===" );
  
  let temp_file = std::env::temp_dir().join( "conflict_resolution_example.md" );
  
  // Create document with ambiguous section names
  let ambiguous_content = r#"# Document with Conflicts

## Performance

First performance section.

## Algorithm Performance 

Main algorithm section.

## Performance Analysis

Detailed performance analysis.

## Performance

Second performance section (duplicate).
"#;
  
  std::fs::write( &temp_file, ambiguous_content ).unwrap();
  
  let results = create_sample_results();
  let report = PerformanceReport::new().generate( &results ).unwrap();
  
  // Try to update ambiguous "Performance" section
  let chain = MarkdownUpdateChain::new( &temp_file ).unwrap()
    .add_section( "Performance", &report );
  
  match chain.check_all_conflicts()
  {
    Ok( conflicts ) =>
    {
      if !conflicts.is_empty()
      {
        println!( "✅ Detected conflicts with ambiguous section names:" );
        for conflict in &conflicts
        {
          println!( "   - {}", conflict );
        }
        
        // Resolution strategy 1: Use more specific section name
        println!( "\n Strategy 1: Using more specific section name" );
        let specific_chain = MarkdownUpdateChain::new( &temp_file ).unwrap()
          .add_section( "Algorithm Performance", &report );
        
        match specific_chain.check_all_conflicts()
        {
          Ok( specific_conflicts ) =>
          {
            if specific_conflicts.is_empty()
            {
              println!( "✅ No conflicts with specific section name" );
              match specific_chain.execute()
              {
                Ok( () ) => println!( "✅ Update successful with specific targeting" ),
                Err( e ) => println!( "❌ Update failed: {}", e ),
              }
            }
            else
            {
              println!( "⚠️ Still has conflicts: {:?}", specific_conflicts );
            }
          },
          Err( e ) => println!( "❌ Validation failed: {}", e ),
        }
      }
      else
      {
        println!( "❌ Should have detected conflicts with duplicate section names" );
      }
    },
    Err( e ) => println!( "❌ Validation failed: {}", e ),
  }
  
  std::fs::remove_file( &temp_file ).unwrap();
  println!();
}

/// Example 5: Performance and Efficiency
fn example_performance_efficiency()
{
  println!( "=== Example 5: Performance and Efficiency ===" );
  
  let temp_file = std::env::temp_dir().join( "performance_example.md" );
  
  // Create large document for performance testing
  let mut large_content = String::from( "# Large Document Performance Test\n\n" );
  for i in 1..=50
  {
    large_content.push_str( &format!( "## Section {}\n\nContent for section {}.\n\n", i, i ) );
  }
  
  std::fs::write( &temp_file, &large_content ).unwrap();
  
  let results = create_sample_results();
  let reports : Vec< String > = ( 0..10 )
    .map( | i | 
    {
      PerformanceReport::new()
        .title( &format!( "Report {}", i ) )
        .generate( &results )
        .unwrap()
    })
    .collect();
  
  // Build chain with many sections
  let start_time = std::time::Instant::now();
  let mut chain = MarkdownUpdateChain::new( &temp_file ).unwrap();
  
  for ( i, report ) in reports.iter().enumerate()
  {
    chain = chain.add_section( &format!( "Section {}", i + 1 ), report );
  }
  
  let build_time = start_time.elapsed();
  println!( "Chain building time: {:.2?} for {} sections", build_time, chain.len() );
  
  // Measure validation performance
  let validation_start = std::time::Instant::now();
  let conflicts = chain.check_all_conflicts().unwrap();
  let validation_time = validation_start.elapsed();
  
  println!( "Validation time: {:.2?} (found {} conflicts)", validation_time, conflicts.len() );
  
  // Measure update performance if no conflicts
  if conflicts.is_empty()
  {
    let update_start = std::time::Instant::now();
    match chain.execute()
    {
      Ok( () ) =>
      {
        let update_time = update_start.elapsed();
        println!( "Update time: {:.2?} for {} sections", update_time, chain.len() );
        
        let final_size = std::fs::metadata( &temp_file ).unwrap().len();
        println!( "Final document size: {} bytes", final_size );
        println!( "✅ Bulk update completed successfully" );
      },
      Err( e ) => println!( "❌ Bulk update failed: {}", e ),
    }
  }
  else
  {
    println!( "⚠️ Conflicts prevent performance measurement: {:?}", conflicts );
  }
  
  std::fs::remove_file( &temp_file ).unwrap();
  println!();
}

/// Example 6: Integration with Templates and Validation
fn example_integrated_workflow()
{
  println!( "=== Example 6: Integrated Workflow ===" );
  
  let temp_file = std::env::temp_dir().join( "integrated_workflow_example.md" );
  std::fs::write( &temp_file, create_test_document() ).unwrap();
  
  let results = create_sample_results();
  
  // Step 1: Validate benchmark quality
  let validator = BenchmarkValidator::new()
    .min_samples( 5 )
    .max_coefficient_variation( 0.20 )
    .require_warmup( false );
  
  let validated_results = ValidatedResults::new( results.clone(), validator );
  println!( "Benchmark validation: {:.1}% reliability", validated_results.reliability_rate() );
  
  // Step 2: Generate multiple report types
  let performance_template = PerformanceReport::new()
    .title( "Integrated Performance Analysis" )
    .include_statistical_analysis( true )
    .add_custom_section( CustomSection::new(
      "Integration Notes",
      "This analysis combines validation, templating, and atomic updates."
    ));
  
  let comparison_template = ComparisonReport::new()
    .baseline( "slow_algorithm" )
    .candidate( "fast_algorithm" )
    .practical_significance_threshold( 0.05 );
  
  // Step 3: Generate all reports
  let performance_report = performance_template.generate( &results ).unwrap();
  let comparison_report = comparison_template.generate( &results ).unwrap();
  let validation_report = validated_results.validation_report();
  let quality_summary = format!(
    "## Quality Summary\n\n- Total benchmarks: {}\n- Reliable results: {}\n- Overall reliability: {:.1}%\n\n",
    validated_results.results.len(),
    validated_results.reliable_count(),
    validated_results.reliability_rate()
  );
  
  // Step 4: Atomic documentation update
  let chain = MarkdownUpdateChain::new( &temp_file ).unwrap()
    .add_section( "Algorithm Performance", &performance_report )
    .add_section( "Comparison Results", &comparison_report )
    .add_section( "Quality Assessment", &validation_report )
    .add_section( "Summary", &quality_summary );
  
  println!( "Integrated workflow updating {} sections", chain.len() );
  
  match chain.check_all_conflicts()
  {
    Ok( conflicts ) =>
    {
      if conflicts.is_empty()
      {
        match chain.execute()
        {
          Ok( () ) =>
          {
            println!( "✅ Integrated workflow completed successfully" );
            
            let final_content = std::fs::read_to_string( &temp_file ).unwrap();
            let lines = final_content.lines().count();
            let chars = final_content.len();
            
            println!( "   Final document: {} lines, {} characters", lines, chars );
            println!( "   All {} sections updated atomically", chain.len() );
            
            // Verify integration worked
            let has_performance = final_content.contains( "Integrated Performance Analysis" );
            let has_comparison = final_content.contains( "faster" ) || final_content.contains( "slower" );
            let has_validation = final_content.contains( "Benchmark Validation Report" );
            let has_summary = final_content.contains( "Quality Summary" );
            
            println!( "   Content verification: performance={}, comparison={}, validation={}, summary={}",
                     has_performance, has_comparison, has_validation, has_summary );
          },
          Err( e ) => println!( "❌ Integrated workflow failed: {}", e ),
        }
      }
      else
      {
        println!( "⚠️ Integration blocked by conflicts: {:?}", conflicts );
      }
    },
    Err( e ) => println!( "❌ Integration validation failed: {}", e ),
  }
  
  std::fs::remove_file( &temp_file ).unwrap();
  println!();
}

fn main()
{
  println!( "🚀 Comprehensive Update Chain Pattern Examples\n" );
  
  example_single_section_update();
  example_multi_section_atomic();
  example_error_handling();
  example_conflict_resolution();
  example_performance_efficiency();
  example_integrated_workflow();
  
  println!( "📋 Update Chain Pattern Use Cases Covered:" );
  println!( "✅ Single section updates with conflict detection" );
  println!( "✅ Multi-section atomic updates with rollback" );
  println!( "✅ Comprehensive error handling and recovery" );
  println!( "✅ Advanced conflict resolution strategies" );
  println!( "✅ Performance optimization for bulk updates" );
  println!( "✅ Full integration with validation and templates" );
  println!( "\n🎯 The Update Chain Pattern provides atomic, conflict-aware documentation updates" );
  println!( "   with comprehensive error handling and recovery mechanisms." );
}