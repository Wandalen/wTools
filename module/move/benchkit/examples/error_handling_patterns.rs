#![allow(clippy::all)]
//! Comprehensive Error Handling Pattern Examples
//!
//! This example demonstrates EVERY error handling scenario for enhanced features:
//! - Update Chain error recovery and rollback patterns
//! - Template generation error handling and validation
//! - Validation framework error scenarios and recovery
//! - File system error handling (permissions, disk space, etc.)
//! - Network and resource error handling patterns
//! - Graceful degradation strategies

#![ cfg( feature = "enabled" ) ]
#![ cfg( feature = "markdown_reports" ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::format_push_string ) ]
#![ allow( clippy::too_many_lines ) ]

use benchkit::prelude::*;
use std::collections::HashMap;
use std::time::Duration;
use std::path::PathBuf;

/// Create sample results for error handling demonstrations
fn create_sample_results() -> HashMap< String, BenchmarkResult >
{
  let mut results = HashMap::new();
  
  let fast_times = vec![
    Duration::from_micros( 100 ), Duration::from_micros( 102 ), Duration::from_micros( 98 ),
    Duration::from_micros( 101 ), Duration::from_micros( 99 ), Duration::from_micros( 100 ),
    Duration::from_micros( 103 ), Duration::from_micros( 97 ), Duration::from_micros( 101 )
  ];
  results.insert( "fast_algorithm".to_string(), BenchmarkResult::new( "fast_algorithm", fast_times ) );
  
  let slow_times = vec![
    Duration::from_millis( 1 ), Duration::from_millis( 1 ) + Duration::from_micros( 50 ),
    Duration::from_millis( 1 ) - Duration::from_micros( 30 ), Duration::from_millis( 1 ) + Duration::from_micros( 20 )
  ];
  results.insert( "slow_algorithm".to_string(), BenchmarkResult::new( "slow_algorithm", slow_times ) );
  
  results
}

