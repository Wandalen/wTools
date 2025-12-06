//! Test that unilang works when used as an external dependency.
//! This simulates how a real user would import and use unilang.

/// Test that we can use unilang's prelude for common operations.
#[ test ]
fn test_external_usage_with_prelude()
{
  use unilang::prelude::*;
  
  // Create a registry - the most basic operation
    let mut registry = CommandRegistry::new();
  
  // Create a simple command
  let cmd = CommandDefinition::former()
    .name( ".hello" )
    .namespace( String::new() )
    .description( "Says hello".to_string() )
    .end();
  
  // Create a simple routine
  let routine = Box::new( | _cmd, _ctx |
  {
    Ok( OutputData
    {
      content : "Hello, World!".to_string(),
      format : "text".to_string(),
      execution_time_ms : None,
    })
  });
  
  // Register the command
    registry.command_add_runtime( &cmd, routine ).unwrap();
  
  // Use Pipeline API
  let pipeline = Pipeline::new( registry );
  let result = pipeline.process_command_simple( ".hello" );
  
  assert!( result.success );
  assert_eq!( result.outputs[ 0 ].content, "Hello, World!" );
}

/// Test that specific imports work correctly for detailed usage.
#[ test ]
fn test_external_usage_with_specific_imports()
{
  use unilang::
  {
    CommandRegistry,
    CommandDefinition,
    ArgumentDefinition,
    Kind,
    ArgumentAttributes,
    OutputData,
    VerifiedCommand,
    ExecutionContext,
    Pipeline,
  };
  
    let mut registry = CommandRegistry::new();
  
  // Create a command with arguments
  let cmd = CommandDefinition::former()
    .name( ".greet" )
    .namespace( String::new() )
    .description( "Greets someone".to_string() )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "name" )
        .kind( Kind::String )
        .description( "The name to greet".to_string() )
        .attributes( ArgumentAttributes::default() )
        .end()
    ])
    .end();
  
  let routine = Box::new( | cmd : VerifiedCommand, _ctx : ExecutionContext |
  {
    let name = cmd.arguments.get( "name" )
      .and_then( | v | match v { unilang::Value::String( s ) => Some( s.clone() ), _ => None } )
      .unwrap_or_else( || "Anonymous".to_string() );
    
    Ok( OutputData
    {
      content : format!( "Hello, {name}!" ),
      format : "text".to_string(),
      execution_time_ms : None,
    })
  });
  
    registry.command_add_runtime( &cmd, routine ).unwrap();
  
  let pipeline = Pipeline::new( registry );
  let result = pipeline.process_command_simple( ".greet name::\"Alice\"" );
  
  assert!( result.success );
  assert_eq!( result.outputs[ 0 ].content, "Hello, Alice!" );
}

/// Test that module-specific imports work for advanced usage.
#[ test ]
fn test_external_usage_with_module_imports()
{
  // Import from specific modules
  use unilang::registry::CommandRegistry;
  use unilang::data::{ CommandDefinition, OutputData };
  use unilang::pipeline::Pipeline;
  use unilang::semantic::VerifiedCommand;
  use unilang::interpreter::ExecutionContext;
  
    let mut registry = CommandRegistry::new();
  
  let cmd = CommandDefinition::former()
    .name( ".test" )
    .namespace( String::new() )
    .description( "Test command".to_string() )
    .end();
  
  let routine = Box::new( | _cmd : VerifiedCommand, _ctx : ExecutionContext |
  {
    Ok( OutputData
    {
      content : "Test successful".to_string(),
      format : "text".to_string(),
      execution_time_ms : None,
    })
  });
  
    registry.command_add_runtime( &cmd, routine ).unwrap();
  
  let pipeline = Pipeline::new( registry );
  let result = pipeline.process_command_simple( ".test" );
  
  assert!( result.success );
  assert_eq!( result.outputs[ 0 ].content, "Test successful" );
}

/// Test that error handling works correctly in external usage.
#[ test ]
fn test_external_usage_error_handling()
{
  use unilang::prelude::*;
  
    let registry = CommandRegistry::new();
  let pipeline = Pipeline::new( registry );
  
  // Try to execute a non-existent command
  let result = pipeline.process_command_simple( "nonexistent" );
  
  assert!( !result.success );
  assert!( result.error.is_some() );
}

/// Test batch processing functionality.
#[ test ]
fn test_external_usage_batch_processing()
{
  use unilang::prelude::*;
  use unilang::{ VerifiedCommand, ExecutionContext };
  
    let mut registry = CommandRegistry::new();
  
  let cmd = CommandDefinition::former()
    .name( ".echo" )
    .namespace( String::new() )
    .description( "Echo command".to_string() )
    .end();
  
  let routine = Box::new( | _cmd : VerifiedCommand, _ctx : ExecutionContext |
  {
    Ok( OutputData
    {
      content : "echo".to_string(),
      format : "text".to_string(),
      execution_time_ms : None,
    })
  });
  
    registry.command_add_runtime( &cmd, routine ).unwrap();
  
  let pipeline = Pipeline::new( registry );
  let commands = vec![ ".echo", ".echo", ".echo" ];
  let batch_result = pipeline.process_batch( &commands, ExecutionContext::default() );
  
  assert_eq!( batch_result.total_commands, 3 );
  assert_eq!( batch_result.successful_commands, 3 );
  assert_eq!( batch_result.failed_commands, 0 );
  assert!( batch_result.all_succeeded() );
}