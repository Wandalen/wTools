//! Tests for the core parsing structures.

use unilang::ca::parsing::input::{ Location, InputState, InputAbstraction, DelimiterType, InputPart };
use unilang::ca::parsing::instruction::GenericInstruction;
use unilang::ca::parsing::error::ParseError;

#[ test ]
fn test_location_enum()
{
  let byte_loc = Location::ByteOffset( 10 );
  let segment_loc = Location::SegmentOffset( 2, 5 );

  assert_eq!( byte_loc, Location::ByteOffset( 10 ) );
  assert_eq!( segment_loc, Location::SegmentOffset( 2, 5 ) );
  assert_ne!( byte_loc, Location::SegmentOffset( 10, 0 ) );
}

#[ test ]
fn test_input_state_enum()
{
  let single_state = InputState::SingleString { input : "test", offset : 0 };
  let segment_state = InputState::SegmentSlice { segments : &["a", "b"], segment_index : 0, offset_in_segment : 0 };

  assert_eq!( single_state, InputState::SingleString { input : "test", offset : 0 } );
  assert_eq!( segment_state, InputState::SegmentSlice { segments : &["a", "b"], segment_index : 0, offset_in_segment : 0 } );
  assert_ne!( single_state, InputState::SegmentSlice { segments : &["test"], segment_index : 0, offset_in_segment : 0 } );
}

#[ test ]
fn test_input_abstraction_creation()
{
  let single_abs = InputAbstraction::from_str( "test" );
  let segment_abs = InputAbstraction::from_segments( &["a", "b"] );

  assert_eq!( single_abs.current_location(), Location::ByteOffset( 0 ) );
  assert_eq!( single_abs.is_empty(), false );
  assert_eq!( segment_abs.current_location(), Location::SegmentOffset( 0, 0 ) );
  assert_eq!( segment_abs.is_empty(), false );
}

#[ test ]
fn test_delimiter_type_enum()
{
  assert_eq!( DelimiterType::ColonColon, DelimiterType::ColonColon );
  assert_ne!( DelimiterType::ColonColon, DelimiterType::SemiColonSemiColon );
}

#[ test ]
fn test_input_part_enum()
{
  let segment_part = InputPart::Segment( "value" );
  let delimiter_part = InputPart::Delimiter( DelimiterType::QuestionMark );

  assert_eq!( segment_part, InputPart::Segment( "value" ) );
  assert_eq!( delimiter_part, InputPart::Delimiter( DelimiterType::QuestionMark ) );
  // qqq: Removed invalid comparison using `as any`.
}

#[ test ]
fn test_generic_instruction_struct()
{
  let instruction = GenericInstruction
  {
    command_name : ".my.command",
    named_args : vec![ ("arg1", "value1"), ("arg2", "value2") ],
    positional_args : vec![ "pos1", "pos2" ],
    help_requested : false,
  };

  assert_eq!( instruction.command_name, ".my.command" );
  assert_eq!( instruction.named_args, vec![ ("arg1", "value1"), ("arg2", "value2") ] );
  assert_eq!( instruction.positional_args, vec![ "pos1", "pos2" ] );
  assert_eq!( instruction.help_requested, false );
}

#[ test ]
fn test_parse_error_enum()
{
  let loc = Location::ByteOffset( 10 );
  let error1 = ParseError::UnexpectedToken { location : loc, token : "::".to_string() };
  let error2 = ParseError::UnterminatedQuote { location : loc, quote_char : ' ' };

  assert_eq!( error1, ParseError::UnexpectedToken { location : loc, token : "::".to_string() } );
  assert_eq!( error2, ParseError::UnterminatedQuote { location : loc, quote_char : ' ' } );
  assert_ne!( error1, error2 );
}