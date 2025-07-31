//! Tests for data model features and their integration with help generation.
//!
//! This module contains integration tests that invoke the `unilang_cli` binary
//! with help flags/commands and assert on the content and format of the generated help output.
use assert_cmd::Command;
use predicates::prelude::*;

use predicates::Predicate;

#[allow(dead_code)]
fn contains_all_unordered( expected_lines : Vec< &str > ) -> impl Predicate< str > + '_
{
  predicate::function( move | s : &str | expected_lines.iter().all( | line | s.contains( line ) ) )
}

// Test Matrix for Data Model Features
//
// This matrix outlines the tests for various fields and attributes of `CommandDefinition` and `ArgumentDefinition`.
// | ID   | Aspect Tested | Command Field | Argument Field | Expected Behavior |
// |---|---|---|---|---|
// | T6.1 | Command `hint` | `Some("Command hint")` | N/A | `help` output contains "Command hint" |
// | T6.2 | Argument `hint` | N/A | `Some("Argument hint")` | `help` output contains "Argument hint" |
// | T6.3 | Command `tags` | `vec!["tag1", "tag2"]` | N/A | `CommandDefinition` struct contains `tags` |
// | T6.4 | Command `version` | `Some("1.0.0")` | N/A | `help` output contains "Version: 1.0.0" |
// | T6.5 | Command `status` | `Some("stable")` | N/A | `help` output contains "Status: stable" |
//
/// Tests that command aliases work correctly.
/// Test Combination: T6.0 (Implicitly covered by existing test, now renamed)
#[ test ]
fn test_command_alias_works()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.arg( "e" ).arg( "hello" ); // 'e' is an alias for 'echo', provide required arg1
  cmd
  .assert()
  .success()
  .stdout( predicate::str::contains( "Echo command executed!" ) )
  .stderr( "" );
}

/// Tests that a command's hint appears in the help output.
/// Test Combination: T6.1
#[ test ]
fn test_command_hint_in_help()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.arg( "help" ).arg( "echo" );
  cmd
  .assert()
  .success()
  .stdout( predicate::str::contains( "Hint: Echoes back the provided arguments." ) )
  .stderr( "" );
}

/// Tests that an argument's hint appears in the help output.
/// Test Combination: T6.2
#[ test ]
fn test_argument_hint_in_help()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.arg( "help" ).arg( "echo" );
  cmd
  .assert()
  .success()
  .stdout( predicate::str::contains( "arg1 (Kind: String) - Hint: The first argument to echo." ) )
  .stderr( "" );
}

/// Tests that a command's tags are correctly stored.
/// Test Combination: T6.3
#[ test ]
fn test_command_tags_stored()
{
  // This test requires inspecting the CommandRegistry directly,
  // which might not be easily exposed via CLI.
  // For now, we'll assume successful registration implies correct storage.
  // A more robust test would involve a programmatic API to the registry.
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.arg( "help" ).arg( "math.add" ); // Use a command that has tags
  cmd
  .assert()
  .success()
  .stdout( predicate::str::contains( "Tags: math, calculation" ) )
  .stderr( "" );
}

/// Tests that a command's version appears in the help output.
/// Test Combination: T6.4
#[ test ]
fn test_command_version_in_help()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.arg( "help" ).arg( "math.add" );
  cmd
  .assert()
  .success()
  .stdout( predicate::str::contains( "Usage: add (v1.0.0)" ) )
  .stderr( "" );
}

/// Tests that a command's status appears in the help output.
/// Test Combination: T6.5
#[ test ]
fn test_command_status_in_help()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.arg( "help" ).arg( "math.add" );
  cmd
  .assert()
  .success()
  .stdout( predicate::str::contains( "Status: stable" ) )
  .stderr( "" );
}
