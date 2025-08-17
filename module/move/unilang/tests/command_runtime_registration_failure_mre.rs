//! MRE test for issue 017: Command Runtime Registration Failure

#![ allow( clippy::doc_markdown ) ]
#![ allow( clippy::unnecessary_wraps ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::inefficient_to_string ) ]

use unilang::{ CommandDefinition, CommandRegistry, Pipeline, ExecutionContext, VerifiedCommand, OutputData, ErrorData };

/// MRE test for issue 017: Command Runtime Registration Failure
/// 
/// This test reproduces the exact issue described in task/017_command_runtime_registration_failure.md:
/// - Commands register successfully 
/// - Commands appear in help/discovery
/// - Command execution fails with "No executable routine found"
/// - Error shows command name without dot prefix (e.g. "chat" instead of ".chat")
fn create_test_command_handler(_cmd: VerifiedCommand, _ctx: ExecutionContext) -> Result< OutputData, ErrorData >
{
  let output_data = OutputData { content: "Test command executed successfully".to_string(), format: "text".to_string() };
  Ok( output_data )
}

#[test]
fn test_dot_prefixed_command_runtime_execution()
{
  // Step 1: Create command with dot prefix (mimicking assistant.rs behavior)
  let test_cmd = CommandDefinition
  {
    name : ".test_chat".to_string(), // Dot-prefixed name like ".chat" in assistant
    namespace : String::new(),
    description : "Test chat command for reproducing issue 017".to_string(),
    routine_link : None, // Runtime registration, not static
    arguments : Vec::new(),
    hint : String::new(),
    status : String::new(),
    version : String::new(),
    tags : Vec::new(),
    aliases : Vec::new(),
    permissions : Vec::new(),
    idempotent : false,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : Vec::new(),
  };

  // Step 2: Register command with runtime handler
  let mut registry = CommandRegistry::new();
  let registration_result = registry.command_add_runtime( &test_cmd, Box::new( create_test_command_handler ) );
  
  // Verify registration succeeded 
  assert!( registration_result.is_ok(), "Command registration should succeed" );
  println!( "✅ Command registration succeeded for: '{}'", test_cmd.name );

  // Step 3: Create pipeline for command processing
  let pipeline = Pipeline::new( registry );

  // Step 4: Verify command discovery works (this should pass)
  // This mimics the working part: `assistant .` shows commands
  let discovery_result = pipeline.process_command_simple( "." );
  println!( "Discovery result: success = {}", discovery_result.success );
  if !discovery_result.success
  {
    if let Some(err) = &discovery_result.error {
      println!( "Discovery error: {}", err );
    }
  }

  // Step 5: Verify command help works (this should pass)  
  // This mimics the working part: `assistant .chat ?` shows help
  let help_command = format!( "{} ?", test_cmd.name );
  let help_result = pipeline.process_command_simple( &help_command );
  println!( "Help result for '{}': success = {}", help_command, help_result.success );
  if !help_result.success
  {
    if let Some(err) = &help_result.error {
      println!( "Help error: {}", err );
    }
  }

  // Step 6: THIS IS WHERE THE BUG REPRODUCES
  // Execute the actual command - this should succeed but will fail with:
  // "No executable routine found for command 'test_chat'" (note: no dot prefix)
  let execution_result = pipeline.process_command_simple( &test_cmd.name );
  
  println!( "\n=== CRITICAL TEST: Command Execution ===" );
  println!( "Command: '{}'", test_cmd.name );
  println!( "Success: {}", execution_result.success );
  if let Some(err) = &execution_result.error {
    println!( "Error: {}", err );
  }
  for output in &execution_result.outputs {
    println!( "Output: {}", output.content );
  }
  
  // This assertion SHOULD pass but will fail due to the bug
  // When it fails, we've successfully reproduced issue 017
  assert!( 
    execution_result.success, 
    "BUG REPRODUCED: Command '{}' failed with: {}", 
    test_cmd.name, 
    execution_result.error.as_ref().unwrap_or(&"unknown error".to_string())
  );
}

#[test] 
fn test_non_dot_command_properly_rejected()
{
  // NEW BEHAVIOR: Verify that non-dot commands are properly rejected by validation
  let test_cmd = CommandDefinition
  {
    name : "test_no_dot".to_string(), // NO dot prefix - should be rejected
    namespace : String::new(),
    description : "Test command without dot prefix".to_string(),
    routine_link : None,
    arguments : Vec::new(),
    hint : String::new(),
    status : String::new(),
    version : String::new(),
    tags : Vec::new(),
    aliases : Vec::new(),
    permissions : Vec::new(),
    idempotent : false,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : Vec::new(),
  };

  let mut registry = CommandRegistry::new();
  let registration_result = registry.command_add_runtime( &test_cmd, Box::new( create_test_command_handler ) );
  
  println!( "\n=== VALIDATION TEST: Non-dot Command Rejection ===" );
  println!( "Command: '{}'", test_cmd.name );
  println!( "Registration succeeded: {}", registration_result.is_ok() );
  
  if let Err(e) = &registration_result {
    println!( "Error (expected): {:?}", e );
  }
  
  // With new validation, non-dot commands should be REJECTED
  assert!( 
    registration_result.is_err(), 
    "Non-dot command '{}' should be rejected by validation, but registration succeeded", 
    test_cmd.name
  );
  
  // Verify error message is helpful
  let error_str = format!("{:?}", registration_result.unwrap_err());
  assert!(error_str.contains("must start with dot prefix"), 
         "Error should mention dot prefix requirement: {}", error_str);
         
  println!( "✅ Non-dot command properly rejected with clear error message" );
}

#[test]
fn test_assistant_style_commands()
{
  // Test multiple commands similar to what assistant.rs registers
  let commands = vec![
    ( ".test_chat", "Start a multi-agent chat session" ),
    ( ".test_run", "Run a test command" ),
    ( ".test_session_list", "List available sessions" ),
  ];
  
  let mut registry = CommandRegistry::new();
  
  // Register all commands
  for (name, description) in &commands 
  {
    let cmd = CommandDefinition
    {
      name : name.to_string(),
      namespace : String::new(),
      description : description.to_string(),
      routine_link : None,
      arguments : Vec::new(),
      hint : String::new(),
      status : String::new(),
      version : String::new(),
      tags : Vec::new(),
      aliases : Vec::new(),
      permissions : Vec::new(),
      idempotent : false,
      deprecation_message : String::new(),
      http_method_hint : String::new(),
      examples : Vec::new(),
    };
    
    let result = registry.command_add_runtime( &cmd, Box::new( create_test_command_handler ) );
    assert!( result.is_ok(), "Failed to register command '{}'", name );
    println!( "✅ Registered: '{}'", name );
  }
  
  let pipeline = Pipeline::new( registry );
  
  println!( "\n=== ASSISTANT-STYLE COMMANDS TEST ===" );
  
  // Test execution of each command
  for (name, _) in &commands 
  {
    let result = pipeline.process_command_simple( name );
    println!( "Command '{}': success = {}", name, result.success );
    if let Some(err) = &result.error {
      println!( "  Error: {}", err );
    }
    for output in &result.outputs {
      println!( "  Output: {}", output.content );
    }
    
    // This will show us which specific commands fail and with what error messages
    assert!( 
      result.success, 
      "Assistant-style command '{}' failed: {}", 
      name, 
      result.error.as_ref().unwrap_or(&"unknown error".to_string())
    );
  }
}