/// Error Pattern 1: Update Chain File System Errors
fn pattern_update_chain_file_errors()
{
  println!( "=== Pattern 1: Update Chain File System Errors ===" );
  
  let results = create_sample_results();
  let report = PerformanceReport::new().generate( &results ).unwrap();
  
  // Test 1: Non-existent file
  println!( "\n🔍 Test 1: Non-existent file handling..." );
  let nonexistent_file = PathBuf::from( "/nonexistent/path/file.md" );
  
  match MarkdownUpdateChain::new( &nonexistent_file )
  {
    Ok( _chain ) => println!( "❌ Should have failed with non-existent file" ),
    Err( e ) =>
    {
      println!( "✅ Correctly caught non-existent file error: {}", e );
      println!( "   Recovery strategy: Create parent directories or use valid path" );
    }
  }
  
  // Test 2: Permission denied (read-only file)
  println!( "\n🔍 Test 2: Permission denied handling..." );
  let readonly_file = std::env::temp_dir().join( "readonly_test.md" );
  std::fs::write( &readonly_file, "# Test Document\n\n## Section\n\nContent." ).unwrap();
  
  // Make file read-only
  let metadata = std::fs::metadata( &readonly_file ).unwrap();
  let mut permissions = metadata.permissions();
  permissions.set_readonly( true );
  std::fs::set_permissions( &readonly_file, permissions ).unwrap();
  
  match MarkdownUpdateChain::new( &readonly_file )
  {
    Ok( chain ) =>
    {
      let chain_with_section = chain.add_section( "Section", &report );
      
      match chain_with_section.execute()
      {
        Ok( () ) => println!( "❌ Should have failed with read-only file" ),
        Err( e ) =>
        {
          println!( "✅ Correctly caught permission error: {}", e );
          println!( "   Recovery strategy: Check file permissions before operations" );
          
          // Demonstrate recovery
          let mut recovery_permissions = std::fs::metadata( &readonly_file ).unwrap().permissions();
          recovery_permissions.set_readonly( false );
          std::fs::set_permissions( &readonly_file, recovery_permissions ).unwrap();
          
          let recovery_chain = MarkdownUpdateChain::new( &readonly_file ).unwrap()
            .add_section( "Section", &report );
          
          match recovery_chain.execute()
          {
            Ok( () ) => println!( "   ✅ Recovery successful after fixing permissions" ),
            Err( e ) => println!( "   ❌ Recovery failed: {}", e ),
          }
        }
      }
    },
    Err( e ) => println!( "✅ Correctly caught file access error: {}", e ),
  }
  
  // Test 3: Conflicting section names
  println!( "\n🔍 Test 3: Section conflict handling..." );
  let conflict_file = std::env::temp_dir().join( "conflict_test.md" );
  let conflict_content = r#"# Document with Conflicts

## Performance

First performance section.

## Algorithm Performance

Detailed algorithm analysis.

## Performance

Second performance section (duplicate).
"#;
  
  std::fs::write( &conflict_file, conflict_content ).unwrap();
  
  let conflict_chain = MarkdownUpdateChain::new( &conflict_file ).unwrap()
    .add_section( "Performance", &report );
  
  match conflict_chain.check_all_conflicts()
  {
    Ok( conflicts ) =>
    {
      if !conflicts.is_empty()
      {
        println!( "✅ Correctly detected section conflicts:" );
        for conflict in &conflicts
        {
          println!( "   - {}", conflict );
        }
        
        println!( "   Recovery strategies:" );
        println!( "   1. Use more specific section names" );
        println!( "   2. Modify document structure to remove duplicates" );
        println!( "   3. Use exact section matching with context" );
        
        // Demonstrate recovery with specific section name
        let recovery_chain = MarkdownUpdateChain::new( &conflict_file ).unwrap()
          .add_section( "Algorithm Performance", &report );
        
        match recovery_chain.check_all_conflicts()
        {
          Ok( recovery_conflicts ) =>
          {
            if recovery_conflicts.is_empty()
            {
              println!( "   ✅ Recovery successful with specific section name" );
              match recovery_chain.execute()
              {
                Ok( () ) => println!( "   ✅ Document updated successfully" ),
                Err( e ) => println!( "   ❌ Update failed: {}", e ),
              }
            }
            else
            {
              println!( "   ⚠️ Still has conflicts: {:?}", recovery_conflicts );
            }
          },
          Err( e ) => println!( "   ❌ Recovery validation failed: {}", e ),
        }
      }
      else
      {
        println!( "❌ Should have detected conflicts with duplicate sections" );
      }
    },
    Err( e ) => println!( "❌ Conflict check failed: {}", e ),
  }
  
  // Cleanup
  let _ = std::fs::remove_file( &readonly_file );
  let _ = std::fs::remove_file( &conflict_file );
  
  println!();
}

