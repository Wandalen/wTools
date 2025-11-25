//! Tests for API consistency improvements to `CommandResult` and error handling.
//!
//! This module tests the implementation of task 019, which improves API consistency
//! by adding helper methods to `CommandResult` and structured error types.

#![ allow( clippy::doc_markdown ) ]

use unilang::{ CommandResult, UnilangError, OutputData };

#[test]
fn test_command_result_is_success()
{
  // Test successful command
  let success_result = CommandResult
  {
    command: ".test".to_string(),
    outputs: vec![ OutputData { content: "success".to_string(), format: "text".to_string(), execution_time_ms: None } ],
    success: true,
    error: None,
  };
  
  assert!( success_result.is_success() );
  assert!( !success_result.is_error() );
  
  // Test failed command with error message
  let error_result = CommandResult
  {
    command: ".test".to_string(),
    outputs: vec![],
    success: false,
    error: Some( "Command failed".to_string() ),
  };
  
  assert!( !error_result.is_success() );
  assert!( error_result.is_error() );
  
  // Test inconsistent state (success=true but error present) - should be considered error
  let inconsistent_result = CommandResult
  {
    command: ".test".to_string(),
    outputs: vec![],
    success: true,
    error: Some( "Error present".to_string() ),
  };
  
  assert!( !inconsistent_result.is_success() );
  assert!( inconsistent_result.is_error() );
}

#[test]
fn test_command_result_error_message()
{
  let result_with_error = CommandResult
  {
    command: ".test".to_string(),
    outputs: vec![],
    success: false,
    error: Some( "Test error message".to_string() ),
  };
  
  assert_eq!( result_with_error.error_message(), Some( "Test error message" ) );
  
  let result_no_error = CommandResult
  {
    command: ".test".to_string(),
    outputs: vec![],
    success: true,
    error: None,
  };
  
  assert_eq!( result_no_error.error_message(), None );
}

#[test]
fn test_command_result_outputs_or_empty()
{
  let success_result = CommandResult
  {
    command: ".test".to_string(),
    outputs: vec![ 
      OutputData { content: "output1".to_string(), format: "text".to_string(), execution_time_ms: None },
      OutputData { content: "output2".to_string(), format: "text".to_string(), execution_time_ms: None },
    ],
    success: true,
    error: None,
  };
  
  let outputs = success_result.outputs_or_empty();
  assert_eq!( outputs.len(), 2 );
  assert_eq!( outputs[0].content, "output1" );
  assert_eq!( outputs[1].content, "output2" );
  
  let error_result = CommandResult
  {
    command: ".test".to_string(),
    outputs: vec![ OutputData { content: "should_not_see".to_string(), format: "text".to_string(), execution_time_ms: None } ],
    success: false,
    error: Some( "Error occurred".to_string() ),
  };
  
  let empty_outputs = error_result.outputs_or_empty();
  assert_eq!( empty_outputs.len(), 0 );
}

#[test]
fn test_interactive_argument_error_parsing()
{
  let interactive_error = CommandResult
  {
    command: ".secure_command".to_string(),
    outputs: vec![],
    success: false,
    error: Some( "Execution Error: Interactive Argument Required: The argument 'password' is marked as interactive and must be provided interactively. The application should prompt the user for this value.".to_string() ),
  };
  
  
  assert!( interactive_error.requires_interactive_input() );
  assert_eq!( interactive_error.interactive_argument(), Some( "password".to_string() ) );
  
  let error_type = interactive_error.error_type();
  assert!( matches!( error_type, Some( UnilangError::InteractiveArgumentRequired { argument, command } )
    if argument == "password" && command == ".secure_command" ) );
}

#[test]
fn test_help_request_error_parsing()
{
  let help_error = CommandResult
  {
    command: ".".to_string(),
    outputs: vec![],
    success: false,
    error: Some( "Execution error: Available commands:\n  .test - Test command\n  .help - Help command".to_string() ),
  };
  
  
  assert!( help_error.is_help_response() );
  assert!( !help_error.requires_interactive_input() );
  
  let error_type = help_error.error_type();
  assert!( matches!( error_type, Some( UnilangError::HelpRequest { .. } ) ) );
  
  let help_content = help_error.help_content();
  assert!( help_content.is_some() );
  assert!( help_content.unwrap().contains( "Available commands:" ) );
}

#[test]
fn test_static_command_error_parsing()
{
  let static_error = CommandResult
  {
    command: ".version".to_string(),
    outputs: vec![],
    success: false,
    error: Some( "The .version command is a static command without an executable routine".to_string() ),
  };
  
  let error_type = static_error.error_type();
  assert!( matches!( error_type, Some( UnilangError::StaticCommandNoRoutine { command } )
    if command == ".version" ) );
}

