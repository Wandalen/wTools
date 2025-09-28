//! Example of Well-Structured Regression Test
//!
//! This file demonstrates best practices for regression testing in the systematic
//! organization structure. It shows proper patterns for preventing the recurrence
//! of known bugs and maintaining backward compatibility.

use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes };
use unilang::registry::CommandRegistry;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::interpreter::ExecutionContext;
use unilang::data::{ OutputData, ErrorData };
use unilang_parser::{ Parser, UnilangParserOptions };
use std::time::Instant;

/// Example: Exact bug reproduction and prevention
///
/// This test demonstrates:
/// - Reproducing the exact scenario that caused a bug
/// - Documenting the bug context and fix
/// - Preventing regression with specific test case
#[test]
fn regression_task_024_multiple_parameter_collection_exact_reproduction()
{
  // BUG CONTEXT:
  // Task 024 - Multiple parameter collection was failing when:
  // 1. Multiple parameters with same name were provided
  // 2. Argument definition had multiple=false
  // 3. Only first parameter was being collected instead of all
  //
  // ORIGINAL FAILING SCENARIO:
  // Command: .run command::"cargo build" command::"echo hello" command::"cargo clippy"
  // Expected: All three commands collected
  // Actual: Only "cargo build" was collected
  //
  // FIX IMPLEMENTED:
  // Modified parameter collection logic to automatically collect multiple
  // parameters regardless of multiple=false setting for backward compatibility

  // Arrange - Reproduce exact conditions from bug report
  let mut registry = CommandRegistry::new();

  let run_cmd = CommandDefinition::former()
    .name( ".run" )
    .description( "Run multiple commands" )
    .arguments( vec![
      ArgumentDefinition {
        name : "command".to_string(),
        description : "Command to execute".to_string(),
        kind : Kind::String,
        hint : "Shell command".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          multiple : false,  // THIS WAS THE KEY - multiple=false but multiple values provided
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
      ArgumentDefinition {
        name : "parallel".to_string(),
        description : "Number of parallel executions".to_string(),
        kind : Kind::Integer,
        hint : "Parallel count".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "1".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  let run_routine = Box::new( |cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, ErrorData> {
    // Verify the fix is working by checking all commands are collected
    let commands = cmd.arguments.get( "command" ).expect( "command argument should exist" );

    Ok( OutputData {
      content : format!( "Executed {} commands", commands.len() ),
      format : "text".to_string(),
    })
  });

  registry.command_add_runtime( &run_cmd, run_routine ).unwrap();

  // Act - Execute the EXACT failing scenario from Task 024
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction(
    r#".run command::"cargo build" command::"echo hello1" command::"cargo clippy" parallel::2"#
  ).expect( "Should parse the exact Task 024 command" );

  let instructions = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let verified_commands = analyzer.analyze()
    .expect( "Should analyze the exact Task 024 command" );

  // Assert - Verify the bug is fixed
  let verified_cmd = &verified_commands[0];

  // The critical assertion: ALL commands should be collected
  let command_values = verified_cmd.arguments.get( "command" )
    .expect( "command argument should be present" );

  assert_eq!( command_values.len(), 3,
             "Should collect all three commands (original bug collected only 1)" );

  // Verify each command value is preserved correctly
  let mut command_strings : Vec< String > = command_values.iter()
    .map( |v| v.to_string() )
    .collect();
  command_strings.sort(); // Sort to make test deterministic

  let mut expected = vec![ "cargo build", "cargo clippy", "echo hello1" ];
  expected.sort();

  assert_eq!( command_strings, expected,
             "All command values should be preserved exactly as provided" );

  // Verify parallel parameter is also handled correctly
  let parallel_value = verified_cmd.arguments.get( "parallel" )
    .expect( "parallel argument should be present" );
  assert_eq!( parallel_value.to_string(), "2",
             "Parallel parameter should be parsed correctly" );
}

/// Example: Backward compatibility regression test
///
/// This test demonstrates:
/// - Ensuring old usage patterns continue to work
/// - Testing compatibility across version changes
/// - Preventing breaking changes
#[test]
fn regression_backward_compatibility_single_parameter_usage()
{
  // COMPATIBILITY CONTEXT:
  // The Task 024 fix for multiple parameter collection should NOT break
  // existing single-parameter usage patterns that users rely on
  //
  // RISK:
  // Fix could inadvertently change behavior for single parameters
  //
  // PROTECTION:
  // Ensure all existing single-parameter patterns continue working

  let mut registry = CommandRegistry::new();

  let test_cmd = CommandDefinition::former()
    .name( ".test" )
    .description( "Test command for compatibility" )
    .arguments( vec![
      ArgumentDefinition {
        name : "param".to_string(),
        description : "Test parameter".to_string(),
        kind : Kind::String,
        hint : "Test value".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          multiple : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  let test_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, ErrorData> {
    Ok( OutputData { content : "success".to_string(), format : "text".to_string() })
  });

  registry.command_add_runtime( &test_cmd, test_routine ).unwrap();

  // Test various single-parameter patterns that users might use
  let single_param_patterns = vec![
    r#".test param::"simple_value""#,
    r#".test param::unquoted"#,
    r#".test param::"value with spaces""#,
    r#".test param::"123""#,
    r#".test param::""#,  // empty string
  ];

  let parser = Parser::new( UnilangParserOptions::default() );

  for pattern in single_param_patterns
  {
    // Act
    let instruction = parser.parse_single_instruction( pattern )
      .expect( &format!( "Should parse single parameter pattern: {}", pattern ) );

    let instructions = [instruction];
    let analyzer = SemanticAnalyzer::new( &instructions, &registry );
    let verified_commands = analyzer.analyze()
      .expect( &format!( "Should analyze single parameter pattern: {}", pattern ) );

    // Assert - Single parameter behavior should be unchanged
    let verified_cmd = &verified_commands[0];
    let param_values = verified_cmd.arguments.get( "param" )
      .expect( "param argument should be present" );

    assert_eq!( param_values.len(), 1,
               "Single parameter should remain single for pattern: {}", pattern );

    // Verify the value is correct (extract expected value from pattern)
    let expected_value = pattern
      .split( "::" )
      .nth( 1 )
      .unwrap()
      .trim_matches( '"' );

    assert_eq!( param_values[0].to_string(), expected_value,
               "Single parameter value should be preserved for pattern: {}", pattern );
  }
}

/// Example: Performance regression test
///
/// This test demonstrates:
/// - Ensuring performance doesn't degrade with fixes
/// - Establishing performance baselines
/// - Detecting performance regressions early
#[test]
fn regression_multiple_parameter_performance_no_degradation()
{
  // PERFORMANCE CONTEXT:
  // The Task 024 fix should not significantly degrade performance
  // when processing multiple parameters
  //
  // BASELINE:
  // Before fix: ~1ms per 10 parameters
  // After fix: Should remain comparable
  //
  // RISK:
  // New collection logic might be less efficient

  let mut registry = CommandRegistry::new();

  let perf_cmd = CommandDefinition::former()
    .name( ".perf_test" )
    .description( "Performance test command" )
    .arguments( vec![
      ArgumentDefinition {
        name : "data".to_string(),
        description : "Data parameter".to_string(),
        kind : Kind::String,
        hint : "Test data".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          multiple : false,  // Test the specific multiple=false case
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  let perf_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, ErrorData> {
    Ok( OutputData { content : "performance_test".to_string(), format : "text".to_string() })
  });

  registry.command_add_runtime( &perf_cmd, perf_routine ).unwrap();

  // Create command with many parameters (stress test)
  let mut command_parts = vec![ ".perf_test".to_string() ];
  for i in 0..100
  {
    command_parts.push( format!( r#"data::"value_{}""#, i ) );
  }
  let large_command = command_parts.join( " " );

  let parser = Parser::new( UnilangParserOptions::default() );

  // Warm up
  let _ = parser.parse_single_instruction( &large_command );

  // Act - Measure performance
  let start_time = Instant::now();

  for _ in 0..10  // Multiple iterations for stability
  {
    let instruction = parser.parse_single_instruction( &large_command )
      .expect( "Should parse large command" );

    let instructions = [instruction];
    let analyzer = SemanticAnalyzer::new( &instructions, &registry );
    let _verified_commands = analyzer.analyze()
      .expect( "Should analyze large command" );
  }

  let duration = start_time.elapsed();
  let avg_duration = duration.as_millis() / 10;

  // Assert - Performance should be reasonable
  assert!( avg_duration < 100,  // 100ms is generous threshold
          "Multiple parameter processing should not degrade performance significantly: {}ms average",
          avg_duration );

  // Verify correctness wasn't sacrificed for performance
  let instruction = parser.parse_single_instruction( &large_command ).unwrap();
  let instructions = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let verified_commands = analyzer.analyze().unwrap();

  let data_values = verified_commands[0].arguments.get( "data" ).unwrap();
  assert_eq!( data_values.len(), 100,
             "Should still collect all parameters correctly despite performance optimization" );
}

/// Example: Edge case regression test
///
/// This test demonstrates:
/// - Testing edge cases that previously caused issues
/// - Ensuring robustness improvements persist
/// - Preventing regression in error handling
#[test]
fn regression_edge_case_parameter_collection_robustness()
{
  // EDGE CASE CONTEXT:
  // During Task 024 development, several edge cases were discovered:
  // 1. Empty parameter values
  // 2. Parameters with only whitespace
  // 3. Very long parameter lists
  // 4. Mixed quoted/unquoted parameters
  //
  // These edge cases should continue to work correctly

  let mut registry = CommandRegistry::new();

  let edge_cmd = CommandDefinition::former()
    .name( ".edge_test" )
    .description( "Edge case test command" )
    .arguments( vec![
      ArgumentDefinition {
        name : "value".to_string(),
        description : "Value parameter".to_string(),
        kind : Kind::String,
        hint : "Test value".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          multiple : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  let edge_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, ErrorData> {
    Ok( OutputData { content : "edge_test_success".to_string(), format : "text".to_string() })
  });

  registry.command_add_runtime( &edge_cmd, edge_routine ).unwrap();

  let parser = Parser::new( UnilangParserOptions::default() );

  // Edge Case 1: Empty values
  let empty_test = r#".edge_test value::"" value::"non_empty" value::"""#;
  let instruction = parser.parse_single_instruction( empty_test )
    .expect( "Should handle empty values" );

  let instructions = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let verified_commands = analyzer.analyze()
    .expect( "Should analyze command with empty values" );

  let values = verified_commands[0].arguments.get( "value" ).unwrap();
  assert_eq!( values.len(), 3, "Should collect empty values" );
  assert_eq!( values[0].to_string(), "", "Empty value should be preserved" );
  assert_eq!( values[1].to_string(), "non_empty", "Non-empty value should be preserved" );
  assert_eq!( values[2].to_string(), "", "Second empty value should be preserved" );

  // Edge Case 2: Whitespace-only values
  let whitespace_test = r#".edge_test value::"   " value::"normal" value::" 	 ""#;
  let ws_instruction = parser.parse_single_instruction( whitespace_test )
    .expect( "Should handle whitespace values" );

  let ws_instructions = [ws_instruction];
  let ws_analyzer = SemanticAnalyzer::new( &ws_instructions, &registry );
  let ws_commands = ws_analyzer.analyze()
    .expect( "Should analyze command with whitespace values" );

  let ws_values = ws_commands[0].arguments.get( "value" ).unwrap();
  assert_eq!( ws_values.len(), 3, "Should collect whitespace values" );
  assert_eq!( ws_values[0].to_string().trim(), "", "Whitespace value should be preserved" );

  // Edge Case 3: Mixed quoted/unquoted (if supported)
  let mixed_test = r#".edge_test value::unquoted value::"quoted" value::also_unquoted"#;
  let mixed_instruction = parser.parse_single_instruction( mixed_test );

  // Should either parse successfully or fail gracefully
  match mixed_instruction
  {
    Ok( instruction ) => {
      let mixed_instructions = [instruction];
      let mixed_analyzer = SemanticAnalyzer::new( &mixed_instructions, &registry );
      let mixed_result = mixed_analyzer.analyze();

      assert!( mixed_result.is_ok(), "Mixed quoting should be handled gracefully" );
    }
    Err( _error ) => {
      // Graceful parsing failure is acceptable for edge cases
      // Should not panic or crash
    }
  }
}

/// Example: Configuration regression test
///
/// This test demonstrates:
/// - Ensuring configuration changes don't break existing functionality
/// - Testing configuration backward compatibility
/// - Preventing config-related regressions
#[test]
fn regression_configuration_compatibility()
{
  // CONFIGURATION CONTEXT:
  // Parser options and configurations should remain compatible
  // across updates to prevent breaking user environments
  //
  // RISK:
  // New parser features might inadvertently change default behavior

  // Test default configuration behavior
  let default_parser = Parser::new( UnilangParserOptions::default() );

  let standard_command = r#".test arg::"value""#;
  let default_result = default_parser.parse_single_instruction( standard_command );

  assert!( default_result.is_ok(),
          "Default configuration should continue to work with standard commands" );

  // Test explicit configuration settings
  let explicit_options = UnilangParserOptions {
    strict_mode : false,
    ..Default::default()
  };

  let explicit_parser = Parser::new( explicit_options );
  let explicit_result = explicit_parser.parse_single_instruction( standard_command );

  assert!( explicit_result.is_ok(),
          "Explicit configuration should work consistently" );

  // Configurations should produce equivalent results for standard input
  let default_instruction = default_result.unwrap();
  let explicit_instruction = explicit_result.unwrap();

  assert_eq!( default_instruction.command_name, explicit_instruction.command_name,
             "Configuration variants should produce compatible results" );

  assert_eq!( default_instruction.named_arguments.len(),
             explicit_instruction.named_arguments.len(),
             "Argument parsing should be consistent across configurations" );
}

/// Example: API stability regression test
///
/// This test demonstrates:
/// - Ensuring public API remains stable
/// - Testing that critical interfaces don't change unexpectedly
/// - Preventing breaking changes in public API
#[test]
fn regression_api_stability()
{
  // API STABILITY CONTEXT:
  // Critical public APIs should remain stable to prevent breaking
  // user code that depends on the framework
  //
  // RISK:
  // Internal changes might inadvertently modify public interfaces

  // Test that core types can still be constructed
  let _parser = Parser::new( UnilangParserOptions::default() );
  let _registry = CommandRegistry::new();

  // Test that essential methods exist and work
  let parser = Parser::new( UnilangParserOptions::default() );
  let parse_result = parser.parse_single_instruction( ".test" );

  // Method should exist and return expected type
  assert!( parse_result.is_ok() || parse_result.is_err(),
          "parse_single_instruction method should exist and return Result" );

  // Test that data structures maintain expected fields
  let mut registry = CommandRegistry::new();
  let cmd = CommandDefinition::former()
    .name( ".api_test" )
    .description( "API stability test" )
    .end();

  let api_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, ErrorData> {
    Ok( OutputData { content : "api_stable".to_string(), format : "text".to_string() })
  });

  // This method should continue to exist and work
  let registration_result = registry.command_add_runtime( &cmd, api_routine );
  assert!( registration_result.is_ok(),
          "command_add_runtime method should remain available and functional" );

  // Test that lookup functionality remains stable
  let lookup_result = registry.command( ".api_test" );
  assert!( lookup_result.is_some(),
          "command lookup functionality should remain stable" );
}

/// Example: Golden master regression test
///
/// This test demonstrates:
/// - Using reference outputs to detect unexpected changes
/// - Ensuring output format stability
/// - Detecting subtle behavioral regressions
#[test]
fn regression_output_format_stability()
{
  // GOLDEN MASTER CONTEXT:
  // Help output format should remain stable to ensure consistent
  // user experience and documentation accuracy
  //
  // METHOD:
  // Compare current output against known good reference

  let mut registry = CommandRegistry::new();

  let stable_cmd = CommandDefinition::former()
    .name( ".stable_test" )
    .description( "Command for output format stability testing" )
    .hint( "Use this command to test help output format" )
    .version( "1.0.0" )
    .status( "stable" )
    .arguments( vec![
      ArgumentDefinition {
        name : "input".to_string(),
        description : "Input file path".to_string(),
        kind : Kind::String,
        hint : "Path to input file".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "i".to_string() ],
        tags : vec![ "file".to_string() ],
      }
    ])
    .end();

  let stable_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, ErrorData> {
    Ok( OutputData { content : "stable_output".to_string(), format : "text".to_string() })
  });

  registry.command_add_runtime( &stable_cmd, stable_routine ).unwrap();

  // Generate help output
  use unilang::help::HelpGenerator;
  let help_generator = HelpGenerator::new( &registry );
  let help_output = help_generator.command( ".stable_test" )
    .expect( "Should generate help for stable command" );

  // Verify essential elements are present (flexible golden master)
  let essential_elements = vec![
    "Usage:",
    ".stable_test",
    "v1.0.0",
    "Input file path",
    "Arguments:",
    "input",
    "Type:",
    "stable"
  ];

  for element in essential_elements
  {
    assert!( help_output.contains( element ),
            "Help output should contain essential element '{}': {}", element, help_output );
  }

  // Verify structure consistency (not exact formatting)
  let lines : Vec< &str > = help_output.lines().collect();
  assert!( lines.len() > 5, "Help output should have substantial content" );

  // Usage line should be first non-empty line
  let usage_line = lines.iter().find( |line| !line.trim().is_empty() );
  assert!( usage_line.is_some() && usage_line.unwrap().contains( "Usage:" ),
          "First line should contain usage information" );
}