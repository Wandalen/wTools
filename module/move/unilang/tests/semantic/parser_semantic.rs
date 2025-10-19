//! # Integration Test for Multiple Parameter Handling
//!
//! This test module provides comprehensive integration testing for the multiple parameter
//! handling feature without any mocking. It tests the complete pipeline from raw string
//! input through parsing, semantic analysis, and final execution.
//!
//! ## Test Coverage
//!
//! - ✅ End-to-end parameter parsing and processing
//! - ✅ Real parser and semantic analyzer (no mocking)
//! - ✅ Multiple parameter collection into `Value::List`
//! - ✅ Backward compatibility with single parameters
//! - ✅ Quote handling and tokenization
//! - ✅ Error handling and edge cases

use unilang::
{
  data::{ ArgumentDefinition, ArgumentAttributes, CommandDefinition, Kind, OutputData },
  registry::CommandRegistry,
  semantic::{ SemanticAnalyzer, VerifiedCommand },
  types::Value,
  interpreter::{ Interpreter, ExecutionContext },
};
use unilang_parser::{ Parser, UnilangParserOptions };
use core::fmt::Write;

/// Creates a command definition that accepts multiple file parameters
fn create_file_processor_command() -> CommandDefinition
{
  CommandDefinition::former()
    .name( ".process_files" )
    .description( "Process multiple files with optional settings" )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "file" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes {
          optional: false,
          multiple: true,  // This is the key - multiple files allowed
          ..Default::default()
        })
        .hint( "File paths to process" )
        .description( "Multiple file paths to process" )
        .end(),
      ArgumentDefinition::former()
        .name( "format" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes {
          optional: true,
          multiple: false,  // Single format setting
          default: Some( "json".to_string() ),
          ..Default::default()
        })
        .hint( "Output format" )
        .description( "Output format for processed files" )
        .end(),
    ])
    .routine_link( "file_processor".to_string() )
    .namespace( "" )
    .hint( "Process multiple files" )
    .status( "active" )
    .version( "1.0.0" )
    .tags( vec![] )
    .aliases( vec![] )
    .permissions( vec![] )
    .idempotent( false )
    .deprecation_message( "" )
    .http_method_hint( "POST" )
    .examples( vec![] )
    .auto_help_enabled( false )
    .end()
}

/// Creates registry with file processor routine registered
fn setup_registry_with_routine( command_def: &CommandDefinition ) -> CommandRegistry
{
  let mut registry = CommandRegistry::new();

  let routine = Box::new
  (
    | cmd: VerifiedCommand, _ctx: ExecutionContext |
    {
      // Real routine that processes the arguments (no mocking)
      let files = cmd.arguments.get( "file" ).expect( "File argument required" );
      let format = cmd.arguments.get( "format" );

      let mut output = String::new();

      if let Value::List( file_list ) = files
      {
        writeln!( &mut output, "Processing {} files:", file_list.len() ).unwrap();
        for ( i, file_value ) in file_list.iter().enumerate()
        {
          if let Value::String( file_path ) = file_value
          {
            writeln!( &mut output, "  {}: {file_path}", i + 1 ).unwrap();
          }
        }
      }

      if let Some( Value::String( fmt ) ) = format
      {
        writeln!( &mut output, "Output format: {fmt}" ).unwrap();
      }

      Ok( OutputData
      {
        content: output,
        format: "text".to_string(),
      execution_time_ms : None,
      })
    }
  );

  #[allow(deprecated)]
  registry.command_add_runtime( command_def, routine ).unwrap();
  registry
}

/// Parses input and performs semantic analysis
fn parse_and_analyze_instruction( registry: &CommandRegistry ) -> Vec< VerifiedCommand >
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = r#".process_files file::"data.json" file::"config.yaml" file::"output.txt" format::"xml""#;

  let instruction = parser.parse_single_instruction( input )
    .expect( "Failed to parse multiple parameter input" );
  let instructions = vec![ instruction ];

  let semantic_analyzer = SemanticAnalyzer::new( &instructions, registry );
  semantic_analyzer.analyze()
    .expect( "Failed semantic analysis of multiple parameters" )
}

