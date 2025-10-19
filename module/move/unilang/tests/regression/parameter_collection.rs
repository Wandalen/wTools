//! Parameter Collection Regression Tests
//!
//! ## Scope
//! Prevents regression of the critical parameter collection bug where multiple parameters
//! with the same name would lose all but the first value during semantic analysis.
//! This bug was originally identified and fixed in Task 024.
//!
//! ## Coverage
//! - Exact reproduction of the original Task 024 bug scenario
//! - Verification that the fix continues to work correctly
//! - Edge cases that could cause the bug to reappear
//! - Performance characteristics to prevent degradation
//!
//! ## Related
//! - `unit/semantic/multiple_parameters.rs` - Comprehensive multiple parameter testing
//! - Original bug report: Task 024 Comprehensive Tokenization Failure Analysis

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::registry::CommandRegistry;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::interpreter::ExecutionContext;
use unilang::types::Value;
use unilang_parser::{ Parser, UnilangParserOptions };

/// Mock routine for regression tests
#[allow(clippy::unnecessary_wraps)]
fn regression_test_routine( _cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, unilang::data::ErrorData >
{
  Ok( OutputData
  {
    content : "Commands executed successfully".to_string(),
    format : "text".to_string(),
      execution_time_ms : None,
  })
}

#[test]
fn regression_task_024_exact_scenario_reproduction()
{
  // This test reproduces the EXACT scenario from Task 024 that was failing
  // Input: .run command::"cargo build" command::"echo hello1" command::"cargo clippy" parallel::2
  // Expected: All three commands should be collected, not just the first one

  let mut registry = CommandRegistry::new();

  // Create the exact command definition from the original bug report
  let cmd = CommandDefinition::former()
    .name( ".run" )
    .description( "Run multiple commands" )
    .arguments( vec![
      ArgumentDefinition {
        name : "command".to_string(),
        description : "Command to execute".to_string(),
        kind : Kind::String,
        hint : "Shell command".to_string(),
        attributes : ArgumentAttributes {
          multiple : false, // ❌ This was the problem - should collect multiple anyway
          optional : false,
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

  registry.command_add_runtime( &cmd, Box::new( regression_test_routine ) ).unwrap();

  // Parse the exact command from the original bug report
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = r#".run command::"cargo build" command::"echo hello1" command::"cargo clippy" parallel::2"#;

  let instruction = parser.parse_single_instruction( input ).expect( "Parse should succeed" );

  // Run semantic analysis (this is where the bug was)
  let instructions_array = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions_array, &registry );
  let verified_commands = analyzer.analyze().expect( "Semantic analysis should succeed" );

  assert_eq!( verified_commands.len(), 1, "Should have one verified command" );
  let verified_cmd = &verified_commands[0];

  // CRITICAL: Verify that ALL THREE commands are collected, not just the first
  let command_value = verified_cmd.arguments.get( "command" ).expect( "command argument should exist" );
  match command_value {
    Value::List( list ) => {
      assert_eq!( list.len(), 3, "All three commands should be collected automatically" );

      let commands : Vec< String > = list.iter().map( |v| match v {
        Value::String( s ) => s.clone(),
        _ => panic!( "Expected string in command list" ),
      }).collect();

      assert_eq!( commands[0], "cargo build" );
      assert_eq!( commands[1], "echo hello1" );
      assert_eq!( commands[2], "cargo clippy" );
    },
    Value::String( s ) => {
      panic!( "❌ REGRESSION DETECTED: Got single string '{s}', but should auto-collect multiple values into list" );
    },
    _ => panic!( "❌ REGRESSION DETECTED: Command value is not String or List" ),
  }

  // Verify the parallel parameter works correctly
  let parallel_value = verified_cmd.arguments.get( "parallel" ).expect( "parallel argument should exist" );
  match parallel_value {
    Value::Integer( n ) => {
      assert_eq!( *n, 2, "Parallel argument should be parsed correctly" );
    },
    _ => panic!( "❌ REGRESSION DETECTED: Parallel value is not Integer" ),
  }
}

#[test]
fn regression_task_024_with_multiple_true_still_works()
{
  // Verify that the fix doesn't break the case where multiple=true was already set
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".run_fixed" )
    .description( "Run multiple commands (with multiple=true)" )
    .arguments( vec![
      ArgumentDefinition {
        name : "command".to_string(),
        description : "Commands to execute".to_string(),
        kind : Kind::String,
        hint : "Shell commands".to_string(),
        attributes : ArgumentAttributes {
          multiple : true, // ✅ This was already working
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.command_add_runtime( &cmd, Box::new( regression_test_routine ) ).unwrap();

  let parser = Parser::new( UnilangParserOptions::default() );
  let input = r#".run_fixed command::"cargo build" command::"echo hello1" command::"cargo clippy""#;

  let instruction = parser.parse_single_instruction( input ).expect( "Parse should succeed" );
  let instructions_array = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions_array, &registry );
  let verified_commands = analyzer.analyze().expect( "Semantic analysis should succeed" );

  let verified_cmd = &verified_commands[0];
  let command_value = verified_cmd.arguments.get( "command" ).expect( "command argument should exist" );

  match command_value {
    Value::List( list ) => {
      assert_eq!( list.len(), 3, "multiple=true should still collect all commands" );
    },
    _ => panic!( "❌ REGRESSION DETECTED: multiple=true case broken" ),
  }
}

#[test]
fn regression_parser_still_collects_parameters_correctly()
{
  // Verify that the parser layer correctly identifies multiple parameters
  // The bug was in semantic analysis, but we need to ensure parser is still working
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = r#".test param::"value1" param::"value2" param::"value3""#;

  let instruction = parser.parse_single_instruction( input ).expect( "Parse should succeed" );

  // Verify parser collected all parameters with the same name
  let param_args = instruction.named_arguments.get( "param" ).expect( "param should exist in parsed result" );
  assert_eq!( param_args.len(), 3, "Parser should collect all three param instances" );

  assert_eq!( param_args[0].value, "value1" );
  assert_eq!( param_args[1].value, "value2" );
  assert_eq!( param_args[2].value, "value3" );
}

#[test]
fn regression_performance_no_degradation()
{
  // Ensure the fix doesn't cause performance regression
  use std::time::Instant;

  let mut registry = CommandRegistry::new();
  let cmd = CommandDefinition::former()
    .name( ".perf_test" )
    .description( "Performance regression test" )
    .arguments( vec![
      ArgumentDefinition {
        name : "data".to_string(),
        description : "Data values".to_string(),
        kind : Kind::String,
        hint : "Test data".to_string(),
        attributes : ArgumentAttributes {
          multiple : false, // Test the fix with multiple=false
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.command_add_runtime( &cmd, Box::new( regression_test_routine ) ).unwrap();

  // Create input with 100 parameters to test performance
  let mut input_parts = vec![ ".perf_test".to_string() ];
  for i in 1..=100 {
    input_parts.push( format!( r#"data::"item{i}""# ) );
  }
  let input = input_parts.join( " " );

  let start = Instant::now();
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( &input ).expect( "Parse should succeed" );
  let instructions_array = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions_array, &registry );
  let verified_commands = analyzer.analyze().expect( "Analysis should succeed" );
  let duration = start.elapsed();

  // Performance regression check: should complete within reasonable time
  assert!( duration.as_millis() < 200, "Performance regression detected: took {duration:?} for 100 parameters" );

  // Verify correctness wasn't sacrificed for performance
  let verified_cmd = &verified_commands[0];
  let data_value = verified_cmd.arguments.get( "data" ).expect( "data should exist" );
  match data_value {
    Value::List( list ) => {
      assert_eq!( list.len(), 100, "All 100 parameters should be collected" );
    },
    _ => panic!( "❌ PERFORMANCE REGRESSION: Failed to collect multiple parameters" ),
  }
}

#[test]
fn regression_single_parameter_backward_compatibility()
{
  // Ensure that single parameters are still handled correctly (backward compatibility)
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".single" )
    .description( "Single parameter test" )
    .arguments( vec![
      ArgumentDefinition {
        name : "value".to_string(),
        description : "Single value".to_string(),
        kind : Kind::String,
        hint : "Test value".to_string(),
        attributes : ArgumentAttributes {
          multiple : false,
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.command_add_runtime( &cmd, Box::new( regression_test_routine ) ).unwrap();

  let parser = Parser::new( UnilangParserOptions::default() );
  let input = r#".single value::"single_item""#;

  let instruction = parser.parse_single_instruction( input ).expect( "Parse should succeed" );
  let instructions_array = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions_array, &registry );
  let verified_commands = analyzer.analyze().expect( "Analysis should succeed" );

  let verified_cmd = &verified_commands[0];
  let value_arg = verified_cmd.arguments.get( "value" ).expect( "value should exist" );

  // Single parameter with multiple=false should remain as String (not wrapped in List)
  match value_arg {
    Value::String( s ) => {
      assert_eq!( s, "single_item", "Single parameter should preserve value" );
    },
    Value::List( _ ) => {
      panic!( "❌ BACKWARD COMPATIBILITY BROKEN: Single parameter wrapped in list when it shouldn't be" );
    },
    _ => panic!( "❌ REGRESSION DETECTED: Unexpected value type for single parameter" ),
  }
}

#[test]
fn regression_edge_case_prevention()
{
  // Test edge cases that could potentially reintroduce the bug
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".edge" )
    .description( "Edge case test" )
    .arguments( vec![
      ArgumentDefinition {
        name : "test".to_string(),
        description : "Test parameter".to_string(),
        kind : Kind::String,
        hint : "Test".to_string(),
        attributes : ArgumentAttributes {
          multiple : false,
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "t".to_string() ],
        tags : vec![],
      }
    ])
    .end();

  registry.command_add_runtime( &cmd, Box::new( regression_test_routine ) ).unwrap();

  // Test with mixed aliases and canonical names
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = r#".edge test::"value1" t::"value2" test::"value3""#;

  let instruction = parser.parse_single_instruction( input ).expect( "Parse should succeed" );
  let instructions_array = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions_array, &registry );
  let verified_commands = analyzer.analyze().expect( "Analysis should succeed" );

  let verified_cmd = &verified_commands[0];
  let test_value = verified_cmd.arguments.get( "test" ).expect( "test should exist" );

  match test_value {
    Value::List( list ) => {
      assert_eq!( list.len(), 3, "Should collect parameters across aliases and canonical name" );
      let mut values : Vec< String > = list.iter().map( |v| match v {
        Value::String( s ) => s.clone(),
        _ => panic!( "Expected string in list" ),
      }).collect();
      values.sort(); // Sort to make test deterministic regardless of collection order
      assert_eq!( values, vec![ "value1", "value2", "value3" ] );
    },
    _ => panic!( "❌ EDGE CASE REGRESSION: Failed to collect across aliases" ),
  }
}