//! ## Test Matrix for Error Reporting
//!
//! This matrix details test cases specifically designed to verify the parser's error reporting
//! capabilities, including the correct identification of error kinds and source locations.
//!
//! **Test Factors:**
//! - Error Type: Invalid Escape, Unexpected Delimiter, Empty Segment, Missing Value, Unexpected Token, Positional After Named, Unexpected Help Operator
//! - Input Format: Correct, Malformed
//! - Location: Start, Middle, End of instruction
//! - Parser Options: `error_on_positional_after_named` (true/false)
//!
//! ---
//!
//! **Test Combinations:**
//!
//! | ID | Aspect Tested | Input String | Error Type | Location | Parser Options (`pos_after_named`) | Expected Error Kind | Expected Location (start, end) | Expected Message Contains |
//! |---|---|---|---|---|---|---|---|---|
//! | T3.1 | Invalid escape sequence | `cmd arg1 "value with \x invalid escape"` | Invalid Escape | Middle | `(false)` | N/A (parsed as literal) | N/A | N/A |
//! | T3.2 | Unexpected delimiter `::` | `cmd :: arg2` | Unexpected Delimiter | Middle | `(false)` | `Syntax` | `(4, 6)` | `Unexpected token '::' in arguments` |
//! | T3.3 | Empty instruction segment (trailing `;;`) | `cmd1 ;;` | Empty Segment | End | `(false)` | `TrailingDelimiter` | `(5, 7)` | N/A |
//! | T3.4 | Empty instruction segment (trailing `;; `) | `cmd1 ;; ` | Empty Segment | End | `(false)` | `TrailingDelimiter` | `(5, 7)` | N/A |
//! | T3.5 | Empty instruction segment (only `;;`) | `;;` | Empty Segment | Start | `(false)` | `EmptyInstructionSegment` | `(0, 2)` | N/A |
//! | T3.6 | Missing value for named arg | `cmd name::` | Missing Value | End | `(false)` | `Syntax` | `(4, 8)` | `Expected value for named argument 'name' but found end of instruction` |
//! | T3.7 | Unexpected `::` (no name) | `cmd ::value` | Unexpected Token | Middle | `(false)` | `Syntax` | `(4, 6)` | `Unexpected token '::' in arguments` |
//! | T3.8 | Unexpected `::` (after value) | `cmd name::val1 ::val2` | Unexpected Token | Middle | `(false)` | `Syntax` | `(15, 17)` | `Unexpected token '::' in arguments` |
//! | T3.9 | Positional after named (error) | `cmd name::val pos1` | Positional After Named | Middle | `(true)` | `Syntax` | `(14, 18)` | `Positional argument after named argument` |
//! | T3.10 | Unexpected help operator in middle | `cmd ? arg1` | Unexpected Help Operator | Middle | `(false)` | `Syntax` | `(4, 5)` | `Help operator '?' must be the last token` |
//! | T3.11 | Unexpected token `!` in args | `cmd arg1 ! badchar` | Unexpected Token | Middle | `(false)` | `Syntax` | `(9, 10)` | `Unexpected token '!' in arguments` |
use unilang_parser::*;
use unilang_parser::error::{ErrorKind, SourceLocation};
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

/// Tests error reporting for an invalid escape sequence in a string.
/// Test Combination: T3.1
#[test]
fn error_invalid_escape_sequence_location_str() {
  let parser = Parser::new(UnilangParserOptions::default());
  let input = r#"cmd arg1 "value with \x invalid escape""#;
  let result = parser.parse_single_instruction(input);

  assert!(
    result.is_ok(),
    "parse_single_instruction unexpectedly failed for input: {input}"
  );
  let instruction = result.unwrap();
  assert_eq!(instruction.positional_arguments[0].value, "arg1".to_string());
  assert_eq!(
    instruction.positional_arguments[1].value,
    "value with \\x invalid escape".to_string()
  );
}

/// Tests error reporting for an unexpected delimiter (::) in a string.
/// Test Combination: T3.2
#[test]
fn error_unexpected_delimiter_location_str() {
  let parser = Parser::new(UnilangParserOptions::default());
  let input = r"cmd :: arg2";
  let result = parser.parse_single_instruction(input);

  assert!(
    result.is_err(),
    "parse_single_instruction failed for input: '{}', error: {:?}",
    input,
    result.err()
  );
  if let Err(e) = result {
    assert_eq!(
      e.kind,
      ErrorKind::Syntax("Unexpected token '::' in arguments".to_string()),
      "ErrorKind mismatch: {:?}",
      e.kind
    );
    assert_eq!(e.location, Some(SourceLocation::StrSpan { start: 4, end: 6 }));
  }
}

/// Tests error reporting for an empty instruction segment caused by a double semicolon.
/// Test Combination: T3.3
#[test]
fn empty_instruction_segment_double_semicolon() {
  let parser = Parser::new(UnilangParserOptions::default());
  let input = "cmd1 ;;";
  let result = parser.parse_multiple_instructions(input); // Changed to parse_multiple_instructions
  assert!(
    result.is_err(),
    "Expected error for empty segment due to ';;', input: '{input}'"
  );
  let err = result.unwrap_err();
  assert_eq!(
    err.kind,
    ErrorKind::TrailingDelimiter,
    "Expected TrailingDelimiter error, but got: {:?}",
    err.kind
  ); // Changed expected error kind
  assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 5, end: 7 }));
}