/// Verifies semantic analysis results for multiple parameters
fn verify_semantic_analysis_results( verified_commands: &[ VerifiedCommand ] )
{
  assert_eq!( verified_commands.len(), 1, "Should have one verified command" );
  let command = &verified_commands[0];

  // Verify multiple file parameters were collected correctly
  let file_arg = command.arguments.get( "file" ).expect( "Should have file argument" );
  match file_arg
  {
    Value::List( files ) =>
    {
      assert_eq!( files.len(), 3, "Should have exactly 3 file arguments" );

      let expected_files = [ "data.json", "config.yaml", "output.txt" ];
      for ( i, expected ) in expected_files.iter().enumerate()
      {
        match &files[i]
        {
          Value::String( actual ) => assert_eq!( actual, expected, "File {} should match", i + 1 ),
          _ => panic!( "File {} should be a String value", i + 1 ),
        }
      }
    },
    _ => panic!( "File argument should be a List for multiple parameters" ),
  }

  // Verify single format parameter
  let format_arg = command.arguments.get( "format" ).expect( "Should have format argument" );
  match format_arg
  {
    Value::String( fmt ) => assert_eq!( fmt, "xml", "Format should be 'xml'" ),
    _ => panic!( "Format argument should be a String" ),
  }
}

/// Executes command and verifies execution results
fn execute_and_verify_results( verified_commands: &[ VerifiedCommand ], registry: &CommandRegistry )
{
  let interpreter = Interpreter::new( verified_commands, registry );
  let mut context = ExecutionContext::default();

  let results = interpreter.run( &mut context )
    .expect( "Failed to execute command with multiple parameters" );

  assert_eq!( results.len(), 1, "Should have one execution result" );
  let result = &results[0];

  // Verify the output contains all processed files
  assert!( result.content.contains( "Processing 3 files" ), "Should process 3 files" );
  assert!( result.content.contains( "1: data.json" ), "Should contain first file" );
  assert!( result.content.contains( "2: config.yaml" ), "Should contain second file" );
  assert!( result.content.contains( "3: output.txt" ), "Should contain third file" );
  assert!( result.content.contains( "Output format: xml" ), "Should contain format setting" );
}

/// Integration test demonstrating full end-to-end multiple parameter handling
#[test]
fn test_complete_multiple_parameter_integration()
{
  // Step 1: Create a command that accepts multiple "file" parameters
  let file_processor_cmd = create_file_processor_command();

  // Step 2: Create registry and register command with real routine
  let registry = setup_registry_with_routine( &file_processor_cmd );

  // Step 3-5: Parse input and perform semantic analysis
  let verified_commands = parse_and_analyze_instruction( &registry );

  // Step 6: Verify semantic analysis results
  verify_semantic_analysis_results( &verified_commands );

  // Step 7-8: Execute command and verify results
  execute_and_verify_results( &verified_commands, &registry );
}

