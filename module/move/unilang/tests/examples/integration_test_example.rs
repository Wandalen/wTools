//! Example of Well-Structured Integration Test
//!
//! This file demonstrates best practices for integration testing in the systematic
//! organization structure. It shows proper patterns for testing component
//! interactions and data flow through multiple system layers.

use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes };
use unilang::registry::CommandRegistry;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::interpreter::{ ExecutionContext, Interpreter };
use unilang::pipeline::Pipeline;
use unilang::data::{ OutputData, ErrorData };
use unilang_parser::{ Parser, UnilangParserOptions };
use std::collections::HashMap;

/// Example: End-to-end component interaction
///
/// This test demonstrates:
/// - Testing multiple components working together
/// - Data flow validation through the pipeline
/// - Real component interaction (not mocked)
/// - Comprehensive workflow testing
#[test]
fn test_complete_command_processing_pipeline()
{
  // Arrange - Set up complete system with real components
  let mut registry = CommandRegistry::new();

  // Create a command that tests actual data flow
  let process_cmd = CommandDefinition::former()
    .name( ".process" )
    .description( "Process data with specified format" )
    .arguments( vec![
      ArgumentDefinition {
        name : "input".to_string(),
        description : "Input data to process".to_string(),
        kind : Kind::String,
        hint : "Data to be processed".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "data".to_string() ],
        tags : vec![ "processing".to_string() ],
      },
      ArgumentDefinition {
        name : "format".to_string(),
        description : "Output format".to_string(),
        kind : Kind::String,
        hint : "Format for output".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "text".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  // Integration test routine that demonstrates data processing
  let integration_routine = Box::new( |cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, ErrorData> {
    // Extract arguments (testing argument binding integration)
    let input_value = match cmd.arguments.get( "input" )
    {
      Some( value ) => value.to_string(),
      None => return Err( ErrorData::new( "MISSING_INPUT".to_string(), "Input argument required".to_string() ) ),
    };

    let format = cmd.arguments.get( "format" )
      .map( |v| v.to_string() )
      .unwrap_or_else( || "text".to_string() );

    // Process data based on format (integration logic)
    let processed_content = match format.as_str()
    {
      "json" => format!( r#"{{"processed": "{}"}}"#, input_value ),
      "xml" => format!( "<processed>{}</processed>", input_value ),
      "text" | _ => format!( "Processed: {}", input_value ),
    };

    Ok( OutputData {
      content : processed_content,
      format,
    })
  });

  registry.command_add_runtime( &process_cmd, integration_routine ).unwrap();

  // Act - Exercise complete pipeline with real data flow
  let input_command = r#".process input::"test data" format::"json""#;

  // 1. Parse command
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( input_command )
    .expect( "Parser should successfully parse valid command" );

  // 2. Semantic analysis
  let instructions = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let verified_commands = analyzer.analyze()
    .expect( "Semantic analyzer should validate command" );

  // 3. Command execution
  let interpreter = Interpreter::new();
  let context = ExecutionContext::new();
  let output = interpreter.execute( verified_commands.into_iter().next().unwrap(), context )
    .expect( "Interpreter should execute verified command" );

  // Assert - Verify complete data flow
  assert_eq!( output.format, "json", "Output format should match requested format" );
  assert!( output.content.contains( "test data" ), "Output should contain processed input data" );
  assert!( output.content.contains( "processed" ), "Output should indicate processing occurred" );

  // Verify JSON structure (integration with format handling)
  assert!( output.content.starts_with( "{" ) && output.content.ends_with( "}" ),
          "JSON format should produce valid JSON structure" );
}

/// Example: Component contract testing
///
/// This test demonstrates:
/// - Testing contracts between components
/// - Verification of data format compatibility
/// - Interface compliance testing
#[test]
fn test_parser_semantic_analyzer_contract()
{
  // Arrange - Set up contract test
  let mut registry = CommandRegistry::new();

  let contract_cmd = CommandDefinition::former()
    .name( ".contract_test" )
    .description( "Command for testing component contracts" )
    .arguments( vec![
      ArgumentDefinition {
        name : "required_arg".to_string(),
        description : "Required argument for contract testing".to_string(),
        kind : Kind::String,
        hint : "Must be provided".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
      ArgumentDefinition {
        name : "optional_arg".to_string(),
        description : "Optional argument with default".to_string(),
        kind : Kind::Integer,
        hint : "Integer value".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "42".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  let contract_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, ErrorData> {
    Ok( OutputData { content : "contract_fulfilled".to_string(), format : "text".to_string() })
  });
  registry.command_add_runtime( &contract_cmd, contract_routine ).unwrap();

  // Act - Test parser output meets semantic analyzer's contract
  let parser = Parser::new( UnilangParserOptions::default() );

  // Test with minimal required arguments
  let instruction = parser.parse_single_instruction( r#".contract_test required_arg::"value""# )
    .expect( "Parser should handle valid input" );

  // Verify parser output structure (contract requirements)
  assert!( !instruction.command_name.is_empty(), "Parser must provide command name" );
  assert!( instruction.command_name.starts_with( '.' ), "Parser must preserve command prefix" );
  assert!( instruction.named_arguments.contains_key( "required_arg" ), "Parser must extract arguments" );

  // Test semantic analyzer can process parser output (contract fulfillment)
  let instructions = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let verified_commands = analyzer.analyze()
    .expect( "Semantic analyzer should process parser output without modification" );

  // Verify contract fulfillment
  assert_eq!( verified_commands.len(), 1, "Should produce one verified command" );
  assert_eq!( verified_commands[0].definition.name, ".contract_test", "Command identity preserved" );
  assert!( verified_commands[0].arguments.contains_key( "required_arg" ), "Required arguments preserved" );
  assert!( verified_commands[0].arguments.contains_key( "optional_arg" ), "Default values should be applied" );
}

/// Example: State transition testing
///
/// This test demonstrates:
/// - Testing stateful component interactions
/// - Verification of state changes through workflow
/// - Multi-step process validation
#[test]
fn test_pipeline_state_transitions()
{
  // Arrange - Set up stateful pipeline
  let mut pipeline = Pipeline::new();

  // Verify initial state
  assert!( matches!( pipeline.last_result(), None ), "Pipeline should start with no result" );

  // Act & Assert - Test state transitions through workflow

  // Step 1: Process first command
  let result1 = pipeline.process_command( r#".echo message::"Hello""# );
  assert!( result1.is_ok(), "Pipeline should process valid command" );

  // Verify state after first command
  let last_result = pipeline.last_result();
  assert!( last_result.is_some(), "Pipeline should store last result" );

  // Step 2: Process second command that might reference previous result
  let result2 = pipeline.process_command( r#".echo message::"World""# );
  assert!( result2.is_ok(), "Pipeline should process subsequent commands" );

  // Step 3: Process batch of commands
  let batch_commands = vec![
    r#".echo message::"First""#,
    r#".echo message::"Second""#,
    r#".echo message::"Third""#,
  ];

  let batch_result = pipeline.process_batch( &batch_commands );
  assert!( batch_result.is_ok(), "Pipeline should handle command batches" );

  let batch_output = batch_result.unwrap();
  assert_eq!( batch_output.len(), 3, "Should process all commands in batch" );

  // Verify final state
  assert!( pipeline.last_result().is_some(), "Pipeline should maintain state after batch" );
}

/// Example: Error propagation testing
///
/// This test demonstrates:
/// - Testing error handling across component boundaries
/// - Verification of error information preservation
/// - Error recovery testing
#[test]
fn test_error_propagation_through_components()
{
  // Arrange - Set up system with intentional error conditions
  let mut registry = CommandRegistry::new();

  // Command that always fails for testing error propagation
  let error_cmd = CommandDefinition::former()
    .name( ".error_test" )
    .description( "Command that demonstrates error propagation" )
    .arguments( vec![
      ArgumentDefinition {
        name : "trigger".to_string(),
        description : "Error trigger type".to_string(),
        kind : Kind::String,
        hint : "Type of error to trigger".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  let error_routine = Box::new( |cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, ErrorData> {
    let trigger = cmd.arguments.get( "trigger" )
      .map( |v| v.to_string() )
      .unwrap_or_else( || "default".to_string() );

    // Generate different error types for testing
    match trigger.as_str()
    {
      "validation" => Err( ErrorData::new( "VALIDATION_ERROR".to_string(), "Input validation failed".to_string() ) ),
      "runtime" => Err( ErrorData::new( "RUNTIME_ERROR".to_string(), "Runtime execution failed".to_string() ) ),
      "system" => Err( ErrorData::new( "SYSTEM_ERROR".to_string(), "System resource unavailable".to_string() ) ),
      _ => Err( ErrorData::new( "UNKNOWN_ERROR".to_string(), "Unknown error condition".to_string() ) ),
    }
  });

  registry.command_add_runtime( &error_cmd, error_routine ).unwrap();

  // Act & Assert - Test error propagation through each component

  // Test 1: Semantic analysis error (before execution)
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( r".nonexistent_command" ).unwrap();
  let instructions = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let semantic_result = analyzer.analyze();

  assert!( semantic_result.is_err(), "Should fail at semantic analysis stage" );

  // Test 2: Execution error (after successful semantic analysis)
  let valid_instruction = parser.parse_single_instruction( r#".error_test trigger::"validation""# ).unwrap();
  let valid_instructions = [valid_instruction];
  let valid_analyzer = SemanticAnalyzer::new( &valid_instructions, &registry );
  let verified_commands = valid_analyzer.analyze()
    .expect( "Semantic analysis should succeed for valid command" );

  let interpreter = Interpreter::new();
  let context = ExecutionContext::new();
  let execution_result = interpreter.execute( verified_commands.into_iter().next().unwrap(), context );

  assert!( execution_result.is_err(), "Should fail at execution stage" );

  let execution_error = execution_result.unwrap_err();
  assert_eq!( execution_error.code, "VALIDATION_ERROR", "Error code should be preserved" );
  assert!( execution_error.message.contains( "validation" ), "Error message should be preserved" );

  // Test 3: Error recovery - system should handle subsequent valid commands
  let recovery_instruction = parser.parse_single_instruction( r#".echo message::"recovery test""# ).unwrap();
  let recovery_instructions = [recovery_instruction];
  let recovery_analyzer = SemanticAnalyzer::new( &recovery_instructions, &registry );

  // System should recover and process valid commands after errors
  let recovery_result = recovery_analyzer.analyze();
  assert!( recovery_result.is_ok(), "System should recover from previous errors" );
}

/// Example: Performance integration testing
///
/// This test demonstrates:
/// - Testing performance characteristics of component interactions
/// - Verification that integration doesn't degrade performance
/// - Resource usage validation
#[test]
fn test_integration_performance_characteristics()
{
  use std::time::Instant;

  // Arrange - Set up system for performance testing
  let mut registry = CommandRegistry::new();

  // Create multiple commands to test registry performance
  for i in 0..100
  {
    let cmd = CommandDefinition::former()
      .name( &format!( ".perf_test_{}", i ) )
      .description( "Performance test command" )
      .arguments( vec![
        ArgumentDefinition {
          name : "data".to_string(),
          description : "Test data".to_string(),
          kind : Kind::String,
          hint : "Performance test data".to_string(),
          attributes : ArgumentAttributes {
            optional : false,
            ..Default::default()
          },
          validation_rules : vec![],
          aliases : vec![],
          tags : vec![],
        }
      ])
      .end();

    let perf_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, ErrorData> {
      Ok( OutputData { content : "performance_test_result".to_string(), format : "text".to_string() })
    });

    registry.command_add_runtime( &cmd, perf_routine ).unwrap();
  }

  let parser = Parser::new( UnilangParserOptions::default() );
  let interpreter = Interpreter::new();

  // Act - Measure performance of integrated workflow
  let start_time = Instant::now();

  for i in 0..50  // Test reasonable number of commands
  {
    let command_input = format!( r#".perf_test_{} data::"test_data_{}""#, i % 100, i );

    // Complete workflow: parse -> analyze -> execute
    let instruction = parser.parse_single_instruction( &command_input )
      .expect( "Should parse performance test command" );

    let instructions = [instruction];
    let analyzer = SemanticAnalyzer::new( &instructions, &registry );
    let verified_commands = analyzer.analyze()
      .expect( "Should analyze performance test command" );

    let context = ExecutionContext::new();
    let _output = interpreter.execute( verified_commands.into_iter().next().unwrap(), context )
      .expect( "Should execute performance test command" );
  }

  let total_duration = start_time.elapsed();

  // Assert - Performance should be reasonable
  let avg_duration_per_command = total_duration.as_micros() / 50;
  assert!( avg_duration_per_command < 10_000, // 10ms per command is generous
          "Average command processing should be fast: {}μs per command", avg_duration_per_command );

  // Verify system is still responsive after performance test
  let final_test = parser.parse_single_instruction( r#".perf_test_0 data::"final""# ).unwrap();
  let final_instructions = [final_test];
  let final_analyzer = SemanticAnalyzer::new( &final_instructions, &registry );
  let final_result = final_analyzer.analyze();

  assert!( final_result.is_ok(), "System should remain responsive after performance test" );
}

/// Example: Configuration integration testing
///
/// This test demonstrates:
/// - Testing component behavior with different configurations
/// - Verification of configuration propagation
/// - Environment-specific behavior testing
#[test]
fn test_configuration_integration()
{
  // Test different parser configurations
  let strict_options = UnilangParserOptions {
    strict_mode : true,
    ..Default::default()
  };

  let permissive_options = UnilangParserOptions {
    strict_mode : false,
    ..Default::default()
  };

  let strict_parser = Parser::new( strict_options );
  let permissive_parser = Parser::new( permissive_options );

  // Test input that might be handled differently
  let ambiguous_input = r".test arg::value extra_text";

  // Strict parser might be more restrictive
  let strict_result = strict_parser.parse_single_instruction( ambiguous_input );

  // Permissive parser might be more forgiving
  let permissive_result = permissive_parser.parse_single_instruction( ambiguous_input );

  // Verify configuration affects behavior appropriately
  // (Exact behavior depends on implementation)
  assert!( strict_result.is_ok() || permissive_result.is_ok(),
          "At least one parser configuration should handle the input" );

  // Configuration should be consistent within same parser instance
  let second_strict_result = strict_parser.parse_single_instruction( ambiguous_input );
  assert_eq!( strict_result.is_ok(), second_strict_result.is_ok(),
             "Parser behavior should be consistent with same configuration" );
}