#[test]
fn test_command_not_found_error_parsing()
{
  let not_found_error = CommandResult
  {
    command: ".unknown".to_string(),
    outputs: vec![],
    success: false,
    error: Some( "Command not found: .unknown. Did you mean: .test, .help".to_string() ),
  };
  
  let error_type = not_found_error.error_type();
  assert!( matches!( error_type, Some( UnilangError::CommandNotFound { command, suggestions } )
    if command == ".unknown" && !suggestions.is_empty() ) );
}

#[test]
fn test_parse_error_parsing()
{
  let parse_error = CommandResult
  {
    command: "invalid..syntax".to_string(),
    outputs: vec![],
    success: false,
    error: Some( "Parse error: Invalid command syntax".to_string() ),
  };
  
  let error_type = parse_error.error_type();
  assert!( matches!( error_type, Some( UnilangError::ParseError { .. } ) ) );
}

#[test]
fn test_semantic_error_parsing()
{
  let semantic_error = CommandResult
  {
    command: ".test invalid_arg".to_string(),
    outputs: vec![],
    success: false,
    error: Some( "Semantic analysis error: Invalid argument type".to_string() ),
  };
  
  let error_type = semantic_error.error_type();
  assert!( matches!( error_type, Some( UnilangError::SemanticError { .. } ) ) );
}

#[test]
fn test_execution_error_parsing()
{
  let execution_error = CommandResult
  {
    command: ".test".to_string(),
    outputs: vec![],
    success: false,
    error: Some( "Execution error: Command execution failed".to_string() ),
  };
  
  let error_type = execution_error.error_type();
  assert!( matches!( error_type, Some( UnilangError::ExecutionFailure { .. } ) ) );
}

#[test]
fn test_other_error_parsing()
{
  let unknown_error = CommandResult
  {
    command: ".test".to_string(),
    outputs: vec![],
    success: false,
    error: Some( "Some unexpected error format".to_string() ),
  };
  
  let error_type = unknown_error.error_type();
  assert!( matches!( error_type, Some( UnilangError::Other { .. } ) ) );
}

#[test]
fn test_successful_command_has_no_error_type()
{
  let success_result = CommandResult
  {
    command: ".test".to_string(),
    outputs: vec![ OutputData { content: "success".to_string(), format: "text".to_string(), execution_time_ms: None } ],
    success: true,
    error: None,
  };
  
  assert_eq!( success_result.error_type(), None );
  assert!( !success_result.requires_interactive_input() );
  assert!( !success_result.is_help_response() );
  assert_eq!( success_result.interactive_argument(), None );
  assert_eq!( success_result.help_content(), None );
}

#[test]
fn test_unilang_error_equality()
{
  let error1 = UnilangError::InteractiveArgumentRequired 
  { 
    argument: "password".to_string(), 
    command: ".login".to_string() 
  };
  
  let error2 = UnilangError::InteractiveArgumentRequired 
  { 
    argument: "password".to_string(), 
    command: ".login".to_string() 
  };
  
  let error3 = UnilangError::InteractiveArgumentRequired 
  { 
    argument: "username".to_string(), 
    command: ".login".to_string() 
  };
  
  assert_eq!( error1, error2 );
  assert_ne!( error1, error3 );
}

#[test]
fn test_api_consistency_example_usage()
{
  // This test demonstrates the improved API usage as shown in the task specification
  let result = CommandResult
  {
    command: ".secure_login".to_string(),
    outputs: vec![],
    success: false,
    error: Some( "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED: Interactive Argument Required: password for command .secure_login".to_string() ),
  };
  
  // The new API allows clean, type-safe error handling
  match result.error_type()
  {
    None => 
    {
      // Command succeeded
      for output in result.outputs_or_empty()
      {
        println!( "Success: {}", output.content );
      }
    }
    Some( UnilangError::InteractiveArgumentRequired { argument, command } ) => 
    {
      // Handle interactive input requirement
      assert_eq!( argument, "password" );
      assert_eq!( command, ".secure_login" );
      println!( "Need interactive input for {argument} in command {command}" );
    }
    Some( UnilangError::HelpRequest { .. } ) => 
    {
      println!( "{}", result.help_content().unwrap_or( "Help not available".to_string() ) );
    }
    Some( UnilangError::CommandNotFound { command, suggestions } ) => 
    {
      println!( "Command '{command}' not found." );
      if !suggestions.is_empty()
      {
        println!( "Did you mean: {}", suggestions.join( ", " ) );
      }
    }
    Some( error ) => 
    {
      println!( "Error: {}", result.error_message().unwrap_or( "Unknown error" ) );
      println!( "Error type: {error:?}" );
    }
  }
  
  // Convenience methods work as expected
  assert!( result.requires_interactive_input() );
  assert!( !result.is_help_response() );
  assert_eq!( result.interactive_argument(), Some( "password".to_string() ) );
}