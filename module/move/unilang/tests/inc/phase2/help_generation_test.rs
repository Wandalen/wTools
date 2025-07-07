//! Tests for help generation and discovery.
//!
//! This module contains integration tests that invoke the `unilang_cli` binary
//! with help flags/commands and assert on the content and format of the generated help output.

use assert_cmd::Command;
use predicates::prelude::*;
// use unilang::registry::CommandRegistry; // Removed unused import
// use unilang::data::{ CommandDefinition, ArgumentDefinition, Kind }; // Removed unused import

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
// | T8.1  | `unilang_cli`      | "Available Commands:\n  echo\n  add\n  cat"             | "Usage: unilang_cli <command> [args...]"                 | 0                  | No arguments, lists all commands          |
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
  cmd.assert()
  .success()
  .stdout( predicate::str::contains( "Available Commands:" )
  .and( predicate::str::contains( "  echo            Echoes a message." ) )
  .and( predicate::str::contains( "  add             Adds two integers." ) )
  .and( predicate::str::contains( "  cat             Prints content of a file." ) ) )
  .stderr( predicate::str::ends_with( "unilang_cli <command> [args...]\n" ) );
}

#[ test ]
fn test_cli_global_help_flag()
{
  // Test Matrix Row: T8.2
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.arg( "--help" );
  cmd.assert()
  .success()
  .stdout( predicate::str::contains( "Available Commands:" )
  .and( predicate::str::contains( "  echo            Echoes a message." ) )
  .and( predicate::str::contains( "  add             Adds two integers." ) )
  .and( predicate::str::contains( "  cat             Prints content of a file." ) ) )
  .stderr( "" ); // No stderr for successful help
}

#[ test ]
fn test_cli_global_help_command()
{
  // Test Matrix Row: T8.3
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.arg( "help" );
  cmd.assert()
  .success()
  .stdout( predicate::str::contains( "Available Commands:" )
  .and( predicate::str::contains( "  echo            Echoes a message." ) )
  .and( predicate::str::contains( "  add             Adds two integers." ) )
  .and( predicate::str::contains( "  cat             Prints content of a file." ) ) )
  .stderr( "" ); // No stderr for successful help
}

#[ test ]
fn test_cli_specific_command_help_echo()
{
  // Test Matrix Row: T8.4
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.args( &vec![ "help", "echo" ] );
  cmd.assert()
  .success()
  .stdout( predicate::str::contains( "Usage: echo\n\n  Echoes a message." ) )
  .stderr( "" );
}

#[ test ]
fn test_cli_specific_command_help_add()
{
  // Test Matrix Row: T8.5
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.args( &vec![ "help", "add" ] );
  cmd.assert()
  .success()
  .stdout( predicate::str::contains( "Usage: add\n\n  Adds two integers.\n\n\nArguments:\n  a                (Kind: Integer)\n  b                (Kind: Integer)\n" ) )
  .stderr( "" );
}

#[ test ]
fn test_cli_help_non_existent_command()
{
  // Test Matrix Row: T8.6
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.args( &vec![ "help", "non_existent" ] );
  cmd.assert()
  .failure()
  .stderr( predicate::str::contains( "Error: Command 'non_existent' not found for help." ) );
}

#[ test ]
fn test_cli_invalid_help_usage()
{
  // Test Matrix Row: T8.7
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.args( &vec![ "help", "arg1", "arg2" ] );
  cmd.assert()
  .failure()
  .stderr( predicate::str::contains( "Error: Invalid usage of help command." ) );
}