/// Error Pattern 2: Template Generation Errors
fn pattern_template_generation_errors()
{
  println!( "=== Pattern 2: Template Generation Errors ===" );
  
  let results = create_sample_results();
  
  // Test 1: Empty results handling
  println!( "\n🔍 Test 1: Empty results handling..." );
  let empty_results = HashMap::new();
  
  let performance_template = PerformanceReport::new()
    .title( "Empty Results Test" );
  
  match performance_template.generate( &empty_results )
  {
    Ok( report ) =>
    {
      println!( "✅ Empty results handled gracefully: {} characters", report.len() );
      println!( "   Contains fallback message: {}", report.contains( "No benchmark results available" ) );
    },
    Err( e ) => println!( "❌ Empty results caused error: {}", e ),
  }
  
  // Test 2: Missing baseline in comparison
  println!( "\n🔍 Test 2: Missing baseline handling..." );
  let missing_baseline_template = ComparisonReport::new()
    .baseline( "nonexistent_baseline" )
    .candidate( "fast_algorithm" );
  
  match missing_baseline_template.generate( &results )
  {
    Ok( _report ) => println!( "❌ Should have failed with missing baseline" ),
    Err( e ) =>
    {
      println!( "✅ Correctly caught missing baseline: {}", e );
      println!( "   Error message is helpful: {}", e.to_string().contains( "nonexistent_baseline" ) );
      
      // Demonstrate recovery by checking available keys
      println!( "   Available algorithms: {:?}", results.keys().collect::< Vec< _ > >() );
      
      let recovery_template = ComparisonReport::new()
        .baseline( "slow_algorithm" )
        .candidate( "fast_algorithm" );
      
      match recovery_template.generate( &results )
      {
        Ok( report ) =>
        {
          println!( "   ✅ Recovery successful with valid baseline: {} characters", report.len() );
        },
        Err( e ) => println!( "   ❌ Recovery failed: {}", e ),
      }
    }
  }
  
  // Test 3: Missing candidate in comparison
  println!( "\n🔍 Test 3: Missing candidate handling..." );
  let missing_candidate_template = ComparisonReport::new()
    .baseline( "fast_algorithm" )
    .candidate( "nonexistent_candidate" );
  
  match missing_candidate_template.generate( &results )
  {
    Ok( _report ) => println!( "❌ Should have failed with missing candidate" ),
    Err( e ) =>
    {
      println!( "✅ Correctly caught missing candidate: {}", e );
      println!( "   Error provides algorithm name: {}", e.to_string().contains( "nonexistent_candidate" ) );
    }
  }
  
  // Test 4: Invalid custom section content
  println!( "\n🔍 Test 4: Malformed custom section handling..." );
  let custom_template = PerformanceReport::new()
    .title( "Custom Section Test" )
    .add_custom_section( CustomSection::new( "", "" ) );  // Empty title and content
  
  match custom_template.generate( &results )
  {
    Ok( report ) =>
    {
      println!( "✅ Empty custom section handled: {} characters", report.len() );
      println!( "   Report remains valid despite empty section" );
    },
    Err( e ) => println!( "❌ Custom section caused error: {}", e ),
  }
  
  println!();
}

/// Error Pattern 3: Validation Framework Errors
fn pattern_validation_errors()
{
  println!( "=== Pattern 3: Validation Framework Errors ===" );
  
  // Test 1: Invalid validator configuration
  println!( "\n🔍 Test 1: Invalid validator configuration..." );
  
  // The validator builder pattern should handle edge cases gracefully
  let edge_case_validator = BenchmarkValidator::new()
    .min_samples( 0 )  // Edge case: zero samples
    .max_coefficient_variation( -0.1 )  // Edge case: negative CV
    .max_time_ratio( 0.0 )  // Edge case: zero ratio
    .min_measurement_time( Duration::from_nanos( 0 ) );  // Edge case: zero duration
  
  println!( "✅ Validator created with edge case values (implementation should handle gracefully)" );
  
  let results = create_sample_results();
  let validation_results = edge_case_validator.validate_result( &results[ "fast_algorithm" ] );
  println!( "   Validation with edge case config: {} warnings", validation_results.len() );
  
  // Test 2: Malformed benchmark data
  println!( "\n🔍 Test 2: Malformed benchmark data handling..." );
  
  // Create result with single measurement (edge case)
  let single_measurement = BenchmarkResult::new( 
    "single_measurement", 
    vec![ Duration::from_micros( 100 ) ] 
  );
  
  let validator = BenchmarkValidator::new();
  let single_warnings = validator.validate_result( &single_measurement );
  
  println!( "✅ Single measurement handled: {} warnings", single_warnings.len() );
  for warning in single_warnings
  {
    println!( "   - {}", warning );
  }
  
  // Test 3: Zero duration measurements
  println!( "\n🔍 Test 3: Zero duration measurement handling..." );
  
  let zero_duration_result = BenchmarkResult::new(
    "zero_duration",
    vec![ Duration::from_nanos( 0 ), Duration::from_nanos( 1 ), Duration::from_nanos( 0 ) ]
  );
  
  let zero_warnings = validator.validate_result( &zero_duration_result );
  println!( "✅ Zero duration measurements handled: {} warnings", zero_warnings.len() );
  
  // Test 4: Extremely variable data
  println!( "\n🔍 Test 4: Extremely variable data handling..." );
  
  let extreme_variance_result = BenchmarkResult::new(
    "extreme_variance",
    vec![
      Duration::from_nanos( 1 ),
      Duration::from_millis( 1 ),
      Duration::from_nanos( 1 ),
      Duration::from_millis( 1 ),
      Duration::from_nanos( 1 ),
    ]
  );
  
  let extreme_warnings = validator.validate_result( &extreme_variance_result );
  println!( "✅ Extreme variance data handled: {} warnings", extreme_warnings.len() );
  for warning in extreme_warnings.iter().take( 3 )  // Show first 3
  {
    println!( "   - {}", warning );
  }
  
  // Test 5: ValidatedResults with problematic data
  println!( "\n🔍 Test 5: ValidatedResults error recovery..." );
  
  let mut problematic_results = HashMap::new();
  problematic_results.insert( "normal".to_string(), results[ "fast_algorithm" ].clone() );
  problematic_results.insert( "single".to_string(), single_measurement );
  problematic_results.insert( "extreme".to_string(), extreme_variance_result );
  
  let validated_results = ValidatedResults::new( problematic_results, validator );
  
  println!( "✅ ValidatedResults handles mixed quality data:" );
  println!( "   Total results: {}", validated_results.results.len() );
  println!( "   Reliable results: {}", validated_results.reliable_count() );
  println!( "   Reliability rate: {:.1}%", validated_results.reliability_rate() );
  
  // Demonstrate graceful degradation: work with reliable results only
  let reliable_only = validated_results.reliable_results();
  println!( "   Reliable subset: {} results available for analysis", reliable_only.len() );
  
  println!();
}

