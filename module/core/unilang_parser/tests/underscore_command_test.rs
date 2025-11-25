//! Tests for command parsing with underscores and mixed arguments

use unilang_parser :: { Parser, UnilangParserOptions };

/// Tests parsing of command with underscore followed by positional arguments.
#[ test ]
fn test_underscore_command_with_positional_args()
{
  let parser = Parser ::new( UnilangParserOptions ::default() );
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