/// Tests error reporting for an empty instruction segment caused by a trailing semicolon with whitespace.
/// Test Combination: T3.4
#[test]
fn empty_instruction_segment_trailing_semicolon() {
  let parser = Parser::new(UnilangParserOptions::default());
  let input = "cmd1 ;; ";
  let result = parser.parse_multiple_instructions(input);
  assert!(
    result.is_err(),
    "Expected error for empty segment due to trailing ';;', input: '{input}'"
  );
  let err = result.unwrap_err();
  assert_eq!(
    err.kind,
    ErrorKind::TrailingDelimiter,
    "Expected TrailingDelimiter error, but got: {:?}",
    err.kind
  );
  assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 5, end: 7 }));
}

/// Tests error reporting for an input consisting only of a double semicolon.
/// Test Combination: T3.5
#[test]
fn empty_instruction_segment_only_semicolon() {
  let parser = Parser::new(UnilangParserOptions::default());
  let input = ";;";
  let result = parser.parse_multiple_instructions(input);
  assert!(
    result.is_err(),
    "Expected error for input being only ';;', input: '{input}'"
  );
  let err = result.unwrap_err();
  assert_eq!(
    err.kind,
    ErrorKind::EmptyInstructionSegment,
    "Expected EmptyInstructionSegment error, but got: {:?}",
    err.kind
  );
  assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 0, end: 2 }));
}

/// Tests error reporting for a named argument with a missing value.
/// Test Combination: T3.6
#[test]
fn missing_value_for_named_arg() {
  let parser = Parser::new(UnilangParserOptions::default());
  let input = "cmd name::";
  let result = parser.parse_single_instruction(input);
  assert!(
    result.is_err(),
    "Expected error for missing value for named arg, input: '{input}'"
  );
  let err = result.unwrap_err();
  match err.kind {
    ErrorKind::Syntax(s) => assert!(
      s.contains("Expected value for named argument 'name' but found end of instruction"),
      "Msg: {s}"
    ),
    _ => panic!("Expected Syntax error, but got: {:?}", err.kind),
  }
  assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 4, end: 8 }));
}

/// Tests error reporting for an unexpected `::` token without a preceding name.
/// Test Combination: T3.7
#[test]
fn unexpected_colon_colon_no_name() {
  let parser = Parser::new(UnilangParserOptions::default());
  let input = "cmd ::value";
  let result = parser.parse_single_instruction(input);
  assert!(
    result.is_err(),
    "Expected error for 'cmd ::value', input: '{}', got: {:?}",
    input,
    result.ok()
  );
  if let Err(e) = result {
    assert_eq!(
      e.kind,
      ErrorKind::Syntax("Unexpected token '::' in arguments".to_string()),
      "ErrorKind mismatch: {:?}",
      e.kind
    );
    assert_eq!(e.location, Some(SourceLocation::StrSpan { start: 4, end: 6 }));
  }
}

/// Tests error reporting for an unexpected `::` token appearing after a value.
/// Test Combination: T3.8
#[test]
fn unexpected_colon_colon_after_value() {
  let parser = Parser::new(UnilangParserOptions::default());
  let input = "cmd name::val1 ::val2";
  let result = parser.parse_single_instruction(input);
  assert!(result.is_err(), "Expected error for 'name::val1 ::val2', input: '{input}'");
  let err = result.unwrap_err();
  assert_eq!(
    err.kind,
    ErrorKind::Syntax("Unexpected token '::' in arguments".to_string()),
    "ErrorKind mismatch: {:?}",
    err.kind
  );
  assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 15, end: 17 }));
}

/// Tests error reporting when a positional argument appears after a named argument and the option is set.
/// Test Combination: T3.9
#[test]
fn positional_after_named_error() {
  let parser = Parser::new(options_error_on_positional_after_named());
  let input = "cmd name::val pos1";
  let result = parser.parse_single_instruction(input);
  assert!(
    result.is_err(),
    "Expected error for positional after named, input: '{input}'"
  );
  let err = result.unwrap_err();
  match err.kind {
    ErrorKind::Syntax(s) => assert!(s.contains("Positional argument after named argument"), "Msg: {s}"), // Removed .to_string()
    _ => panic!("Expected Syntax error, but got: {:?}", err.kind),
  }
  assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 14, end: 18 }));
}

/// Tests error reporting when the help operator `?` appears in the middle of an instruction.
/// Test Combination: T3.10
#[test]
fn unexpected_help_operator_middle() {
  let parser = Parser::new(UnilangParserOptions::default());
  let input = "cmd ? arg1";
  let result = parser.parse_single_instruction(input);
  assert!(result.is_err(), "Expected error for '?' in middle, input: '{input}'");
  let err = result.unwrap_err();
  assert_eq!(
    err.kind,
    ErrorKind::Syntax("Help operator '?' must be the last token".to_string()),
    "ErrorKind mismatch: {:?}",
    err.kind
  );
  assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 4, end: 5 })); // Adjusted location
}

/// Tests error reporting for an unexpected token `!` in arguments.
/// Test Combination: T3.11
#[test]
fn unexpected_token_in_args() {
  let parser = Parser::new(UnilangParserOptions::default());
  let input = "cmd arg1 ! badchar";
  let result = parser.parse_single_instruction(input);
  assert!(
    result.is_err(),
    "Expected error for unexpected token '!', input: '{}', got: {:?}",
    input,
    result.ok()
  );
  if result.is_ok() {
    return;
  }
  let err = result.unwrap_err();
  assert_eq!(
    err.kind,
    ErrorKind::Syntax("Unexpected token '!' in arguments".to_string()),
    "ErrorKind mismatch: {:?}",
    err.kind
  );
  assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 9, end: 10 }));
}