/// Error Pattern 4: Resource and System Errors
fn pattern_system_errors()
{
  println!( "=== Pattern 4: System and Resource Errors ===" );
  
  let results = create_sample_results();
  
  // Test 1: Disk space simulation (create very large content)
  println!( "\n🔍 Test 1: Large content handling..." );
  
  let large_content = "x".repeat( 10_000_000 );  // 10MB string
  let large_template = PerformanceReport::new()
    .title( "Large Content Test" )
    .add_custom_section( CustomSection::new( "Large Section", &large_content ) );
  
  match large_template.generate( &results )
  {
    Ok( report ) =>
    {
      println!( "✅ Large content generated: {:.1}MB", report.len() as f64 / 1_000_000.0 );
      
      // Test writing large content to disk
      let large_file = std::env::temp_dir().join( "large_test.md" );
      
      match std::fs::write( &large_file, &report )
      {
        Ok( () ) =>
        {
          println!( "   ✅ Large file written successfully" );
          let file_size = std::fs::metadata( &large_file ).unwrap().len();
          println!( "   File size: {:.1}MB", file_size as f64 / 1_000_000.0 );
          
          std::fs::remove_file( &large_file ).unwrap();
        },
        Err( e ) =>
        {
          println!( "   ⚠️ Large file write failed: {}", e );
          println!( "   This might indicate disk space or system limits" );
        }
      }
    },
    Err( e ) =>
    {
      println!( "⚠️ Large content generation failed: {}", e );
      println!( "   This might indicate memory limitations" );
    }
  }
  
  // Test 2: Invalid path characters
  println!( "\n🔍 Test 2: Invalid path character handling..." );
  
  let invalid_paths = vec![
    "/invalid\0null/path.md",  // Null character
    "con.md",  // Reserved name on Windows
    "file?.md",  // Invalid character on Windows
  ];
  
  for invalid_path in invalid_paths
  {
    match std::fs::write( invalid_path, "test content" )
    {
      Ok( () ) =>
      {
        println!( "   ⚠️ Invalid path '{}' was accepted (platform-dependent)", invalid_path );
        let _ = std::fs::remove_file( invalid_path );
      },
      Err( e ) =>
      {
        println!( "   ✅ Invalid path '{}' correctly rejected: {}", invalid_path, e );
      }
    }
  }
  
  // Test 3: Concurrent access simulation
  println!( "\n🔍 Test 3: Concurrent access handling..." );
  
  let concurrent_file = std::env::temp_dir().join( "concurrent_test.md" );
  std::fs::write( &concurrent_file, "# Test\n\n## Section\n\nContent." ).unwrap();
  
  // Simulate file being locked by another process (simplified simulation)
  let chain1 = MarkdownUpdateChain::new( &concurrent_file ).unwrap()
    .add_section( "Section", "Updated by chain 1" );
  
  let chain2 = MarkdownUpdateChain::new( &concurrent_file ).unwrap()
    .add_section( "Section", "Updated by chain 2" );
  
  // Execute both chains to see how conflicts are handled
  match chain1.execute()
  {
    Ok( () ) =>
    {
      println!( "   ✅ Chain 1 execution successful" );
      
      match chain2.execute()
      {
        Ok( () ) =>
        {
          println!( "   ✅ Chain 2 execution successful" );
          
          let final_content = std::fs::read_to_string( &concurrent_file ).unwrap();
          let chain2_content = final_content.contains( "Updated by chain 2" );
          
          if chain2_content
          {
            println!( "   → Chain 2 overwrote chain 1 (last writer wins)" );
          }
          else
          {
            println!( "   → Chain 1 result preserved" );
          }
        },
        Err( e ) => println!( "   ❌ Chain 2 failed: {}", e ),
      }
    },
    Err( e ) => println!( "   ❌ Chain 1 failed: {}", e ),
  }
  
  std::fs::remove_file( &concurrent_file ).unwrap();
  
  println!();
}

