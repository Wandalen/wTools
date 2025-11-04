//! Validation Enforcement Tests (Phase 1 → Phase 2 Evolution)
//!
//! ## Architecture Evolution
//!
//! **Phase 1:** Validation at registration time (both `register()` and `command_add_runtime()`)
//! - Fixed code path divergence where validation rules differed between registration methods
//! - Prevented invalid commands from being registered
//!
//! **Phase 2:** Validation at construction time (fail-fast)
//! - Moved validation earlier in lifecycle: `CommandDefinition::former().name()` panics immediately
//! - Invalid commands cannot be constructed, making registration-time validation redundant
//! - Tests adapted to use `#[should_panic]` pattern since construction panics replace registration errors
//!
//! ## Test Matrix
//!
//! | Test ID | What's Tested | Input | Expected | Phase 2 Behavior |
//! |---------|---------------|-------|----------|------------------|
//! | VAL-1 | Missing dot prefix | "build" | Panic | `#[should_panic]` test |
//! | VAL-2 | Invalid namespace | "ns" | Error | Still at registration |
//! | VAL-3 | Duplicate registration | Same name twice | Error | Still at registration |
//! | VAL-4 | Valid command | ".build" | Success | Unchanged |
//! | VAL-5 | Path consistency | Both paths panic | Identical | At construction |

#![ allow( clippy::uninlined_format_args ) ]
//!
//! ## Design Rationale
//!
//! Phase 2's fail-fast validation prevents invalid state from ever existing:
//! - **Earlier Detection:** Bugs caught at construction, not registration
//! - **Simpler Code:** No need for duplicate validation in multiple registration paths
//! - **Type Safety:** Invalid commands cannot be constructed, even temporarily
//! - **Better DX:** Panics provide clear stack traces pointing to exact error location
//!
//! ## Testing Pattern
//!
//! Tests validating construction-time failures use `#[should_panic(expected = "...")]`:
//! ```ignore
//! #[test]
//! #[should_panic(expected = "MissingDotPrefix")]
//! fn test_invalid_construction()
//! {
//!   let _cmd = CommandDefinition::former()
//!     .name( "invalid" )  // ❌ Panics here
//!     .end();
//! }
//! ```
//!
//! Tests validating registration-time failures (e.g., duplicates, invalid namespaces) use
//! traditional `assert!(result.is_err())` pattern.

#![ allow( deprecated ) ]

use unilang::data::CommandDefinition;
use unilang::registry::CommandRegistry;
use unilang::interpreter::ExecutionContext;
use unilang::data::{ OutputData, ErrorData };
use unilang::semantic::VerifiedCommand;

/// Helper: Create mock routine for testing
fn create_mock_routine() -> Box< dyn Fn( VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData > + Send + Sync + 'static >
{
  Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result< OutputData, ErrorData >
  {
    Ok( OutputData::new( "test", "text" ) )
  })
}

//
// VAL-1: Commands Without Dot Prefix Must Be Rejected
//

/// Test that construction rejects commands without dot prefix (Phase 2 fail-fast)
///
/// **Phase 2 Update:** Validation moved from registration to construction time.
/// Invalid names now panic during `CommandDefinition::former().name()` call.
#[test]
#[should_panic(expected = "MissingDotPrefix")]
fn test_register_rejects_command_without_dot_prefix()
{
  // Phase 2: This panics at construction time, before registration
  let _invalid_cmd = CommandDefinition::former()
    .name( "build" )  // ❌ No dot prefix - panics here
    .description( "Build project" )
    .end();
}

/// Test that construction rejects commands without dot prefix (Phase 2 fail-fast)
///
/// **Phase 2 Update:** Validation moved to construction time for both paths.
#[test]
#[should_panic(expected = "MissingDotPrefix")]
fn test_command_add_runtime_rejects_command_without_dot_prefix()
{
  // Phase 2: This panics at construction time, before `command_add_runtime()` call
  let _invalid_cmd = CommandDefinition::former()
    .name( "build" )  // ❌ No dot prefix - panics here
    .description( "Build project" )
    .end();
}

//
// VAL-2: Commands With Invalid Namespaces Must Be Rejected
//

/// Test that `register()` rejects commands with invalid namespaces
///
/// **Bug Fixed:** Phase 1.1 - Code path divergence closed
#[test]
fn test_register_rejects_invalid_namespace()
{
  let mut registry = CommandRegistry::new();

  let mut invalid_cmd = CommandDefinition::former()
    .name( ".build" )
    .description( "Build project" )
    .end();

  // Non-empty namespace without dot prefix is invalid
  invalid_cmd.namespace = "admin".to_string();

  let result = registry.register( invalid_cmd );

  assert!(
    result.is_err(),
    "register() should reject invalid namespaces"
  );

  let err_msg = result.unwrap_err().to_string();
  assert!(
    err_msg.contains( "namespace" ) && err_msg.contains( "dot prefix" ),
    "Error message should explain namespace rule"
  );
}

//
// VAL-3: Duplicate Registration Must Be Rejected
//

