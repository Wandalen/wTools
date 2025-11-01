//! Validation Enforcement Tests (Phase 1 Fixes)
//!
//! Tests verifying that `register()` enforces the same validation rules as
//! `command_add_runtime()`, preventing the code path divergence vulnerability
//! identified in Task 085 (Day 3 Audit).
//!
//! ## Test Matrix
//!
//! | Test ID | What's Tested | Input | Expected | Bug Fixed |
//! |---------|---------------|-------|----------|-----------|
//! | VAL-1 | Missing dot prefix | "build" | Error | Phase 1.1 |
//! | VAL-2 | Invalid namespace | "ns" | Error | Phase 1.1 |
//! | VAL-3 | Duplicate registration | Same name twice | Error | Phase 1.2 |
//! | VAL-4 | Valid command | ".build" | Success | Baseline |
//! | VAL-5 | Path consistency | Both paths reject | Identical | Phase 1.1 |
//!
//! ## Scope
//!
//! Tests that Phase 1 fixes prevent:
//! 1. Commands without dot prefix being registered via `register()`
//! 2. Commands with invalid namespaces being registered
//! 3. Duplicate commands silently overwriting existing ones
//! 4. Code path divergence between `register()` and `command_add_runtime()`

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

/// Test that `register()` rejects commands without dot prefix
///
/// **Bug Fixed:** Phase 1.1 - Previously `register()` didn't validate,
/// allowing invalid commands to be registered.
#[test]
fn test_register_rejects_command_without_dot_prefix()
{
  let mut registry = CommandRegistry::new();

  let invalid_cmd = CommandDefinition::former()
    .name( "build" )  // ❌ No dot prefix
    .description( "Build project" )
    .end();

  let result = registry.register( invalid_cmd );

  assert!(
    result.is_err(),
    "register() should reject commands without dot prefix"
  );

  let err_msg = result.unwrap_err().to_string();
  assert!(
    err_msg.contains( "must start with dot prefix" ),
    "Error message should explain the naming rule"
  );
}

/// Test that `command_add_runtime()` also rejects commands without dot prefix
///
/// **Baseline:** This always worked - used as comparison for VAL-5
#[test]
fn test_command_add_runtime_rejects_command_without_dot_prefix()
{
  let mut registry = CommandRegistry::new();

  let invalid_cmd = CommandDefinition::former()
    .name( "build" )  // ❌ No dot prefix
    .description( "Build project" )
    .end();

  let result = registry.command_add_runtime( &invalid_cmd, create_mock_routine() );

  assert!(
    result.is_err(),
    "command_add_runtime() should reject commands without dot prefix"
  );

  let err_msg = result.unwrap_err().to_string();
  assert!(
    err_msg.contains( "must start with dot prefix" ),
    "Error messages should match between paths"
  );
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
    registered_cmd.description,
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
fn test_both_paths_reject_invalid_commands_identically()
{
  let mut registry1 = CommandRegistry::new();
  let mut registry2 = CommandRegistry::new();

  let invalid_cmd = CommandDefinition::former()
    .name( "invalid" )  // ❌ No dot prefix
    .description( "Test" )
    .end();

  // Both paths should reject
  let result1 = registry1.register( invalid_cmd.clone() );
  let result2 = registry2.command_add_runtime( &invalid_cmd, create_mock_routine() );

  assert!(
    result1.is_err() && result2.is_err(),
    "Both registration paths should reject invalid commands"
  );

  // Error messages should be similar (both mention dot prefix requirement)
  let err1 = result1.unwrap_err().to_string();
  let err2 = result2.unwrap_err().to_string();

  assert!(
    err1.contains( "dot prefix" ) && err2.contains( "dot prefix" ),
    "Both paths should give similar error messages about naming convention"
  );
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

  // Test 1: No dot prefix
  let no_dot = CommandDefinition::former()
    .name( "build" )
    .description( "Test" )
    .end();

  let err1 = registry.register( no_dot ).unwrap_err().to_string();
  assert!(
    err1.contains( "must start" ) && err1.contains( "dot prefix" ),
    "Error should explain what's required: {}",
    err1
  );

  // Test 2: Duplicate
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