/// Error Pattern 5: Graceful Degradation Strategies
fn pattern_graceful_degradation()
{
  println!( "=== Pattern 5: Graceful Degradation Strategies ===" );
  
  let results = create_sample_results();
  
  // Strategy 1: Fallback to basic templates when custom sections fail
  println!( "\n🔧 Strategy 1: Template fallback patterns..." );
  
  let complex_template = PerformanceReport::new()
    .title( "Complex Analysis" )
    .include_statistical_analysis( true )
    .add_custom_section( CustomSection::new( "Advanced Analysis", "Complex content here" ) );
  
  match complex_template.generate( &results )
  {
    Ok( report ) =>
    {
      println!( "✅ Complex template succeeded: {} characters", report.len() );
    },
    Err( _e ) =>
    {
      println!( "⚠️ Complex template failed, falling back to basic template..." );
      
      let fallback_template = PerformanceReport::new()
        .title( "Basic Analysis" )
        .include_statistical_analysis( false );  // Simplified version
      
      match fallback_template.generate( &results )
      {
        Ok( report ) =>
        {
          println!( "   ✅ Fallback template succeeded: {} characters", report.len() );
        },
        Err( e ) =>
        {
          println!( "   ❌ Even fallback failed: {}", e );
        }
      }
    }
  }
  
  // Strategy 2: Partial update when full atomic update fails
  println!( "\n🔧 Strategy 2: Partial update fallback..." );
  
  let test_file = std::env::temp_dir().join( "fallback_test.md" );
  let test_content = r#"# Test Document

## Section 1

Content 1.

## Section 2

Content 2.

## Section 3

Content 3.
"#;
  
  std::fs::write( &test_file, test_content ).unwrap();
  
  let report1 = PerformanceReport::new().generate( &results ).unwrap();
  let report2 = "This is a simple report.";
  let invalid_report = ""; // Empty report might cause issues
  
  // Try atomic update with potentially problematic content
  let atomic_chain = MarkdownUpdateChain::new( &test_file ).unwrap()
    .add_section( "Section 1", &report1 )
    .add_section( "Section 2", report2 )
    .add_section( "Section 3", invalid_report );
  
  match atomic_chain.execute()
  {
    Ok( () ) => println!( "✅ Atomic update succeeded" ),
    Err( e ) =>
    {
      println!( "⚠️ Atomic update failed: {}", e );
      println!( "   Falling back to individual section updates..." );
      
      // Fallback: update sections individually
      let updates = vec![
        ( "Section 1", report1.as_str() ),
        ( "Section 2", report2 ),
        ( "Section 3", invalid_report ),
      ];
      
      let mut successful_updates = 0;
      
      for ( section, content ) in updates
      {
        let individual_chain = MarkdownUpdateChain::new( &test_file ).unwrap()
          .add_section( section, content );
        
        match individual_chain.execute()
        {
          Ok( () ) =>
          {
            successful_updates += 1;
            println!( "   ✅ {} updated successfully", section );
          },
          Err( e ) =>
          {
            println!( "   ❌ {} update failed: {}", section, e );
          }
        }
      }
      
      println!( "   Partial success: {}/3 sections updated", successful_updates );
    }
  }
  
  // Strategy 3: Quality-based selective processing
  println!( "\n🔧 Strategy 3: Quality-based selective processing..." );
  
  // Create mixed quality results
  let mut mixed_results = results.clone();
  mixed_results.insert( 
    "unreliable".to_string(), 
    BenchmarkResult::new( "unreliable", vec![ Duration::from_nanos( 1 ) ] )
  );
  
  let validator = BenchmarkValidator::new();
  let validated_results = ValidatedResults::new( mixed_results.clone(), validator );
  
  println!( "   Mixed quality data: {:.1}% reliable", validated_results.reliability_rate() );
  
  if validated_results.reliability_rate() < 50.0
  {
    println!( "   ⚠️ Low reliability detected, using conservative approach..." );
    
    // Use only reliable results
    let reliable_only = validated_results.reliable_results();
    
    if reliable_only.is_empty()
    {
      println!( "   ❌ No reliable results - generating warning report" );
      
      let warning_template = PerformanceReport::new()
        .title( "Benchmark Quality Warning" )
        .add_custom_section( CustomSection::new(
          "Quality Issues",
          "⚠️ **Warning**: All benchmark results failed quality validation. Please review benchmark methodology and increase sample sizes."
        ));
      
      match warning_template.generate( &HashMap::new() )
      {
        Ok( warning_report ) =>
        {
          println!( "   ✅ Warning report generated: {} characters", warning_report.len() );
        },
        Err( e ) =>
        {
          println!( "   ❌ Even warning report failed: {}", e );
        }
      }
    }
    else
    {
      println!( "   ✅ Using {} reliable results for analysis", reliable_only.len() );
      
      let conservative_template = PerformanceReport::new()
        .title( "Conservative Analysis (Reliable Results Only)" )
        .add_context( "Analysis limited to statistically reliable benchmark results" );
      
      match conservative_template.generate( &reliable_only )
      {
        Ok( report ) =>
        {
          println!( "   ✅ Conservative analysis generated: {} characters", report.len() );
        },
        Err( e ) =>
        {
          println!( "   ❌ Conservative analysis failed: {}", e );
        }
      }
    }
  }
  else
  {
    println!( "   ✅ Quality acceptable, proceeding with full analysis" );
  }
  
  std::fs::remove_file( &test_file ).unwrap();
  
  println!();
}

