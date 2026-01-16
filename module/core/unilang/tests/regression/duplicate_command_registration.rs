//! Regression test for duplicate command registration bug.
//!
//! ## Lessons Learned (Bugs Fixed)
//!
//! - **2026-01-11 (issue-duplicate-commands):** Example tried to merge registries without
//!   checking for duplicates. Built-in commands (.help, .version) exist in both registries,
//!   causing duplicate registration errors. Fix: check if command exists before registering.

#![ allow( clippy::unnecessary_wraps ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::doc_markdown ) ]

use unilang::{ CommandDefinition, CommandRegistry };

// test_kind: bug_reproducer(issue-duplicate-commands)
#[ test ]
fn test_duplicate_registration_fails()
{
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".test" )
    .description( "Test command" )
    .end();

  // First registration succeeds
  let result1 = registry.register( cmd.clone() );
  assert!(
    result1.is_ok(),
    "First registration should succeed"
  );

  // Second registration of same command fails
  let result2 = registry.register( cmd.clone() );
  assert!(
    result2.is_err(),
    "Duplicate registration should fail"
  );

  let error = result2.unwrap_err().to_string();
  assert!(
    error.contains( "already registered" ),
    "Error should mention duplicate, got: {}",
    error
  );
}

#[ test ]
fn test_merging_registries_with_duplicates()
{
  // Create first registry with command
  let mut registry1 = CommandRegistry::new();
  let cmd1 = CommandDefinition::former()
    .name( ".shared_test" )
    .description( "Shared test command" )
    .end();
  registry1.register( cmd1.clone() ).unwrap();

  // Create second registry with same command
  let mut registry2 = CommandRegistry::new();
  registry2.register( cmd1.clone() ).unwrap();

  // Create empty target registry to avoid builtin conflicts
  let mut combined = CommandRegistry::new();

  // Merge registry1 - check before registering to handle any auto-registered commands
  for ( name, command ) in registry1.commands()
  {
    if combined.command( &name ).is_none()
    {
      combined.register( command.clone() ).unwrap();
    }
  }

  // Merge registry2 - would fail without duplicate check
  // CORRECT PATTERN: Check before registering
  for ( name, command ) in registry2.commands()
  {
    if combined.command( &name ).is_none()
    {
      combined.register( command.clone() ).unwrap();
    }
  }

  // Verify combined registry has command (not duplicated)
  assert!(
    combined.command( ".shared_test" ).is_some(),
    "Merged registry should have .shared_test command"
  );
}
