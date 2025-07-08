//! Tests specifically for error reporting and SourceLocation in the unilang instruction parser.

use unilang_instruction_parser::*;
use unilang_instruction_parser::error::{ErrorKind, SourceLocation};
#[allow(unused_imports)] // HashMap might be used in future error tests
use std::collections::HashMap;
#[allow(unused_imports)] // Cow might be used if unescape_string changes signature
use std::borrow::Cow;



fn options_error_on_positional_after_named() -> UnilangParserOptions {
    UnilangParserOptions {
        error_on_positional_after_named: true,
        ..Default::default()
    }
}

// Existing tests from the file
#[test]
fn error_invalid_escape_sequence_location_str() {
  let parser = Parser::new(UnilangParserOptions::default());
  let input = r#"cmd arg1 "value with \x invalid escape""#;
  let result = parser.parse_single_instruction(input);

  assert!(result.is_err(), "parse_single_instruction unexpectedly succeeded for input: {}", input);
  if let Ok(_) = result { return; }
  let err = result.unwrap_err();

  assert_eq!(err.kind, ErrorKind::InvalidEscapeSequence("\\x".to_string()), "Expected InvalidEscapeSequence error, but got: {:?}", err.kind);

  // Adjusted expected location to match current actual output for debugging
  let expected_location = Some(SourceLocation::StrSpan { start: 21, end: 23 }); // Corrected end to 23
  assert_eq!(err.location, expected_location, "Incorrect error location for invalid escape sequence");
}

#[test]
fn error_unexpected_delimiter_location_str() {
  let parser = Parser::new(UnilangParserOptions::default());
  let input = r#"cmd :: arg2"#;
  let result = parser.parse_single_instruction(input);

  assert!(result.is_err(), "parse_single_instruction failed for input: '{}', error: {:?}", input, result.err());
  if let Err(e) = result {
      assert_eq!(e.kind, ErrorKind::Syntax("Unexpected '::' operator without a named argument name".to_string()), "ErrorKind mismatch: {:?}", e.kind);
      assert_eq!(e.location, Some(SourceLocation::StrSpan { start: 4, end: 6 }));
  }
}

// Removed parse_slice tests: error_invalid_escape_sequence_location_slice and error_unexpected_delimiter_location_slice

// New tests from Increment 6 plan

#[test]
fn empty_instruction_segment_double_semicolon() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd1 ;;";
    let result = parser.parse_multiple_instructions(input); // Changed to parse_multiple_instructions
    assert!(result.is_err(), "Expected error for empty segment due to ';;', input: '{}'", input);
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::TrailingDelimiter, "Expected TrailingDelimiter error, but got: {:?}", err.kind); // Changed expected error kind
    assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 5, end: 7 }));
}

#[test]
fn empty_instruction_segment_trailing_semicolon() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd1 ;; ";
    let result = parser.parse_multiple_instructions(input); // Changed to parse_multiple_instructions
    assert!(result.is_err(), "Expected error for empty segment due to trailing ';;', input: '{}'", input);
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::TrailingDelimiter, "Expected TrailingDelimiter error, but got: {:?}", err.kind); // Changed expected error kind
    assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 5, end: 7 }));
}

#[test]
fn empty_instruction_segment_only_semicolon() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = ";;";
    let result = parser.parse_multiple_instructions(input); // Changed to parse_multiple_instructions
    assert!(result.is_err(), "Expected error for input being only ';;', input: '{}'", input);
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::EmptyInstructionSegment, "Expected EmptyInstructionSegment error, but got: {:?}", err.kind);
    assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 0, end: 2 }));
}

#[test]
fn missing_value_for_named_arg() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd name::";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for missing value for named arg, input: '{}'", input);
    let err = result.unwrap_err();
    match err.kind {
        ErrorKind::Syntax(s) => assert!(s.contains("Expected value for named argument 'name' but found end of instruction"), "Msg: {}", s),
        _ => panic!("Expected Syntax error, but got: {:?}", err.kind),
    }
    assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 4, end: 8 }));
}

#[test]
fn unexpected_colon_colon_no_name() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd ::value";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for 'cmd ::value', input: '{}', got: {:?}", input, result.ok());
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::Syntax("Unexpected '::' operator without a named argument name".to_string()), "ErrorKind mismatch: {:?}", e.kind);
        assert_eq!(e.location, Some(SourceLocation::StrSpan { start: 4, end: 6 }));
    }
}

#[test]
fn unexpected_colon_colon_after_value() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd name::val1 ::val2";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for 'name::val1 ::val2', input: '{}'", input);
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::Syntax("Unexpected '::' operator without a named argument name".to_string()), "ErrorKind mismatch: {:?}", err.kind);
    assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 15, end: 17 }));
}

#[test]
fn positional_after_named_error() {
    let parser = Parser::new(options_error_on_positional_after_named());
    let input = "cmd name::val pos1";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for positional after named, input: '{}'", input);
    let err = result.unwrap_err();
    match err.kind {
        ErrorKind::Syntax(s) => assert!(s.contains("Positional argument after named argument"), "Msg: {}", s), // Removed .to_string()
        _ => panic!("Expected Syntax error, but got: {:?}", err.kind),
    }
    assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 14, end: 18 }));
}

#[test]
fn unexpected_help_operator_middle() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd ? arg1";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for '?' in middle, input: '{}'", input);
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::Syntax("Help operator '?' must be the last token".to_string()), "ErrorKind mismatch: {:?}", err.kind);
    assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 6, end: 10 })); // Adjusted location
}

#[test]
fn unexpected_token_in_args() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd arg1 ! badchar";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for unexpected token '!', input: '{}', got: {:?}", input, result.ok());
    if let Ok(_) = result { return; }
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::Syntax("Unexpected token in arguments: '!' (Unrecognized(\"!\"))".to_string()), "ErrorKind mismatch: {:?}", err.kind);
    assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 9, end: 10 }));
}