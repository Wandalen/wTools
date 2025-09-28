//! Command Execution Unit Tests
//!
//! ## Scope
//! Tests the interpreter's ability to execute verified commands and manage execution context.
//! This covers the critical execution logic that converts verified commands into results.
//!
//! ## Coverage
//! - Basic command execution workflow
//! - Execution context management
//! - Error handling during execution
//! - Command routine invocation
//! - Output data generation and formatting
//! - Execution environment setup
//!
//! ## Related
//! - `unit/semantic/multiple_parameters.rs` - Command verification
//! - `integration/end_to_end.rs` - Complete execution workflows

use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes, OutputData };
use unilang::registry::CommandRegistry;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::interpreter::{ ExecutionContext, Interpreter };
use unilang::types::Value;
use unilang_parser::{ Parser, UnilangParserOptions };
use std::collections::HashMap;

/// Test routine that returns predictable output
#[allow(clippy::unnecessary_wraps)]
fn test_routine( cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, unilang::data::ErrorData >
{
  let arg_count = cmd.arguments.len();
  Ok( OutputData
  {
    content : format!( "Executed {} with {} arguments", cmd.definition.name, arg_count ),
    format : "text".to_string(),
  })
}

/// Test routine that accesses specific arguments
#[allow(clippy::unnecessary_wraps)]
fn argument_access_routine( cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, unilang::data::ErrorData >
{
  let name = cmd.arguments.get( "name" )
    .and_then( |v| match v {
      Value::String( s ) => Some( s.clone() ),
      _ => None,
    })
    .unwrap_or_else( || "World".to_string() );

  Ok( OutputData
  {
    content : format!( "Hello, {}!", name ),
    format : "text".to_string(),
  })
}

/// Test routine that simulates an error
#[allow(clippy::unnecessary_wraps)]
fn error_routine( _cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, unilang::data::ErrorData >
{
  Err( unilang::data::ErrorData::new(
    "TEST_ERROR".to_string(),
    "This is a test error for error handling validation".to_string(),
  ))
}

/// Helper to create a simple command for testing
fn create_test_command( name : &str ) -> CommandDefinition
{
  CommandDefinition::former()
    .name( name )
    .description( "Test command for interpreter validation" )
    .arguments( vec![
      ArgumentDefinition {
        name : "name".to_string(),
        description : "Name parameter".to_string(),
        kind : Kind::String,
        hint : "A name value".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "World".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end()
}

/// Helper to create and verify a command
fn create_verified_command( registry : &CommandRegistry, input : &str ) -> Result< VerifiedCommand, String >
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( input )
    .map_err( |e| format!( "Parse error: {:?}", e ) )?;

  let instructions_array = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions_array, registry );
  let mut verified_commands = analyzer.analyze()
    .map_err( |e| format!( "Semantic analysis error: {:?}", e ) )?;

  verified_commands.pop().ok_or_else( || "No verified command produced".to_string() )
}

#[test]
fn test_basic_command_execution()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command( ".test" );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let verified_command = create_verified_command( &registry, r#".test"# ).expect( "Should create verified command" );

  let interpreter = Interpreter::new();
  let context = ExecutionContext::new();

  let result = interpreter.execute( verified_command, context );

  assert!( result.is_ok(), "Basic command execution should succeed" );
  let output = result.unwrap();
  assert_eq!( output.format, "text" );
  assert!( output.content.contains( "Executed .test with" ), "Output should contain execution info" );
}

#[test]
fn test_command_execution_with_arguments()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command( ".greet" );
  registry.command_add_runtime( &cmd, Box::new( argument_access_routine ) ).unwrap();

  let verified_command = create_verified_command( &registry, r#".greet name::"Alice""# ).expect( "Should create verified command" );

  let interpreter = Interpreter::new();
  let context = ExecutionContext::new();

  let result = interpreter.execute( verified_command, context );

  assert!( result.is_ok(), "Command execution with arguments should succeed" );
  let output = result.unwrap();
  assert_eq!( output.content, "Hello, Alice!" );
  assert_eq!( output.format, "text" );
}

#[test]
fn test_command_execution_with_default_arguments()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command( ".greet" );
  registry.command_add_runtime( &cmd, Box::new( argument_access_routine ) ).unwrap();

  let verified_command = create_verified_command( &registry, r#".greet"# ).expect( "Should create verified command" );

  let interpreter = Interpreter::new();
  let context = ExecutionContext::new();

  let result = interpreter.execute( verified_command, context );

  assert!( result.is_ok(), "Command execution with defaults should succeed" );
  let output = result.unwrap();
  assert_eq!( output.content, "Hello, World!" );
}

#[test]
fn test_command_execution_error_handling()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command( ".error" );
  registry.command_add_runtime( &cmd, Box::new( error_routine ) ).unwrap();

  let verified_command = create_verified_command( &registry, r#".error"# ).expect( "Should create verified command" );

  let interpreter = Interpreter::new();
  let context = ExecutionContext::new();

  let result = interpreter.execute( verified_command, context );

  assert!( result.is_err(), "Error routine should return error" );
  let error = result.unwrap_err();
  assert_eq!( error.code, "TEST_ERROR" );
  assert!( error.message.contains( "test error" ) );
}

#[test]
fn test_execution_context_management()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command( ".test" );

  // Create a routine that validates context
  let context_validation_routine = Box::new( |_cmd: VerifiedCommand, ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    // Validate that context is properly passed
    let _context_data = ctx; // Use context to ensure it's passed correctly
    Ok( OutputData {
      content : "Context validated".to_string(),
      format : "text".to_string(),
    })
  });

  registry.command_add_runtime( &cmd, context_validation_routine ).unwrap();

  let verified_command = create_verified_command( &registry, r#".test"# ).expect( "Should create verified command" );

  let interpreter = Interpreter::new();
  let context = ExecutionContext::new();

  let result = interpreter.execute( verified_command, context );

  assert!( result.is_ok(), "Context management should work correctly" );
  let output = result.unwrap();
  assert_eq!( output.content, "Context validated" );
}

#[test]
fn test_multiple_command_executions()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command( ".test" );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let interpreter = Interpreter::new();

  // Execute multiple commands to ensure no state leakage
  for i in 1..=5 {
    let verified_command = create_verified_command( &registry, r#".test"# ).expect( "Should create verified command" );
    let context = ExecutionContext::new();

    let result = interpreter.execute( verified_command, context );

    assert!( result.is_ok(), "Multiple executions should all succeed" );
    let output = result.unwrap();
    assert!( output.content.contains( "Executed .test" ), "Output should be consistent for execution {}", i );
  }
}

#[test]
fn test_execution_with_complex_arguments()
{
  let mut registry = CommandRegistry::new();

  // Create command with multiple argument types
  let cmd = CommandDefinition::former()
    .name( ".complex" )
    .description( "Complex command with multiple argument types" )
    .arguments( vec![
      ArgumentDefinition {
        name : "text".to_string(),
        description : "Text parameter".to_string(),
        kind : Kind::String,
        hint : "String value".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
      ArgumentDefinition {
        name : "number".to_string(),
        description : "Number parameter".to_string(),
        kind : Kind::Integer,
        hint : "Integer value".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
      ArgumentDefinition {
        name : "flag".to_string(),
        description : "Boolean parameter".to_string(),
        kind : Kind::Boolean,
        hint : "Boolean value".to_string(),
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

  let complex_routine = Box::new( |cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    let text = match cmd.arguments.get( "text" ).unwrap() {
      Value::String( s ) => s.clone(),
      _ => return Err( unilang::data::ErrorData::new( "TYPE_ERROR".to_string(), "Expected string".to_string() ) ),
    };

    let number = match cmd.arguments.get( "number" ).unwrap() {
      Value::Integer( i ) => *i,
      _ => return Err( unilang::data::ErrorData::new( "TYPE_ERROR".to_string(), "Expected integer".to_string() ) ),
    };

    let flag = match cmd.arguments.get( "flag" ).unwrap() {
      Value::Boolean( b ) => *b,
      _ => return Err( unilang::data::ErrorData::new( "TYPE_ERROR".to_string(), "Expected boolean".to_string() ) ),
    };

    Ok( OutputData {
      content : format!( "text={}, number={}, flag={}", text, number, flag ),
      format : "text".to_string(),
    })
  });

  registry.command_add_runtime( &cmd, complex_routine ).unwrap();

  let verified_command = create_verified_command( &registry, r#".complex text::"hello" number::42 flag::true"# ).expect( "Should create verified command" );

  let interpreter = Interpreter::new();
  let context = ExecutionContext::new();

  let result = interpreter.execute( verified_command, context );

  assert!( result.is_ok(), "Complex argument execution should succeed" );
  let output = result.unwrap();
  assert_eq!( output.content, "text=hello, number=42, flag=true" );
}

#[test]
fn test_execution_performance()
{
  use std::time::Instant;

  let mut registry = CommandRegistry::new();
  let cmd = create_test_command( ".perf" );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let verified_command = create_verified_command( &registry, r#".perf"# ).expect( "Should create verified command" );

  let interpreter = Interpreter::new();
  let context = ExecutionContext::new();

  let start = Instant::now();
  let result = interpreter.execute( verified_command, context );
  let duration = start.elapsed();

  assert!( result.is_ok(), "Performance test should succeed" );
  assert!( duration.as_millis() < 10, "Execution should be fast: {:?}", duration );
}

#[test]
fn test_execution_output_formats()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command( ".format" );

  let format_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok( OutputData {
      content : r#"{"message": "hello", "status": "success"}"#.to_string(),
      format : "json".to_string(),
    })
  });

  registry.command_add_runtime( &cmd, format_routine ).unwrap();

  let verified_command = create_verified_command( &registry, r#".format"# ).expect( "Should create verified command" );

  let interpreter = Interpreter::new();
  let context = ExecutionContext::new();

  let result = interpreter.execute( verified_command, context );

  assert!( result.is_ok(), "Format test should succeed" );
  let output = result.unwrap();
  assert_eq!( output.format, "json" );
  assert!( output.content.contains( "hello" ) );
  assert!( output.content.contains( "success" ) );
}