//! Tests specifically for error reporting and SourceLocation in the unilang instruction parser.

use unilang_instruction_parser::*;
use unilang_instruction_parser::error::{ParseError, ErrorKind, SourceLocation};
use std::borrow::Cow;

fn default_options() -> UnilangParserOptions {
  UnilangParserOptions::default()
}

// Detailed Plan Step 6: Add 1-2 specific tests to verify error locations.

#[test]
fn error_invalid_escape_sequence_location_str() {
  let parser = Parser::new(default_options());
  // Input with an invalid escape sequence in a string
  let input = r#"cmd arg1 "value with \x invalid escape""#;
  let result = parser.parse_single_str(input);

  assert!(result.is_err(), "parse_single_str unexpectedly succeeded");
  let err = result.unwrap_err();

  assert!(matches!(err.kind, ErrorKind::InvalidEscapeSequence));

  // Expected location of the invalid escape sequence '\x'
  // The string starts at index 10. The escape sequence starts at index 22 (\)
  // The invalid character 'x' is at index 23.
  // The location should cover '\x'.
  let expected_location = Some(SourceLocation::StrSpan { start: 20, end: 22 });
  assert_eq!(err.location, expected_location, "Incorrect error location for invalid escape sequence");
}

#[test]
fn error_unexpected_delimiter_location_str() {
  let parser = Parser::new(default_options());
  // Input with an unexpected delimiter '::' in the arguments section
  let input = r#"cmd arg1 :: arg2"#; // '::' is unexpected after 'arg1'
  let result = parser.parse_single_str(input);

  assert!(result.is_err(), "parse_single_str unexpectedly succeeded");
  let err = result.unwrap_err();

  assert!(matches!(err.kind, ErrorKind::Syntax(_)));
  assert!(err.to_string().contains("Unexpected delimiter '::' in arguments section"));

  // Expected location of the unexpected delimiter '::'
  // 'cmd' is 3 chars, space 1, 'arg1' 4 chars, space 1. '::' starts at index 9.
  let expected_location = Some(SourceLocation::StrSpan { start: 8, end: 10 });
  assert_eq!(err.location, expected_location, "Incorrect error location for unexpected delimiter");
}

#[test]
fn error_invalid_escape_sequence_location_slice() {
  let parser = Parser::new(default_options());
  // Input with an invalid escape sequence in a string within a slice segment
  let input: &[&str] = &[r#"cmd"#, r#"arg1"#, r#""value with \y invalid escape""#]; // Invalid escape in segment 2
  let result = parser.parse_slice(input);

  assert!(result.is_err(), "parse_slice unexpectedly succeeded");
  let err = result.unwrap_err();

  assert!(matches!(err.kind, ErrorKind::InvalidEscapeSequence));

  // Expected location of the invalid escape sequence '\y' in segment 2
  // The string in segment 2 is '"value with \y invalid escape"'.
  // The escape sequence starts at index 12 (\) within this segment.
  // The invalid character 'y' is at index 13.
  // The location should cover '\y' within segment 2.
  let expected_location = Some(SourceLocation::SliceSegment { segment_index: 2, start_in_segment: 12, end_in_segment: 14 });
  assert_eq!(err.location, expected_location, "Incorrect error location for invalid escape sequence in slice");
}

#[test]
fn error_unexpected_delimiter_location_slice() {
  let parser = Parser::new(default_options());
  // Input with an unexpected delimiter '::' in the arguments section within a slice segment
  let input: &[&str] = &[r#"cmd"#, r#"arg1"#, r#"::"#, r#"arg2"#]; // '::' is unexpected after 'arg1'
  let result = parser.parse_slice(input);

  assert!(result.is_err(), "parse_slice unexpectedly succeeded");
  let err = result.unwrap_err();

  assert!(matches!(err.kind, ErrorKind::Syntax(_)));
  assert!(err.to_string().contains("Unexpected delimiter '::' in arguments section"));

  // Expected location of the unexpected delimiter '::' in segment 2
  // '::' is the item at index 2 in the input slice.
  // The location should cover the entire '::' item in segment 2.
  let expected_location = Some(SourceLocation::SliceSegment { segment_index: 2, start_in_segment: 0, end_in_segment: 2 });
  assert_eq!(err.location, expected_location, "Incorrect error location for unexpected delimiter in slice");
}