fn main()
{
  println!( "🚀 Comprehensive Error Handling Pattern Examples\n" );
  
  pattern_update_chain_file_errors();
  pattern_template_generation_errors();
  pattern_validation_errors();
  pattern_system_errors();
  pattern_graceful_degradation();
  
  println!( "📋 Error Handling Patterns Covered:" );
  println!( "✅ Update Chain: file system errors, permissions, conflicts" );
  println!( "✅ Templates: missing data, invalid parameters, empty results" );
  println!( "✅ Validation: edge cases, malformed data, extreme variance" );
  println!( "✅ System: resource limits, invalid paths, concurrent access" );
  println!( "✅ Graceful Degradation: fallbacks, partial updates, quality-based processing" );
  println!( "\n🎯 These patterns ensure robust operation under adverse conditions" );
  println!( "   with meaningful error messages and automatic recovery strategies." );
  
  println!( "\n🛡️ Error Handling Best Practices Demonstrated:" );
  println!( "• Always check for conflicts before atomic operations" );
  println!( "• Provide helpful error messages with context" );
  println!( "• Implement fallback strategies for graceful degradation" );
  println!( "• Validate inputs early and handle edge cases" );
  println!( "• Use reliable results when quality is questionable" );
  println!( "• Clean up resources even when operations fail" );
}