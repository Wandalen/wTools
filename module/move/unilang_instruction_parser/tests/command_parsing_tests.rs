//! ## Test Matrix for Command Path Parsing
//!
//! | ID   | Input String         | Expected `command_path_slices` | Expected `positional_arguments` | Notes                                   |
//! |------|----------------------|--------------------------------|---------------------------------|-----------------------------------------|
//! | T1.1 | `.test.command arg1` | `["test", "command"]`          | `["arg1"]`                      | The primary failing case.               |
//! | T1.2 | `command arg1`       | `["command"]`                  | `["arg1"]`                      | Should already pass.                    |
//! | T1.3 | `.command arg1`      | `["command"]`                  | `["arg1"]`                      | Should fail.                            |
//! | T1.4 | `command.sub arg1`   | `["command", "sub"]`           | `["arg1"]`                      | Should fail.                            |
//! | T1.5 | `command`            | `["command"]`                  | `[]`                            | Should already pass.                    |

use unilang_instruction_parser::{ Parser, UnilangParserOptions };

fn parse_and_assert( input : &str, expected_path : &[ &str ], expected_args : &[ &str ] )
{
  let options = UnilangParserOptions::default();
  let parser = Parser::new( options );
  let instructions = parser.parse_single_str( input ).unwrap();
  assert_eq!( instructions.len(), 1 );
  let instruction = &instructions[ 0 ];
  assert_eq!( instruction.command_path_slices, expected_path );
  let positional_values: Vec<&str> = instruction.positional_arguments.iter().map(|arg| arg.value.as_str()).collect();
  assert_eq!( positional_values, expected_args );
}

/// Tests the primary failing case.
/// Test Combination: T1.1
#[test]
fn parses_dotted_prefix_command_path_correctly()
{
  parse_and_assert( ".test.command arg1", &["test", "command"], &["arg1"] );
}

/// Tests a simple command without dots.
/// Test Combination: T1.2
#[test]
fn parses_simple_command_path_correctly()
{
  parse_and_assert( "command arg1", &["command"], &["arg1"] );
}

/// Tests a command with a leading dot.
/// Test Combination: T1.3
#[test]
fn parses_leading_dot_command_path_correctly()
{
  parse_and_assert( ".command arg1", &["command"], &["arg1"] );
}

/// Tests a command with an infix dot.
/// Test Combination: T1.4
#[test]
fn parses_infix_dot_command_path_correctly()
{
  parse_and_assert( "command.sub arg1", &["command", "sub"], &["arg1"] );
}

/// Tests a command with no arguments.
/// Test Combination: T1.5
#[test]
fn parses_command_only_correctly()
{
  parse_and_assert( "command", &["command"], &[] );
}