/// Test that `register()` rejects duplicate command names
///
/// **Bug Fixed:** Phase 1.2 - Previously silently overwrote first command
#[test]
fn test_register_rejects_duplicate_commands()
{
  let mut registry = CommandRegistry::new();

  let cmd1 = CommandDefinition::former()
    .name( ".build" )
    .description( "First build command" )
    .end();

  let cmd2 = CommandDefinition::former()
    .name( ".build" )  // ← Same name
    .description( "Second build command" )
    .end();

  // First registration should succeed
  registry.register( cmd1 ).expect( "First registration should succeed" );

  // Second registration should fail
  let result = registry.register( cmd2 );

  assert!(
    result.is_err(),
    "register() should reject duplicate command names"
  );

  let err_msg = result.unwrap_err().to_string();
  assert!(
    err_msg.contains( "already registered" ),
    "Error message should mention duplicate: {}",
    err_msg
  );
}

/// Test that duplicate rejection preserves first command
///
/// **Verifies:** Silent overwrite bug is fixed
#[test]
fn test_duplicate_rejection_preserves_first_command()
{
  let mut registry = CommandRegistry::new();

  let cmd1 = CommandDefinition::former()
    .name( ".build" )
    .description( "First build command" )
    .end();

  let cmd2 = CommandDefinition::former()
    .name( ".build" )
    .description( "Second build command - should not replace first" )
    .end();

  registry.register( cmd1 ).expect( "First registration should succeed" );

  // Try to register duplicate (should fail)
  let _result = registry.register( cmd2 );

  // Verify first command is still there and unchanged
  let registered_cmd = registry.command( ".build" ).expect( "First command should still exist" );

  assert_eq!(
    registered_cmd.description(),
    "First build command",
    "First command should be preserved (not overwritten)"
  );
}

//
// VAL-4: Valid Commands Should Succeed
//

/// Test that `register()` accepts valid commands
///
/// **Baseline:** Verify normal operation still works
#[test]
fn test_register_accepts_valid_command()
{
  let mut registry = CommandRegistry::new();

  let valid_cmd = CommandDefinition::former()
    .name( ".build" )
    .description( "Build project" )
    .end();

  let result = registry.register( valid_cmd );

  assert!(
    result.is_ok(),
    "register() should accept valid commands"
  );

  // Verify command is accessible
  assert!(
    registry.command( ".build" ).is_some(),
    "Registered command should be accessible"
  );
}

//
// VAL-5: Path Consistency - Both Paths Reject Identically
//

/// Test that both registration paths enforce identical validation
///
/// **Bug Fixed:** Phase 1.1 - Code path divergence eliminated
#[test]
#[should_panic(expected = "MissingDotPrefix")]
fn test_both_paths_reject_invalid_commands_identically()
{
  // Phase 2 Update: Validation moved to construction time, so both paths
  // now reject at the same point (during `CommandDefinition::former().name()`)
  let _invalid_cmd = CommandDefinition::former()
    .name( "invalid" )  // ❌ No dot prefix - panics here
    .description( "Test" )
    .end();
}

/// Test that both registration paths accept valid commands identically
///
/// **Baseline:** Verify consistency extends to success cases
#[test]
fn test_both_paths_accept_valid_commands_identically()
{
  let mut registry1 = CommandRegistry::new();
  let mut registry2 = CommandRegistry::new();

  let valid_cmd = CommandDefinition::former()
    .name( ".test" )
    .description( "Test command" )
    .end();

  // Both paths should accept
  let result1 = registry1.register( valid_cmd.clone() );
  let result2 = registry2.command_add_runtime( &valid_cmd, create_mock_routine() );

  assert!(
    result1.is_ok() && result2.is_ok(),
    "Both registration paths should accept valid commands"
  );

  // Both commands should be accessible
  assert!(
    registry1.command( ".test" ).is_some() && registry2.command( ".test" ).is_some(),
    "Commands registered via both paths should be accessible"
  );
}

//
// Edge Cases
//

/// Test that empty namespace is allowed (root-level commands)
///
/// **Specification:** Empty namespace means root-level, which is valid
#[test]
fn test_empty_namespace_is_valid()
{
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".build" )
    .description( "Build project" )
    // namespace defaults to empty string (root level)
    .end();

  let result = registry.register( cmd );

  assert!(
    result.is_ok(),
    "Commands with empty namespace (root level) should be accepted"
  );
}

/// Test that namespace with dot prefix is valid
///
/// **Specification:** Non-empty namespaces must start with dot
#[test]
fn test_namespace_with_dot_prefix_is_valid()
{
  let mut registry = CommandRegistry::new();

  let mut cmd = CommandDefinition::former()
    .name( ".build" )
    .description( "Build project" )
    .end();

  cmd.namespace = ".admin".to_string();

  let result = registry.register( cmd );

  assert!(
    result.is_ok(),
    "Commands with dot-prefixed namespace should be accepted"
  );
}

//
// Documentation Tests
//

/// Test that error messages guide users to correct usage
///
/// **Quality Check:** Errors should be actionable, not just rejections
#[test]
fn test_error_messages_are_helpful()
{
  let mut registry = CommandRegistry::new();

  // Phase 2 Update: Removed "no dot prefix" test case - validation now happens at
  // construction time (see #[should_panic] tests above)

  // Test: Duplicate registration
  let dup1 = CommandDefinition::former().name( ".test" ).description( "A" ).end();
  let dup2 = CommandDefinition::former().name( ".test" ).description( "B" ).end();

  registry.register( dup1 ).unwrap();
  let err2 = registry.register( dup2 ).unwrap_err().to_string();

  assert!(
    err2.contains( "already registered" ) && ( err2.contains( "unregister" ) || err2.contains( "replace" ) ),
    "Error should suggest alternatives: {}",
    err2
  );
}
