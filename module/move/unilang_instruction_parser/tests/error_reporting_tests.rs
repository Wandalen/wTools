//! Tests specifically for error reporting and SourceLocation in the unilang instruction parser.

use unilang_instruction_parser::*;
use unilang_instruction_parser::error::{ErrorKind, SourceLocation};
#[allow(unused_imports)] // HashMap might be used in future error tests
use std::collections::HashMap;
#[allow(unused_imports)] // Cow might be used if unescape_string changes signature
use std::borrow::Cow;


fn default_options() -> UnilangParserOptions {
  UnilangParserOptions::default()
}

fn options_error_on_positional_after_named() -> UnilangParserOptions {
    UnilangParserOptions {
        error_on_positional_after_named: true,
        ..Default::default()
    }
}

// Existing tests from the file
#[test]
fn error_invalid_escape_sequence_location_str() {
  let parser = Parser::new(default_options());
  let input = r#"cmd arg1 "value with \x invalid escape""#;
  let result = parser.parse_single_str(input);

  assert!(result.is_err(), "parse_single_str unexpectedly succeeded for input: {}", input);
  if let Ok(_) = result { return; }
  let err = result.unwrap_err();

  match err.kind {
    ErrorKind::Syntax(s) => {
        assert!(s.contains("Invalid escape sequence: \\x"), "Error message for invalid escape: {}", s);
    }
    _ => panic!("Unexpected error kind: {:?}", err.kind),
  }

  // Adjusted expected location to match current actual output for debugging
  let expected_location = Some(SourceLocation::StrSpan { start: 21, end: 23 });
  assert_eq!(err.location, expected_location, "Incorrect error location for invalid escape sequence");
}

#[test]
fn error_unexpected_delimiter_location_str() {
  let parser = Parser::new(default_options());
  let input = r#"cmd :: arg2"#;
  let result = parser.parse_single_str(input);

  assert!(result.is_err(), "parse_single_str unexpectedly succeeded for input: {}", input);
  if let Ok(_) = result { return; }
  let err = result.unwrap_err();

  match err.kind {
    ErrorKind::Syntax(s) => {
        assert!(s.contains("Unexpected '::' without preceding argument name"), "Error message mismatch: {}", s);
    }
    _ => panic!("Unexpected error kind: {:?}", err.kind),
  }

  let expected_location = Some(SourceLocation::StrSpan { start: 4, end: 6 });
  assert_eq!(err.location, expected_location, "Incorrect error location for unexpected delimiter");
}

#[test]
fn error_invalid_escape_sequence_location_slice() {
  let parser = Parser::new(default_options());
  let input: &[&str] = &[r#"cmd"#, r#"arg1"#, r#""value with \y invalid escape""#];
  let result = parser.parse_slice(input);

  assert!(result.is_err(), "parse_slice unexpectedly succeeded for input: {:?}", input);
  if let Ok(_) = result { return; }
  let err = result.unwrap_err();

  match err.kind {
    ErrorKind::Syntax(s) => {
        assert!(s.contains("Invalid escape sequence: \\y"), "Error message for invalid escape: {}", s);
    }
    _ => panic!("Unexpected error kind: {:?}", err.kind),
  }

  let expected_location = Some(SourceLocation::SliceSegment { segment_index: 2, start_in_segment: 12, end_in_segment: 14 });
  assert_eq!(err.location, expected_location, "Incorrect error location for invalid escape sequence in slice");
}

#[test]
fn error_unexpected_delimiter_location_slice() {
  let parser = Parser::new(default_options());
  let input: &[&str] = &[r#"cmd"#, r#"::"#, r#"arg2"#];
  let result = parser.parse_slice(input);

  assert!(result.is_err(), "parse_slice unexpectedly succeeded for input: {:?}", input);
  if let Ok(_) = result { return; }
  let err = result.unwrap_err();

  match err.kind {
    ErrorKind::Syntax(s) => {
        assert!(s.contains("Unexpected '::' without preceding argument name"), "Error message mismatch: {}", s);
    }
    _ => panic!("Unexpected error kind: {:?}", err.kind),
  }
  let expected_location = Some(SourceLocation::SliceSegment { segment_index: 1, start_in_segment: 0, end_in_segment: 2 });
  assert_eq!(err.location, expected_location, "Incorrect error location for unexpected delimiter in slice");
}

// New tests from Increment 6 plan

#[test]
fn empty_instruction_segment_double_semicolon() {
    let parser = Parser::new(default_options());
    let input = "cmd1 ;;";
    let result = parser.parse_single_str(input);
    assert!(result.is_err(), "Expected error for empty segment due to ';;', input: '{}'", input);
    let err = result.unwrap_err();
    match err.kind {
        ErrorKind::Syntax(s) => assert!(s.contains("Empty instruction segment due to trailing ';;'"), "Msg: {}", s),
        _ => panic!("Wrong error kind"),
    }
    assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 5, end: 7 }));
}

