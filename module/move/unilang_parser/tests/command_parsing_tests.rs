//! ## Test Matrix for Command Path Parsing

//!
//! This matrix details the test cases for parsing command paths, covering various dot usages and argument presence.
//!
//! **Test Factors:**
//! - Input Type: Command path only, Command path with positional arguments
//! - Command Path Format: Simple, Dotted, Leading Dot, Infix Dot
//! - Arguments: Present, Absent
//!
//! ---
//!
//! **Test Combinations:**
//!
//! | ID | Aspect Tested | Input String | Expected Command Path Slices | Expected Positional Arguments | Expected Behavior |
//! |---|---|---|---|---|---|
//! | T2.1 | Dotted prefix command with args | `.test.command arg1` | `["test", "command"]` | `["arg1"]` | Parses command path and positional arguments correctly. |
//! | T2.2 | Simple command with args | `command arg1` | `["command"]` | `["arg1"]` | Parses simple command path and positional arguments correctly. |
//! | T2.3 | Leading dot command with args | `.command arg1` | `["command"]` | `["arg1"]` | Consumes leading dot, parses command path and positional arguments correctly. |
//! | T2.4 | Infix dot command with args | `command.sub arg1` | `["command", "sub"]` | `["arg1"]` | Parses command path with infix dot and positional arguments correctly. |
//! | T2.5 | Command only | `command` | `["command"]` | `[]` | Parses command path correctly with no arguments. |

use unilang_parser::{ Parser, UnilangParserOptions };

fn parse_and_assert( input : &str, expected_path : &[ &str ], expected_args : &[ &str ] )
{
  let options = UnilangParserOptions::default();
  let parser = Parser::new( options ); // Updated Parser instantiation
  let instruction = parser.parse_single_instruction( input ).unwrap(); // Updated method call and direct unwrap
  assert_eq!( instruction.command_path_slices, expected_path );
  assert_eq!( instruction.positional_arguments.len(), expected_args.len() );
  for ( i, expected_arg ) in expected_args.iter().enumerate()
  {
    assert_eq!( instruction.positional_arguments[ i ].value, (*expected_arg).to_string() );
  }
}

/// Tests parsing of a command path with a dotted prefix and arguments.
/// Test Combination: T2.1
#[ test ]
fn parses_dotted_prefix_command_path_correctly()
{
  parse_and_assert( ".test.command arg1", &[ "test", "command" ], &[ "arg1" ] );
}

/// Tests parsing of a simple command path with arguments.
/// Test Combination: T2.2
#[ test ]
fn parses_simple_command_path_correctly()
{
  parse_and_assert( "command arg1", &[ "command" ], &[ "arg1" ] );
}

/// Tests parsing of a command path with a leading dot and arguments.
/// Test Combination: T2.3
#[ test ]
fn parses_leading_dot_command_path_correctly()
{
  parse_and_assert( ".command arg1", &[ "command" ], &[ "arg1" ] );
}

/// Tests parsing of a command path with an infix dot and arguments.
/// Test Combination: T2.4
#[ test ]
fn parses_infix_dot_command_path_correctly()
{
  parse_and_assert( "command.sub arg1", &[ "command", "sub" ], &[ "arg1" ] );
}

/// Tests parsing of a command path with no arguments.
/// Test Combination: T2.5
#[ test ]
fn parses_command_only_correctly()
{
  parse_and_assert( "command", &[ "command" ], &[] );
}
/// Tests that a command path with a hyphen (kebab-case) is rejected.
#[ test ]
fn rejects_kebab_case_in_command_path()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = "cmd.my-sub.command arg1";
  let result = parser.parse_single_instruction( input );
  assert!( result.is_err(), "Expected error for kebab-case in command path" );
  if let Err( e ) = result
  {
    assert!( matches!( e.kind, ErrorKind::Syntax( _ ) ) );
    assert!( e
      .to_string()
      .contains( "Invalid character '-' in command path segment 'my-sub'" ) );
  }
}
use unilang_parser::error::ErrorKind;
