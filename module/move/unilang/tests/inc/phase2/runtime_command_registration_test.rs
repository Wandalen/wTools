use unilang::data::{ ArgumentDefinition, CommandDefinition, OutputData, ErrorData, Kind };
use unilang::parsing::Parser;
use unilang::registry::{ CommandRegistry, CommandRoutine };
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::interpreter::{ Interpreter, ExecutionContext };
use unilang::error::Error;
// use std::collections::HashMap; // Removed unused import

// --- Test Routines ---

fn test_routine_no_args( _command: VerifiedCommand, _context: ExecutionContext ) -> Result<OutputData, ErrorData>
{
  Ok( OutputData { content: "Routine executed!".to_string(), format: "text".to_string() } )
}

fn test_routine_with_args( command: VerifiedCommand, _context: ExecutionContext ) -> Result<OutputData, ErrorData>
{
  let arg1_value = command.arguments.get( "arg1" ).unwrap().to_string();
  Ok( OutputData { content: format!( "Routine with arg1: {}", arg1_value ), format: "text".to_string() } )
}

fn test_routine_error( _command: VerifiedCommand, _context: ExecutionContext ) -> Result<OutputData, ErrorData>
{
  Err( ErrorData { code: "ROUTINE_ERROR".to_string(), message: "Simulated routine error".to_string() } )
}

// --- Helper Functions ---

fn setup_registry_with_runtime_command( command_name: &str, routine: CommandRoutine, args: Vec<ArgumentDefinition> ) -> CommandRegistry
{
  let mut registry = CommandRegistry::new();
  let command_def = CommandDefinition {
    name: command_name.to_string(),
    description: "A runtime test command".to_string(),
    arguments: args,
    routine_link : Some( format!( "{}_link", command_name ) ),
  };
  registry.command_add_runtime( command_def, routine ).unwrap();
  registry
}

fn analyze_and_run( program_str: &str, registry: &CommandRegistry ) -> Result< Vec< OutputData >, Error >
{
  let program = Parser::new( program_str ).parse();
  let analyzer = SemanticAnalyzer::new( &program, registry );
  let verified_commands = analyzer.analyze()?;
  let interpreter = Interpreter::new( &verified_commands, registry );
  let mut context = ExecutionContext::default();
  interpreter.run( &mut context )
}

// --- Tests ---

#[test]
fn test_runtime_command_registration_success()
{
  // Test Matrix Row: T4.1
  let command_name = ".runtime.test";
  let registry = setup_registry_with_runtime_command( command_name, Box::new( test_routine_no_args ), vec![] );
  assert!( registry.commands.contains_key( command_name ) );
  assert!( registry.get_routine( command_name ).is_some() );
}

#[test]
fn test_runtime_command_execution()
{
  // Test Matrix Row: T4.3
  let command_name = ".runtime.test";
  let registry = setup_registry_with_runtime_command( command_name, Box::new( test_routine_no_args ), vec![] );
  let result = analyze_and_run( command_name, &registry );
  assert!( result.is_ok() );
  assert_eq!( result.unwrap().len(), 1 );
}

#[test]
fn test_runtime_command_with_arguments()
{
  // Test Matrix Row: T4.4
  let command_name = ".runtime.args";
  let args = vec![ArgumentDefinition {
    name: "arg1".to_string(),
    description: "An argument".to_string(),
    kind: Kind::String,
    optional: false,
    multiple: false, // Added
    validation_rules: vec![], // Added
  }];
  let registry = setup_registry_with_runtime_command( command_name, Box::new( test_routine_with_args ), args );
  assert!( registry.commands.contains_key( command_name ) );
  assert!( registry.get_routine( command_name ).is_some() );

  // Test Matrix Row: T4.5
  let result = analyze_and_run( &format!( "{} value1", command_name ), &registry );
  assert!( result.is_ok() );
  let outputs = result.unwrap();
  assert_eq!( outputs.len(), 1 );
  assert_eq!( outputs[0].content, "Routine with arg1: value1" );
}

#[test]
fn test_runtime_command_duplicate_registration()
{
  // Test Matrix Row: T4.2
  let command_name = ".runtime.duplicate";
  let mut registry = CommandRegistry::new();
  let command_def = CommandDefinition {
    name: command_name.to_string(),
    description: "A runtime test command".to_string(),
    arguments: vec![],
    routine_link : Some( format!( "{}_link", command_name ) ),
  };
  
  // First registration (should succeed)
  let result1 = registry.command_add_runtime( command_def.clone(), Box::new( test_routine_no_args ) );
  assert!( result1.is_ok() );

  // Second registration (should also succeed for now, as per registry.rs comment)
  // xxx: Update this test when the registry policy for overwriting is implemented.
  let result2 = registry.command_add_runtime( command_def.clone(), Box::new( test_routine_error ) );
  assert!( result2.is_ok() ); // Currently allows overwrite

  // Verify that the second routine (error routine) is now active
  let result_run = analyze_and_run( command_name, &registry );
  assert!( result_run.is_err() );
  let error = result_run.err().unwrap();
  assert!( matches!( error, Error::Execution( data ) if data.code == "ROUTINE_ERROR" ) );
}

// Test Matrix Row: T4.6 (Optional) - Remove command
// Test Matrix Row: T4.7 (Optional) - Execute removed command
// These tests will be implemented if `command_remove_runtime` is added.