//! ## Test Matrix for Debug Parsing
//!
//! This matrix details test cases for debugging specific parsing behaviors.
//!
//! **Test Factors:**
//! - Input String
//! - Expected Outcome
//!
//! ---
//!
//! **Test Combinations:**
//!
//! | ID | Input String | Expected Behavior |
//! |---|---|---|
//! | D1.1 | `test_cmd hello 123` | Parses `test_cmd` as command, `hello`, `123` as positional arguments. |

use unilang_parser::{ Parser, UnilangParserOptions };

/// Tests the parsing of "`test_cmd` hello 123" to debug unexpected command path behavior.
/// Test Combination: D1.1
#[ test ]
fn debug_test_cmd_hello_123_parsing()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = "test_cmd hello 123";
  let result = parser.parse_single_instruction( input );

  assert!( result.is_ok(), "Parse error: {:?}", result.err() );
  let instruction = result.unwrap();

  assert_eq!( instruction.command_path_slices, vec![ "test_cmd".to_string() ] );
  assert_eq!( instruction.positional_arguments.len(), 2 );
  assert_eq!( instruction.positional_arguments[ 0 ].value, "hello".to_string() );
  assert_eq!( instruction.positional_arguments[ 1 ].value, "123".to_string() );
  assert!( instruction.named_arguments.is_empty() );
}
