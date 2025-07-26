//!
//! Integration tests for new data model features.
//!
use assert_cmd::Command;
use predicates::prelude::*;

/// Tests that command aliasing is not yet implemented and fails.
/// Test Combination: T-ALIAS-1
#[ test ]
fn test_command_alias_fails_before_implementation()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.arg( "e" ); // 'e' is an alias for 'echo'
  cmd.assert()
  .success()
  .stdout( predicate::str::contains( "Echo command executed!" ) )
  .stderr( "" );
}