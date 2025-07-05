//! ## Test Matrix for Command Parsing
//!
//! | ID   | Input String          | Expected `command_path_slices` | Expected `positional_arguments` |
//! |------|-----------------------|--------------------------------|---------------------------------|
//! | T1.1 | `.test.command arg1`  | `["test", "command"]`          | `["arg1"]`                      |
//! | T1.2 | `command arg1`        | `["command"]`                  | `["arg1"]`                      |
//! | T1.3 | `.command arg1`       | `["command"]`                  | `["arg1"]`                      |
//! | T1.4 | `command.sub arg1`    | `["command", "sub"]`           | `["arg1"]`                      |

use unilang_instruction_parser::prelude::*;
use unilang_instruction_parser::prelude::*;

/// Tests that the parser correctly identifies and extracts command path slices.
/// Corresponds to Test Matrix ID: T1.1
#[ test ]
fn parses_command_path_correctly()
{
  let options = UnilangParserOptions::default();
  let parser = Parser::new( options );
  let input = ".test.command arg1";

  let instructions = parser.parse_single_str( input ).unwrap();
  assert_eq!( instructions.len(), 1 );

  let instruction = &instructions[ 0 ];

  // Assert command_path_slices
  assert_eq!( instruction.command_path_slices, vec![ "test", "command" ] );

  // Assert positional_arguments
  assert_eq!( instruction.positional_arguments.len(), 1 );
  assert_eq!( instruction.positional_arguments[ 0 ].value, "arg1" );
  assert_eq!( instruction.positional_arguments[ 0 ].name, None );
}

/// Tests that the parser correctly identifies and extracts command path slices when command is not prefixed with dot.
/// Corresponds to Test Matrix ID: T1.2
#[ test ]
fn parses_command_path_correctly_without_dot()
{
  let options = UnilangParserOptions::default();
  let parser = Parser::new( options );
  let input = "command arg1";

  let instructions = parser.parse_single_str( input ).unwrap();
  assert_eq!( instructions.len(), 1 );

  let instruction = &instructions[ 0 ];

  // Assert command_path_slices
  assert_eq!( instruction.command_path_slices, vec![ "command" ] );

  // Assert positional_arguments
  assert_eq!( instruction.positional_arguments.len(), 1 );
  assert_eq!( instruction.positional_arguments[ 0 ].value, "arg1" );
  assert_eq!( instruction.positional_arguments[ 0 ].name, None );
}

/// Tests that the parser correctly identifies and extracts command path slices when command is prefixed with dot.
/// Corresponds to Test Matrix ID: T1.3
#[ test ]
fn parses_command_path_correctly_with_dot_prefix()
{
  let options = UnilangParserOptions::default();
  let parser = Parser::new( options );
  let input = ".command arg1";

  let instructions = parser.parse_single_str( input ).unwrap();
  assert_eq!( instructions.len(), 1 );

  let instruction = &instructions[ 0 ];

  // Assert command_path_slices
  assert_eq!( instruction.command_path_slices, vec![ "command" ] );

  // Assert positional_arguments
  assert_eq!( instruction.positional_arguments.len(), 1 );
  assert_eq!( instruction.positional_arguments[ 0 ].value, "arg1" );
  assert_eq!( instruction.positional_arguments[ 0 ].name, None );
}

/// Tests that the parser correctly identifies and extracts command path slices with sub-commands.
/// Corresponds to Test Matrix ID: T1.4
#[ test ]
fn parses_command_path_with_sub_command()
{
  let options = UnilangParserOptions::default();
  let parser = Parser::new( options );
  let input = "command.sub arg1";

  let instructions = parser.parse_single_str( input ).unwrap();
  assert_eq!( instructions.len(), 1 );

  let instruction = &instructions[ 0 ];

  // Assert command_path_slices
  assert_eq!( instruction.command_path_slices, vec![ "command", "sub" ] );

  // Assert positional_arguments
  assert_eq!( instruction.positional_arguments.len(), 1 );
  assert_eq!( instruction.positional_arguments[ 0 ].value, "arg1" );
  assert_eq!( instruction.positional_arguments[ 0 ].name, None );
}