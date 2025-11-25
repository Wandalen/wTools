//! Duplicate Command Detection Tests
//!
//! ## Test Matrix
//!
//! | Test ID | What's Tested | Input | Expected | Actual Bug |
//! |---------|---------------|-------|----------|------------|
//! | DD-1 | Duplicate name detection | Same name twice | Error | SILENT OVERWRITE |
//! | DD-2 | Second registration fails | Register duplicate | `Err()` returned | May succeed |
//! | DD-3 | First registration preserved | Duplicate attempt | Original kept | May be lost |
//! | DD-4 | Error message clarity | Duplicate error | Clear message | No error |
//!
//! ## Scope
//!
//! Tests that `CommandRegistry` detects and prevents duplicate command registration.
//! Currently, registering the same command name twice silently overwrites the first
//! registration with no error or warning.
//!
//! ## Coverage
//!
//! - Duplicate command name detection
//! - Registration error handling (Result return)
//! - Preservation of first registration
//! - Clear error messages for duplicate attempts
//!
//! ## Related
//!
//! - `task/make_illegal_states_unrepresentable.md` - Type safety requirements
//! - `tests/registry/registration_error_handling.rs` - Error handling tests
//! - `tests/help/help_divergence_prevention.rs` - Related registry issues
//!
//! ## Known Issues
//!
//! These tests MAY FAIL initially - they demonstrate the duplicate registration bug:
//! - No duplicate detection in current implementation
//! - `register()` returns () instead of Result<>
//! - Second registration silently overwrites first

#![ allow( deprecated ) ]

use unilang::data::{ CommandDefinition, OutputData };
use unilang::registry::CommandRegistry;
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;

/// Helper: Create test command
fn create_test_command( name : &str, description : &str ) -> CommandDefinition
{
  CommandDefinition::former()
    .name( name )
    .description( description )
    .hint( format!( "Hint for {name}" ) )
    .end()
}

/// Helper: Create mock routine with specific output
fn create_mock_routine_with_output( output : &str ) -> Box< dyn Fn( VerifiedCommand, ExecutionContext ) -> Result< OutputData, unilang::data::ErrorData > + Send + Sync + 'static >
{
  let output_copy = output.to_string();
  Box::new( move | _cmd : VerifiedCommand, _ctx : ExecutionContext | -> Result< OutputData, unilang::data::ErrorData >
  {
    Ok( OutputData::new( &output_copy, "text" ) )
  })
}

/// DD-1: Registry should detect duplicate command names
///
/// **Expected behavior:** Registering same name twice should fail
/// **Actual bug:** Second registration silently overwrites first
#[ test ]
fn test_duplicate_command_name_detection()
{
  let mut registry = CommandRegistry::new();

  // First registration - should succeed
  let cmd1 = create_test_command( ".deploy", "First deploy implementation" );
  let result1 = registry.command_add_runtime( &cmd1, create_mock_routine_with_output( "first" ) );

  assert!(
    result1.is_ok(),
    "First registration should succeed"
  );

  // Second registration with SAME name - should fail
  let cmd2 = create_test_command( ".deploy", "Second deploy implementation" );
  let result2 = registry.command_add_runtime( &cmd2, create_mock_routine_with_output( "second" ) );

  // CRITICAL CHECK: Second registration should fail
  assert!(
    result2.is_err(),
    "DUPLICATE DETECTION BUG: Second registration of '.deploy' should FAIL but succeeded!\n\
     This allows silent overwriting of commands."
  );

  // Verify error mentions the duplicate
  if let Err( ref error ) = result2
  {
    let error_msg = format!( "{error:?}" );
    assert!(
      error_msg.contains( "duplicate" ) || error_msg.contains( "Duplicate" ) || error_msg.contains( "already" ),
      "Error message should mention duplicate: {error_msg}"
    );
  }
}

/// DD-2: Second registration attempt must return error
///
/// **Expected behavior:** `register()` returns Result<(), `RegistrationError`>
/// **Actual bug:** `register()` returns () and silently succeeds
#[ test ]
fn test_duplicate_registration_returns_error()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_test_command( ".test", "Test command" );

  // First registration
  let _first = registry.command_add_runtime( &cmd, create_mock_routine_with_output( "first" ) );

  // Second registration of same command
  let second = registry.command_add_runtime( &cmd, create_mock_routine_with_output( "second" ) );

  // CRITICAL CHECK: Must return Err for duplicate
  assert!(
    second.is_err(),
    "DUPLICATE REGISTRATION BUG: Registering '.test' twice returned Ok()!\n\
     Expected: Err(RegistrationError::DuplicateCommand)\n\
     Actual: Ok(()) - silent overwrite occurred"
  );
}

/// DD-3: First registration must be preserved when duplicate is rejected
///
/// **Expected behavior:** Original command remains in registry after duplicate attempt fails
/// **Actual bug:** Second registration overwrites first
#[ test ]
fn test_first_registration_preserved_on_duplicate()
{
  let mut registry = CommandRegistry::new();

  // Register first version
  let cmd1 = create_test_command( ".build", "Original build command" );
  registry.command_add_runtime( &cmd1, create_mock_routine_with_output( "original" ) )
    .expect( "First registration should succeed" );

  // Verify first version is in registry
  assert!(
    registry.command( ".build" ).is_some(),
    "Command should exist after first registration"
  );

  // Attempt to register duplicate
  let cmd2 = create_test_command( ".build", "Modified build command" );
  let _duplicate_result = registry.command_add_runtime( &cmd2, create_mock_routine_with_output( "modified" ) );

  // CRITICAL CHECK: Original command should still be present
  assert!(
    registry.command( ".build" ).is_some(),
    "Original command should still exist after duplicate rejection"
  );

  // Verify the description matches ORIGINAL, not duplicate
  if let Some( stored_cmd ) = registry.command( ".build" )
  {
    assert!(
      stored_cmd.description().contains( "Original" ),
      "OVERWRITE BUG: Command description changed from 'Original' to '{}'!\n\
       This proves the duplicate registration OVERWROTE the original.",
      stored_cmd.description()
    );
  }
}

/// DD-4: Error message should be clear and actionable
///
/// **Expected behavior:** Error clearly states which command name is duplicated
/// **Actual bug:** No error produced
#[ test ]
fn test_duplicate_error_message_clarity()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_test_command( ".config", "Configuration command" );

  // First registration
  registry.command_add_runtime( &cmd, create_mock_routine_with_output( "first" ) )
    .expect( "First registration should succeed" );

  // Duplicate registration
  let duplicate_result = registry.command_add_runtime( &cmd, create_mock_routine_with_output( "second" ) );

  // Check error message quality
  if let Err( error ) = duplicate_result
  {
    let error_msg = format!( "{error:?}" );

    // Error should mention the command name
    assert!(
      error_msg.contains( ".config" ) || error_msg.contains( "config" ),
      "Error should mention command name '.config': {error_msg}"
    );

    // Error should indicate it's a duplicate
    assert!(
      error_msg.contains( "duplicate" )
        || error_msg.contains( "Duplicate" )
        || error_msg.contains( "already registered" )
        || error_msg.contains( "already exists" ),
      "Error should clearly indicate duplicate: {error_msg}"
    );
  }
  else
  {
    panic!(
      "ERROR MESSAGE BUG: No error produced for duplicate '.config' registration!\n\
       Expected: Err with clear message about duplicate\n\
       Actual: Ok() returned (silent overwrite)"
    );
  }
}
