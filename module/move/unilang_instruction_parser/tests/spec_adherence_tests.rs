//! Tests for syntactic analysis adherence to `spec.md`.

use unilang_instruction_parser::*;
use unilang_instruction_parser::error::ErrorKind;
use unilang_instruction_parser::UnilangParserOptions;

/// Test Combination: TM2.1
/// Command path with multiple dot-separated segments followed by a positional argument.
#[test]
fn tm2_1_multi_segment_path_with_positional_arg() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd.sub.another arg";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string(), "sub".to_string(), "another".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "arg".to_string());
    assert!(instruction.named_arguments.is_empty());
    assert!(!instruction.help_requested);
}

/// Test Combination: TM2.2
/// Command path ending with `::` (named argument).
#[test]
fn tm2_2_command_path_ends_with_named_arg() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd arg::val";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.positional_arguments.is_empty());
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("arg").unwrap().value, "val".to_string());
    assert!(!instruction.help_requested);
}

/// Test Combination: TM2.3
/// Command path ending with a correctly quoted string.
#[test]
fn tm2_3_command_path_ends_with_quoted_string() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd \"quoted_arg\"";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "quoted_arg".to_string());
    assert!(instruction.named_arguments.is_empty());
    assert!(!instruction.help_requested);
}

/// Test Combination: TM2.4
/// Command path ending with `#` (comment operator).
#[test]
fn tm2_4_command_path_ends_with_comment_operator() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd #comment";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for input '{}', but got Ok: {:?}", input, result.ok());
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::Syntax("Unexpected token '#' in arguments".to_string()));
    }
}

/// Test Combination: TM2.5
/// Trailing dot after command path.
#[test]
fn tm2_5_trailing_dot_after_command_path() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd.";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for input '{}', but got Ok: {:?}", input, result.ok());
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::Syntax("Command path cannot end with a '.'".to_string()));
    }
}

/// Test Combination: TM2.6
/// Named argument followed by `?`.
#[test]
fn tm2_6_named_arg_followed_by_help_operator() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd name::val ?";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.positional_arguments.is_empty());
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, "val".to_string());
    assert!(instruction.help_requested);
}

/// Test Combination: TM2.7
/// Help operator followed by other tokens.
#[test]
fn tm2_7_help_operator_followed_by_other_tokens() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd ? arg";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for input '{}', but got Ok: {:?}", input, result.ok());
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::Syntax("Help operator '?' must be the last token".to_string()));
    }
}

/// Test Combination: TM2.8
/// Named argument with a simple quoted value (no escapes).
#[test]
fn tm2_8_named_arg_with_simple_quoted_value() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd name::\"value with spaces\"";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.positional_arguments.is_empty());
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, "value with spaces".to_string());
    assert!(!instruction.help_requested);
}

/// Test Combination: TM2.9
/// Named argument with quoted value containing `::`.
#[test]
fn tm2_9_named_arg_with_quoted_value_containing_double_colon() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd msg::\"DEPRECATED::message\"";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.positional_arguments.is_empty());
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("msg").unwrap().value, "DEPRECATED::message".to_string());
    assert!(!instruction.help_requested);
}

/// Test Combination: TM2.10
/// Multiple named arguments with simple quoted values.
#[test]
fn tm2_10_multiple_named_args_with_simple_quoted_values() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd name1::\"val1\" name2::\"val2\"";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.positional_arguments.is_empty());
    assert_eq!(instruction.named_arguments.len(), 2);
    assert_eq!(instruction.named_arguments.get("name1").unwrap().value, "val1".to_string());
    assert_eq!(instruction.named_arguments.get("name2").unwrap().value, "val2".to_string());
    assert!(!instruction.help_requested);
}

/// Test Combination: TM2.11
/// Named argument with comma-separated value (syntactically, it's just a string).
#[test]
fn tm2_11_named_arg_with_comma_separated_value() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd tags::dev,rust,unilang";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.positional_arguments.is_empty());
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("tags").unwrap().value, "dev,rust,unilang".to_string());
    assert!(!instruction.help_requested);
}

/// Test Combination: TM2.12
/// Named argument with key-value pair string (syntactically, it's just a string).
#[test]
fn tm2_12_named_arg_with_key_value_pair_string() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd headers::Content-Type=application/json,Auth-Token=xyz";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.positional_arguments.is_empty());
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("headers").unwrap().value, "Content-Type=application/json,Auth-Token=xyz".to_string());
    assert!(!instruction.help_requested);
}