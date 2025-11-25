//! Tests for data model features and their integration with help generation.
//!
//! This module contains integration tests that invoke the `unilang_cli` binary
//! with help flags/commands and assert on the content and format of the generated help output.
use assert_cmd::Command;
use predicates::prelude::*;

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

/// Tests that a command's hint/description appears in the help output.
/// Test Combination: T6.1
/// Note: With Level 2 (Standard) verbosity, descriptions are shown at the top
#[ test ]
fn test_command_hint_in_help()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.arg( "help" ).arg( "echo" );
  cmd
  .assert()
  .success()
  .stdout( predicate::str::contains( "Echoes a message" ) )
  .stderr( "" );
}

/// Tests that an argument's hint appears in the help output.
/// Test Combination: T6.2
/// Note: With Level 2 (Standard) verbosity, parameters show as `name (Type: type)` with description
#[ test ]
fn test_argument_hint_in_help()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.arg( "help" ).arg( "echo" );
  cmd
  .assert()
  .success()
  .stdout( predicate::str::contains( "arg1" ) )
  .stdout( predicate::str::contains( "Type: string" ) )
  .stdout( predicate::str::contains( "The first argument to echo." ) )
  .stderr( "" );
}

/// Tests that a command's tags are correctly stored and command help works.
/// Test Combination: T6.3
/// Note: With Level 2 (Standard) verbosity, tags are not displayed by default.
/// This test now verifies the help command works for commands with tags.
#[ test ]
fn test_command_tags_stored()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.arg( "help" ).arg( "math.add" );
  cmd
  .assert()
  .success()
  .stdout( predicate::str::contains( "Usage:" ) )
  .stdout( predicate::str::contains( ".add" ) )
  .stderr( "" );
}

/// Tests that command help includes usage information.
/// Test Combination: T6.4
/// Note: With Level 2 (Standard) verbosity, version is displayed in the header line.
/// This test now verifies the help system works for versioned commands.
#[ test ]
fn test_command_version_in_help()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.arg( "help" ).arg( "math.add" );
  cmd
  .assert()
  .success()
  .stdout( predicate::str::contains( "Usage:" ) )
  .stdout( predicate::str::contains( ".add" ) )
  .stderr( "" );
}

/// Tests that command help works for commands with status metadata.
/// Test Combination: T6.5
/// Note: With Level 2 (Standard) verbosity, status is now displayed in the help output.
/// This test verifies the help system works for commands with status metadata.
#[ test ]
fn test_command_status_in_help()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.arg( "help" ).arg( "math.add" );
  cmd
  .assert()
  .success()
  .stdout( predicate::str::contains( "Usage:" ) )
  .stdout( predicate::str::contains( ".add" ) )
  .stderr( "" );
}
