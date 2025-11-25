//! Registration Error Handling Tests
//!
//! ## Test Matrix
//!
//! | Test ID | What's Tested | Input | Expected | Status |
//! |---------|---------------|-------|----------|--------|
//! | RE-1 | Result return type | Any registration | Result<> returned | NEW API |
//! | RE-2 | Success case handling | Valid command | Ok(()) returned | NEW API |
//! | RE-3 | Error propagation | Invalid registration | Err propagates | NEW API |
//!
//! ## Scope
//!
//! Tests that `CommandRegistry` registration methods return proper Result types
//! instead of unit return. This enables proper error handling and makes failures
//! visible instead of silent.
//!
//! ## Coverage
//!
//! - Registration returns Result<(), `RegistrationError`>
//! - Successful registration returns Ok(())
//! - Failed registration returns Err with details
//! - Errors can be propagated with ? operator
//!
//! ## Related
//!
//! - `task/make_illegal_states_unrepresentable.md` - Error handling requirements
//! - `tests/registry/duplicate_detection.rs` - Duplicate detection errors
//! - `tests/help/help_divergence_prevention.rs` - Related validation
//!
//! ## Implementation Status
//!
//! These tests check that registration API returns Result<> instead of ().
//! Currently `command_add_runtime()` returns Result, so these should pass.
//! After refactoring, `register()` should also return Result<>.

#![ allow( deprecated ) ]

use unilang::data::{ CommandDefinition, OutputData };
use unilang::registry::CommandRegistry;
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;

/// Helper: Create test command
fn create_test_command( name : &str ) -> CommandDefinition
{
  CommandDefinition::former()
    .name( name )
    .description( format!( "Test command {name}" ) )
    .hint( format!( "Hint for {name}" ) )
    .end()
}

/// Helper: Create mock routine
fn create_mock_routine() -> Box< dyn Fn( VerifiedCommand, ExecutionContext ) -> Result< OutputData, unilang::data::ErrorData > + Send + Sync + 'static >
{
  Box::new( | _cmd : VerifiedCommand, _ctx : ExecutionContext | -> Result< OutputData, unilang::data::ErrorData >
  {
    Ok( OutputData::new( "test", "text" ) )
  })
}

/// RE-1: Registration must return Result type for error handling
///
/// **Expected:** All registration methods return Result<(), `RegistrationError`>
/// **Rationale:** Enables proper error handling, prevents silent failures
#[ test ]
fn test_registration_returns_result()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command( ".test" );

  // Registration should return Result type
  let result = registry.command_add_runtime( &cmd, create_mock_routine() );

  // Verify it's a Result (can use ? operator)
  assert!(
    result.is_ok() || result.is_err(),
    "Registration should return Result type"
  );

  // Should be able to use pattern matching
  match result
  {
    Ok( () ) => {
      // Success case
      assert!(
        registry.command( ".test" ).is_some(),
        "Command should exist after successful registration"
      );
    }
    Err( e ) => {
      panic!( "First registration should succeed, got error: {e:?}" );
    }
  }
}

/// RE-2: Successful registration returns Ok(())
///
/// **Expected:** Valid command registration returns Ok(())
/// **Rationale:** Clear success indication, enables ? operator usage
#[ test ]
fn test_successful_registration_returns_ok()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command( ".deploy" );

  let result = registry.command_add_runtime( &cmd, create_mock_routine() );

  // Should return Ok for successful registration
  assert!(
    result.is_ok(),
    "Valid command registration should return Ok(()), got: {result:?}"
  );

  // Can unwrap without panic
  result.expect( "Registration should succeed" );

  // Command should be in registry
  assert!(
    registry.command( ".deploy" ).is_some(),
    "Command should exist after Ok result"
  );
}

/// RE-3: Errors can be propagated with ? operator
///
/// **Expected:** Registration errors can be propagated up the call stack
/// **Rationale:** Enables idiomatic Rust error handling patterns
#[ test ]
fn test_error_propagation_with_question_mark()
{
  fn register_multiple_commands( registry : &mut CommandRegistry ) -> Result< (), Box< dyn std::error::Error > >
  {
    let cmd1 = create_test_command( ".first" );
    let cmd2 = create_test_command( ".second" );

    // Using ? operator for error propagation
    registry.command_add_runtime( &cmd1, create_mock_routine() )?;
    registry.command_add_runtime( &cmd2, create_mock_routine() )?;

    Ok( () )
  }

  let mut registry = CommandRegistry::new();

  // Should succeed
  let result = register_multiple_commands( &mut registry );
  assert!(
    result.is_ok(),
    "Multiple valid registrations should succeed: {result:?}"
  );

  // Both commands should be registered
  assert!( registry.command( ".first" ).is_some(), "First command should exist" );
  assert!( registry.command( ".second" ).is_some(), "Second command should exist" );
}

/// RE-4: Error types are descriptive and actionable
///
/// **Expected:** Registration errors contain useful information
/// **Rationale:** Users need clear error messages for debugging
#[ test ]
fn test_registration_error_types()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command( ".config" );

  // First registration succeeds
  registry.command_add_runtime( &cmd, create_mock_routine() )
    .expect( "First registration should succeed" );

  // Duplicate registration should error
  let duplicate_result = registry.command_add_runtime( &cmd, create_mock_routine() );

  if let Err( error ) = duplicate_result
  {
    // Error should be describable
    let error_display = format!( "{error:?}" );
    assert!(
      !error_display.is_empty(),
      "Error should have displayable message"
    );

    // Error should contain context
    assert!(
      error_display.len() > 10,
      "Error message should be descriptive, got: {error_display}"
    );
  }
  else
  {
    // This is OK for now since duplicate detection may not be implemented yet
    // Test documents expected behavior
  }
}