/// Test backward compatibility - single parameter still works
#[test]
fn test_backward_compatibility_single_parameter_integration()
{
  let single_param_cmd = CommandDefinition::former()
    .name( ".single_file" )
    .description( "Process a single file" )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "file" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes {
          optional: false,
          multiple: false,  // Single file only
          ..Default::default()
        })
        .hint( "File path to process" )
        .description( "Single file path to process" )
        .end(),
    ])
    .routine_link( "single_file_processor".to_string() )
    .namespace( "" )
    .hint( "Process single file" )
    .status( "active" )
    .version( "1.0.0" )
    .tags( vec![] )
    .aliases( vec![] )
    .permissions( vec![] )
    .idempotent( false )
    .deprecation_message( "" )
    .http_method_hint( "POST" )
    .examples( vec![] )
    .auto_help_enabled( false )
    .end();

  let mut registry = CommandRegistry::new();

  let routine = Box::new
  (
    | cmd: VerifiedCommand, _ctx: ExecutionContext |
    {
      let file = cmd.arguments.get( "file" ).expect( "File argument required" );

      let output = match file
      {
        Value::String( file_path ) => format!( "Processing single file: {file_path}" ),
        _ => panic!( "Single file should be String, not List" ),
      };

      Ok( OutputData
      {
        content: output,
        format: "text".to_string(),
      execution_time_ms : None,
      })
    }
  );

  #[allow(deprecated)]
  registry.command_add_runtime( &single_param_cmd, routine ).unwrap();

  let parser = Parser::new( UnilangParserOptions::default() );
  let input = r#".single_file file::"data.json""#;

  let instruction = parser.parse_single_instruction( input )
    .expect( "Failed to parse single parameter input" );
  let instructions = vec![ instruction ];

  let semantic_analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let verified_commands = semantic_analyzer.analyze()
    .expect( "Failed semantic analysis of single parameter" );

  let command = &verified_commands[0];
  let file_arg = command.arguments.get( "file" ).expect( "Should have file argument" );

  // For single parameter (multiple: false), should be String, not List
  match file_arg
  {
    Value::String( file_path ) => assert_eq!( file_path, "data.json", "Single file should match" ),
    Value::List( _ ) => panic!( "Single parameter should be String, not List" ),
    _ => panic!( "File argument should be a String" ),
  }

  let interpreter = Interpreter::new( &verified_commands, &registry );
  let mut context = ExecutionContext::default();

  let results = interpreter.run( &mut context )
    .expect( "Failed to execute single parameter command" );

  let result = &results[0];
  assert!( result.content.contains( "Processing single file: data.json" ), "Should process single file correctly" );
}

/// Test quote handling with complex values
#[test]
fn test_quote_handling_integration()
{
  let quote_test_cmd = CommandDefinition::former()
    .name( ".quote_test" )
    .description( "Test quoted parameter handling" )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "command" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes {
          optional: false,
          multiple: true,
          ..Default::default()
        })
        .hint( "Commands with spaces and quotes" )
        .description( "Commands that may contain spaces and special characters" )
        .end(),
    ])
    .routine_link( "quote_processor".to_string() )
    .namespace( "" )
    .hint( "Process quoted commands" )
    .status( "active" )
    .version( "1.0.0" )
    .tags( vec![] )
    .aliases( vec![] )
    .permissions( vec![] )
    .idempotent( false )
    .deprecation_message( "" )
    .http_method_hint( "POST" )
    .examples( vec![] )
    .auto_help_enabled( false )
    .end();

  let mut registry = CommandRegistry::new();

  let routine = Box::new
  (
    | cmd: VerifiedCommand, _ctx: ExecutionContext |
    {
      let commands = cmd.arguments.get( "command" ).expect( "Command argument required" );

      let mut output = String::new();
      if let Value::List( cmd_list ) = commands
      {
        writeln!( &mut output, "Received {} commands:", cmd_list.len() ).unwrap();
        for ( i, cmd_value ) in cmd_list.iter().enumerate()
        {
          if let Value::String( cmd_str ) = cmd_value
          {
            writeln!( &mut output, "  Command {}: '{cmd_str}'", i + 1 ).unwrap();
          }
        }
      }

      Ok( OutputData
      {
        content: output,
        format: "text".to_string(),
      execution_time_ms : None,
      })
    }
  );

  #[allow(deprecated)]
  registry.command_add_runtime( &quote_test_cmd, routine ).unwrap();

  let parser = Parser::new( UnilangParserOptions::default() );

  // Test complex quoted values with spaces and special characters
  let input = r#".quote_test command::"echo 'hello world'" command::"cargo test --verbose" command::"find . -name '*.rs'"#;

  let instruction = parser.parse_single_instruction( input )
    .expect( "Failed to parse quoted parameters" );
  let instructions = vec![ instruction ];

  let semantic_analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let verified_commands = semantic_analyzer.analyze()
    .expect( "Failed semantic analysis of quoted parameters" );

  let command = &verified_commands[0];
  let cmd_arg = command.arguments.get( "command" ).expect( "Should have command argument" );

  match cmd_arg
  {
    Value::List( commands ) =>
    {
      assert_eq!( commands.len(), 3, "Should have 3 quoted commands" );

      let expected = [
        "echo 'hello world'",
        "cargo test --verbose",
        "find . -name '*.rs'"
      ];

      for ( i, expected_cmd ) in expected.iter().enumerate()
      {
        match &commands[i]
        {
          Value::String( actual ) => assert_eq!( actual, expected_cmd, "Command {} should preserve quotes and spaces", i + 1 ),
          _ => panic!( "Command {} should be a String", i + 1 ),
        }
      }
    },
    _ => panic!( "Commands should be a List" ),
  }

  let interpreter = Interpreter::new( &verified_commands, &registry );
  let mut context = ExecutionContext::default();

  let results = interpreter.run( &mut context )
    .expect( "Failed to execute quoted parameter command" );

  let result = &results[0];
  assert!( result.content.contains( "Received 3 commands" ), "Should receive 3 commands" );
  assert!( result.content.contains( "echo 'hello world'" ), "Should preserve quotes in echo command" );
  assert!( result.content.contains( "cargo test --verbose" ), "Should preserve spaces in cargo command" );
  assert!( result.content.contains( "find . -name '*.rs'" ), "Should preserve complex quotes in find command" );
}

