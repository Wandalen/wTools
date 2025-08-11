//! ## Test Matrix for `CommandRegistry` Key Mismatch Debugging
//!
//! This test file is created as part of a focused debugging increment to diagnose
//! why commands are not being found in the `CommandRegistry` despite seemingly
//! correct registration and lookup. It will explicitly test the registration
//! and retrieval of commands using fully qualified names, including debug prints
//! of string keys and their byte representations.
//!
//! | ID | Test Case | Expected Behavior | Debug Output |
//! |---|---|---|---|
//! | T-REG-1 | Register and retrieve command with namespace | Command should be found using its fully qualified name. | Print registered key and lookup key with byte representations. |

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind };
use unilang::registry::CommandRegistry;

/// Tests that a command with a namespace can be registered and retrieved using its fully qualified name.
/// Test Combination: T-REG-1
#[ test ]
fn test_command_registry_key_mismatch()
{
  let mut registry = CommandRegistry::new();

  let command_def = CommandDefinition::former()
  .name( ".my_command" )
  .namespace( ".my_namespace" )
  .hint( "A test command." )
  .description( "This is a test command for debugging registry issues." )
  .status( "experimental" )
  .version( "0.1.0" )
  .tags( vec![ "test".to_string() ] )
  .aliases( vec![ "mc".to_string() ] )
  .permissions( vec![ "debug".to_string() ] )
  .idempotent( false )
  .arguments( vec!
  [
    ArgumentDefinition::former()
    .name( "arg1" )
    .hint( "A test argument." )
    .kind( Kind::String )
    .attributes( ArgumentAttributes::default() )
    .form()
  ])
  .form();

  // Register the command and a dummy routine
  registry
  .command_add_runtime
  (
    &command_def,
    Box::new( | _, _ |
    {
      Ok( unilang::data::OutputData
      {
        content : "Dummy routine executed".to_string(),
        format : "text".to_string(),
      })
    }),
  )
  .expect( "Failed to register command with dummy routine" );

  // Attempt to retrieve the command using the fully qualified name
  let lookup_key = if command_def.namespace.is_empty() {
    command_def.name.clone() // Name already has dot prefix
  } else {
    let ns = &command_def.namespace;
    let name_without_dot = command_def.name.strip_prefix('.').unwrap_or(&command_def.name);
    if ns.starts_with( '.' )
    {
      format!( "{}.{}", ns, name_without_dot )
    }
    else
    {
      format!( ".{}.{}", ns, name_without_dot )
    }
  };
  println!( "DEBUG: Lookup key: '{}' (bytes: {:?})", lookup_key, lookup_key.as_bytes() );

  let retrieved_command = registry.command( &lookup_key );

  // Assert that the command is found
  assert!
  (
    retrieved_command.is_some(),
    "Command '{lookup_key}' was not found in the registry."
  );
  assert_eq!( retrieved_command.unwrap().name, command_def.name );

  // Also check the routine map
  let retrieved_routine = registry.get_routine( &lookup_key );
  assert!
  (
    retrieved_routine.is_some(),
    "Routine for command '{lookup_key}' was not found in the registry."
  );
}
