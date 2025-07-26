//! Test suite for parsing paths with special characters.

use unilang_parser::{ Parser, UnilangParserOptions };
use unilang_parser::instruction::{ Argument, GenericInstruction };
use unilang_parser::error::{ ParseError, ErrorKind, SourceLocation };
use std::collections::HashMap;

#[test]
fn test_parse_path_with_dots()
{
  let options = UnilangParserOptions::default();
  let parser = Parser::new( options );
  let input = "cat path::/tmp/.test.file";
  let result = parser.parse_single_instruction( input );

  let mut expected_named_args = HashMap::new();
  expected_named_args.insert
  (
    "path".to_string(),
    Argument
    {
      name : Some( "path".to_string() ),
      value : "/tmp/.test.file".to_string(),
      name_location : Some( SourceLocation::StrSpan { start : 4, end : 8 } ),
      value_location : SourceLocation::StrSpan { start : 10, end : 25 }
    }
  );

  let expected_instruction = GenericInstruction
  {
    command_path_slices : vec![ "cat".to_string() ],
    positional_arguments : vec![],
    named_arguments : expected_named_args,
    help_requested : false,
    overall_location : SourceLocation::StrSpan { start : 0, end : 25 },
  };

  assert_eq!( result.unwrap(), expected_instruction );
}