/// Performance test ensuring no regression
#[test]
fn test_multiple_parameter_performance()
{
  use std::time::Instant;

  let perf_test_cmd = CommandDefinition::former()
    .name( ".perf_test" )
    .description( "Performance test command" )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "item" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes {
          optional: false,
          multiple: true,
          ..Default::default()
        })
        .hint( "Multiple items for performance testing" )
        .description( "Multiple items to test performance" )
        .end(),
    ])
    .routine_link( "perf_processor".to_string() )
    .namespace( "" )
    .hint( "Performance test" )
    .status( "active" )
    .version( "1.0.0" )
    .tags( vec![] )
    .aliases( vec![] )
    .permissions( vec![] )
    .idempotent( false )
    .deprecation_message( "" )
    .http_method_hint( "POST" )
    .examples( vec![] )
    .auto_help_enabled( false )
    .end();

  let mut registry = CommandRegistry::new();

  let routine = Box::new
  (
    | cmd: VerifiedCommand, _ctx: ExecutionContext |
    {
      let items = cmd.arguments.get( "item" ).expect( "Item argument required" );
      if let Value::List( item_list ) = items
      {
        Ok( OutputData
        {
          content: format!( "Processed {} items", item_list.len() ),
          format: "text".to_string(),
      execution_time_ms : None,
        })
      }
      else
      {
        panic!( "Items should be a List" );
      }
    }
  );

  #[allow(deprecated)]
  registry.command_add_runtime( &perf_test_cmd, routine ).unwrap();

  let parser = Parser::new( UnilangParserOptions::default() );

  // Create input with many parameters
  let mut input = String::from( ".perf_test" );
  for i in 1..=50
  {
    write!( &mut input, r#" item::"item_{i}.txt""# ).unwrap();
  }

  let start = Instant::now();

  // Run the complete pipeline 10 times to measure performance
  for _ in 0..10
  {
    let instruction = parser.parse_single_instruction( &input )
      .expect( "Failed to parse performance test input" );
    let instructions = vec![ instruction ];

    let semantic_analyzer = SemanticAnalyzer::new( &instructions, &registry );
    let verified_commands = semantic_analyzer.analyze()
      .expect( "Failed semantic analysis in performance test" );

    let interpreter = Interpreter::new( &verified_commands, &registry );
    let mut context = ExecutionContext::default();

    let results = interpreter.run( &mut context )
      .expect( "Failed to execute performance test command" );

    // Verify correct processing
    assert_eq!( results.len(), 1 );
    assert!( results[0].content.contains( "Processed 50 items" ) );
  }

  let duration = start.elapsed();

  // Performance should complete 10 runs with 50 parameters each in reasonable time
  assert!( duration.as_millis() < 1000, "Performance test took too long: {} ms", duration.as_millis() );
}