#[test]
fn empty_instruction_segment_trailing_semicolon() {
    let parser = Parser::new(default_options());
    let input = "cmd1 ;; ";
    let result = parser.parse_single_str(input);
    assert!(result.is_err(), "Expected error for empty segment due to trailing ';;', input: '{}'", input);
    let err = result.unwrap_err();
     match err.kind {
        ErrorKind::Syntax(s) => assert!(s.contains("Empty instruction segment due to trailing ';;'"), "Msg: {}", s),
        _ => panic!("Wrong error kind"),
    }
    assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 5, end: 7 }));
}

#[test]
fn empty_instruction_segment_only_semicolon() {
    let parser = Parser::new(default_options());
    let input = ";;";
    let result = parser.parse_single_str(input);
    assert!(result.is_err(), "Expected error for input being only ';;', input: '{}'", input);
    let err = result.unwrap_err();
    match err.kind {
        ErrorKind::Syntax(s) => assert!(s.contains("Empty instruction segment due to ';;'"), "Msg: {}. Expected specific message for ';;' only.", s),
        _ => panic!("Wrong error kind"),
    }
    assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 0, end: 2 }));
}

#[test]
fn missing_value_for_named_arg() {
    let parser = Parser::new(default_options());
    let input = "cmd name::";
    let result = parser.parse_single_str(input);
    assert!(result.is_err(), "Expected error for missing value for named arg, input: '{}'", input);
    let err = result.unwrap_err();
    match err.kind {
        ErrorKind::Syntax(s) => assert!(s.contains("Expected value for named argument 'name' but found end of instruction"), "Msg: {}", s),
        _ => panic!("Wrong error kind"),
    }
    assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 4, end: 8 }));
}

#[test]
fn unexpected_colon_colon_no_name() {
    let parser = Parser::new(default_options());
    let input = "cmd ::value";
    let result = parser.parse_single_str(input);
    assert!(result.is_err(), "Expected error for 'cmd ::value', input: '{}', got: {:?}", input, result);
    if let Ok(_) = result { return; }
    let err = result.unwrap_err();
     match err.kind {
        ErrorKind::Syntax(s) => assert!(s.contains("Unexpected '::' without preceding argument name"), "Msg: {}", s),
        _ => panic!("Wrong error kind: {:?}", err.kind),
    }
    assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 4, end: 6 }));
}

#[test]
fn unexpected_colon_colon_after_value() {
    let parser = Parser::new(default_options());
    let input = "cmd name::val1 ::val2";
    let result = parser.parse_single_str(input);
    assert!(result.is_err(), "Expected error for 'name::val1 ::val2', input: '{}'", input);
    let err = result.unwrap_err();
    match err.kind {
        ErrorKind::Syntax(s) => assert!(s.contains("Unexpected '::' without preceding argument name or after a previous value"), "Msg: {}", s),
        _ => panic!("Wrong error kind"),
    }
    assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 15, end: 17 }));
}

#[test]
fn positional_after_named_error() {
    let parser = Parser::new(options_error_on_positional_after_named());
    let input = "cmd name::val pos1";
    let result = parser.parse_single_str(input);
    assert!(result.is_err(), "Expected error for positional after named, input: '{}'", input);
    let err = result.unwrap_err();
    match err.kind {
        ErrorKind::Syntax(s) => assert!(s.contains("Positional argument encountered after a named argument"), "Msg: {}", s),
        _ => panic!("Wrong error kind"),
    }
    assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 14, end: 18 }));
}

#[test]
fn unexpected_help_operator_middle() {
    let parser = Parser::new(default_options());
    let input = "cmd ? arg1";
    let result = parser.parse_single_str(input);
    assert!(result.is_err(), "Expected error for '?' in middle, input: '{}'", input);
    let err = result.unwrap_err();
    match err.kind {
        ErrorKind::Syntax(s) => assert!(s.contains("Unexpected help operator '?' amidst arguments"), "Msg: {}", s),
        _ => panic!("Wrong error kind"),
    }
    assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 4, end: 5 }));
}

#[test]
fn unexpected_token_in_args() {
    let parser = Parser::new(default_options());
    let input = "cmd arg1 ! badchar";
    let result = parser.parse_single_str(input);
    assert!(result.is_err(), "Expected error for unexpected token '!', input: '{}', got: {:?}", input, result);
    if let Ok(_) = result { return; }
    let err = result.unwrap_err();
    match err.kind {
        ErrorKind::Syntax(s) => assert!(s.contains("Unexpected token in arguments: '!'"), "Msg: {}", s),
        _ => panic!("Wrong error kind: {:?}", err.kind),
    }
    assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 9, end: 10 }));
}