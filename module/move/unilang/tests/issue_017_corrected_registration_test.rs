//! Test demonstrating the correct way to register dot-prefixed commands
//! 
//! This test shows the solution to issue 017: register commands without dot prefix,
//! let the interpreter add it automatically.

use unilang::{ CommandDefinition, CommandRegistry, Pipeline, ExecutionContext, VerifiedCommand, OutputData, ErrorData };

fn create_test_command_handler(_cmd: VerifiedCommand, _ctx: ExecutionContext) -> Result< OutputData, ErrorData >
{
  let output_data = OutputData { content: "Test command executed successfully".to_string(), format: "text".to_string() };
  Ok( output_data )
}

#[test]
fn test_correct_dot_command_registration()
{
  // NEW APPROACH: Explicit command naming with dot prefixes
  // Commands must be registered with dot prefixes - no implicit magic!
  
  let test_cmd = CommandDefinition
  {
    name : ".test_chat".to_string(), // Explicit dot prefix required!
    namespace : String::new(), // Empty namespace means root level
    description : "Test chat command registered correctly".to_string(),
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
  
  assert!( registration_result.is_ok(), "Command registration should succeed" );
  println!( "✅ Command registered correctly: '{}'", test_cmd.name );

  let pipeline = Pipeline::new( registry );

  // Test that the command can be executed with dot prefix
  let execution_result = pipeline.process_command_simple( ".test_chat" );
  
  println!( "\n=== CORRECTED COMMAND EXECUTION ===" );
  println!( "Command: '.test_chat'" );
  println!( "Success: {}", execution_result.success );
  if let Some(err) = &execution_result.error {
    println!( "Error: {}", err );
  }
  for output in &execution_result.outputs {
    println!( "Output: {}", output.content );
  }
  
  // This should now pass!
  assert!( 
    execution_result.success, 
    "Command '.test_chat' should work when registered as 'test_chat': {}", 
    execution_result.error.as_ref().unwrap_or(&"unknown error".to_string())
  );
}

#[test]
fn test_multiple_corrected_commands()
{
  // Test assistant-style commands registered correctly
  let commands = vec![
    ( ".chat", "Start a multi-agent chat session" ),
    ( ".run", "Run a command" ), 
    ( ".test_version", "Show version information" ),
  ];
  
  let mut registry = CommandRegistry::new();
  
  // Register all commands WITH explicit dot prefix
  for (name, description) in &commands 
  {
    let cmd = CommandDefinition
    {
      name : name.to_string(), // Already has dot prefix!
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
  
  println!( "\n=== CORRECTED ASSISTANT-STYLE COMMANDS TEST ===" );
  
  // Test execution of each command (already has dot prefix)
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
    
    // All should work now!
    assert!( 
      result.success, 
      "Command '{}' should work: {}", 
      name,
      result.error.as_ref().unwrap_or(&"unknown error".to_string())
    );
  }
}

#[test]
fn test_namespaced_commands_work_correctly() 
{
  // Test that namespaced commands still work as expected
  let session_cmd = CommandDefinition
  {
    name : ".list".to_string(), // Command name with dot prefix
    namespace : ".session".to_string(), // Namespace WITH dot prefix
    description : "List available sessions".to_string(),
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
  let result = registry.command_add_runtime( &session_cmd, Box::new( create_test_command_handler ) );
  assert!( result.is_ok(), "Namespaced command registration should succeed" );
  
  let pipeline = Pipeline::new( registry );
  
  // This should become ".session.list" 
  let execution_result = pipeline.process_command_simple( ".session.list" );
  
  println!( "\n=== NAMESPACED COMMAND TEST ===" );
  println!( "Command: '.session.list'" );
  println!( "Success: {}", execution_result.success );
  if let Some(err) = &execution_result.error {
    println!( "Error: {}", err );
  }
  
  assert!( 
    execution_result.success, 
    "Namespaced command '.session.list' should work: {}", 
    execution_result.error.as_ref().unwrap_or(&"unknown error".to_string())
  );
}