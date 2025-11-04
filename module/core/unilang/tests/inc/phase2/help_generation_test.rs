//! Tests for help generation and discovery.
//!
//! This module contains integration tests that invoke the `unilang_cli` binary
//! with help flags/commands and assert on the content and format of the generated help output.

use assert_cmd::Command;
use predicates::prelude::*;

use predicates::Predicate;

fn contains_all_unordered( expected_lines : Vec< &str > ) -> impl Predicate< str > + '_
{
  predicate::function( move | s : &str | expected_lines.iter().all( | line | s.contains( line ) ) )
}

// Test Matrix for Help Generation
//
// Factors:
// - Help Command: "--help", "help", "help <command_name>", "help <non_existent_command>"
// - Expected Output: stdout (list of commands, specific command help), stderr (error messages), exit code
//
// Combinations:
//
// | ID    | Command Invocation | Expected Stdout (contains)                               | Expected Stderr (contains)                               | Expected Exit Code | Notes                                     |
// |-------|--------------------|----------------------------------------------------------|----------------------------------------------------------|--------------------|-------------------------------------------|
// | T8.1  | `unilang_cli`      | "Available Commands:\n  echo\n  add\n  cat"             | "Usage: unilang_cli <command> [args...]"                 | 0                  | Basic echo command                        |
// | T8.2  | `unilang_cli --help` | "Available Commands:\n  echo\n  add\n  cat"             |                                                          | 0                  | Global help, lists all commands           |
// | T8.3  | `unilang_cli help` | "Available Commands:\n  echo\n  add\n  cat"             |                                                          | 0                  | Global help, lists all commands (alias)   |
// | T8.4  | `unilang_cli help echo` | "Usage: echo\n\n  Echoes a message."                 |                                                          | 0                  | Specific command help                     |
// | T8.5  | `unilang_cli help add` | "Usage: add\n\n  Adds two integers.\n\nArguments:\n  a              (Kind: Integer)\n  b              (Kind: Integer)" |                                                          | 0                  | Specific command help with arguments      |
// | T8.6  | `unilang_cli help non_existent` |                                                          | "Error: Command 'non_existent' not found for help."      | 1                  | Help for non-existent command             |
// | T8.7  | `unilang_cli help arg1 arg2` |                                                          | "Error: Invalid usage of help command."                  | 1                  | Invalid help command usage                |

#[ test ]
fn test_cli_no_args_help()
{
  // Test Matrix Row: T8.1
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd
  .assert()
  .success()
  .stdout( contains_all_unordered( vec![
    "Available commands:",
    "  .math.add            Adds two numbers.",
    "  .math.sub            Subtracts two numbers.",
    "  .greet               Greets the specified person.",
    "  .config.set          Sets a configuration value.",
  ]) )
  .stderr( predicate::str::contains( "Usage: unilang_cli <command> [args...]" ) );
}

#[ test ]
fn test_cli_global_help_flag()
{
  // Test Matrix Row: T8.2
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.arg( "--help" );
  cmd
  .assert()
  .success()
  .stdout( contains_all_unordered( vec![
    "Available commands:",
    "  .math.add            Adds two numbers.",
    "  .math.sub            Subtracts two numbers.",
    "  .greet               Greets the specified person.",
    "  .config.set          Sets a configuration value.",
  ]) )
  .stderr( "" ); // No stderr for successful help
}

#[ test ]
fn test_cli_global_help_command()
{
  // Test Matrix Row: T8.3
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.arg( "help" );
  cmd
  .assert()
  .success()
  .stdout( contains_all_unordered( vec![
    "Available commands:",
    "  .math.add            Adds two numbers.",
    "  .math.sub            Subtracts two numbers.",
    "  .greet               Greets the specified person.",
    "  .config.set          Sets a configuration value.",
  ]) )
  .stderr( "" ); // No stderr for successful help
}

#[ test ]
fn test_cli_specific_command_help_add()
{
  // Test Matrix Row: T8.5
  // Note: Using Level 2 (Standard) verbosity by default - improved readability format
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.args( vec![ "help", ".math.add" ] );
  cmd
  .assert()
  .success()
  .stdout(
    predicate::str::contains( "Usage:" )
    .and( predicate::str::contains( ".add" ) )
    .and( predicate::str::contains( "Adds two numbers" ) )
    .and( predicate::str::contains( "Arguments:" ) )
    .and( predicate::str::contains( "a" ) )
    .and( predicate::str::contains( "Type: integer" ) )
    .and( predicate::str::contains( "First number" ) )
    .and( predicate::str::contains( "b" ) )
    .and( predicate::str::contains( "Second number" ) ),
  )
  .stderr( "" );
}

#[ test ]
fn test_cli_help_non_existent_command()
{
  // Test Matrix Row: T8.6
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.args( vec![ "help", "non_existent" ] );
  cmd
  .assert()
  .failure()
  .stderr( predicate::str::contains( "Error: Command 'non_existent' not found for help." ) );
}

#[ test ]
fn test_cli_invalid_help_usage()
{
  // Test Matrix Row: T8.7
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.args( vec![ "help", "arg1", "arg2" ] );
  cmd.assert().failure().stderr( predicate::str::contains(
    "Error: Invalid usage of help command. Use `help` or `help <command_name>`.",
  ) );
}
