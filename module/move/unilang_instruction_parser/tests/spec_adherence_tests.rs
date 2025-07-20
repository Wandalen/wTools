//! ## Test Matrix for Spec Adherence
//!
//! This matrix details test cases specifically designed to verify the parser's adherence to the
//! Unilang specification (`spec.md`), covering various command path formats, argument types,
//! and help operator usage.
//!
//! **Test Factors:**
//! - Command Path: Multi-segment, Ends with named arg, Ends with quoted string, Ends with comment operator, Trailing dot
//! - Arguments: Positional, Named, None
//! - Help Operator: Present, Followed by other tokens
//! - Named Argument Value: Simple quoted, Quoted with `::`, Comma-separated, Key-value pair string
//!
//! ---
//!
//! **Test Combinations:**
//!
//! | ID | Aspect Tested | Input String | Command Path Format | Arguments | Help Operator | Named Arg Value Type | Expected Behavior |
//! |---|---|---|---|---|---|---|---|
//! | T4.1 | Multi-segment path with positional arg | `cmd.sub.another arg` | Multi-segment | Positional | Absent | N/A | Command `cmd.sub.another`, Positional `arg` |
//! | T4.2 | Command path ends with named arg | `cmd arg::val` | Ends with named arg | Named | Absent | Simple | Command `cmd`, Named `arg::val` |
//! | T4.3 | Command path ends with quoted string | `cmd "quoted_arg"` | Ends with quoted string | Positional | Absent | N/A | Command `cmd`, Positional `"quoted_arg"` |
//! | T4.4 | Command path ends with comment operator | `cmd #comment` | Ends with comment operator | N/A | Absent | N/A | Error: Unexpected token '#' |
//! | T4.5 | Trailing dot after command path | `cmd.` | Trailing dot | N/A | Absent | N/A | Error: Command path cannot end with a '.' |
//! | T4.6 | Named arg followed by help operator | `cmd name::val ?` | N/A | Named | Present | Simple | Command `cmd`, Named `name::val`, Help requested |
//! | T4.7 | Help operator followed by other tokens | `cmd ? arg` | N/A | Positional | Followed by other tokens | N/A | Error: Help operator '?' must be the last token |
//! | T4.8 | Named arg with simple quoted value | `cmd name::"value with spaces"` | N/A | Named | Absent | Simple Quoted | Command `cmd`, Named `name::value with spaces` |
//! | T4.9 | Named arg with quoted value containing `::` | `cmd msg::"DEPRECATED::message"` | N/A | Named | Absent | Quoted with `::` | Command `cmd`, Named `msg::DEPRECATED::message` |
//! | T4.10 | Multiple named args with simple quoted values | `cmd name1::"val1" name2::"val2"` | N/A | Named | Absent | Simple Quoted | Command `cmd`, Named `name1::val1`, `name2::val2` |
//! | T4.11 | Named arg with comma-separated value | `cmd tags::dev,rust,unilang` | N/A | Named | Absent | Comma-separated | Command `cmd`, Named `tags::dev,rust,unilang` |
//! | T4.12 | Named arg with key-value pair string | `cmd headers::Content-Type=application/json,Auth-Token=xyz` | N/A | Named | Absent | Key-value pair string | Command `cmd`, Named `headers::Content-Type=application/json,Auth-Token=xyz` |
//! | S6.1 | R0, R1 | `  cmd.sub  arg1  ` | Single | Multi-segment | Positional | Identifier | None | Correct | Leading/Trailing, Internal | None | `(false, false)` | `cmd.sub`, `arg1` (whitespace ignored) |
//! | S6.2 | R0, R5.1 | `cmd "val with spaces"` | Single | Simple | Positional | Quoted String | None | Correct | In quotes | None | `(false, false)` | `cmd`, `"val with spaces"` |
//! | S6.3 | R1, R2 | `cmd.sub.action arg1` | Single | Multi-segment | Positional | Identifier | None | Correct | None | None | `(false, false)` | `cmd.sub.action`, `arg1` |
//! | S6.4 | R1, R2, R5.2 | `cmd.sub name::val` | Single | Multi-segment | Named | Identifier | `::` | Correct | None | None | `(false, false)` | `cmd.sub`, `name::val` |
//! | S6.5 | R3.1 | `.cmd arg` | Single | Leading Dot | Positional | Identifier | None | Correct | None | None | `(false, false)` | `cmd`, `arg` (leading dot consumed) |
//! | S6.6 | R3.3 | `cmd.` | Single | Trailing Dot | None | N/A | None | Incorrect | None | Syntax Error | `(false, false)` | Error: Trailing dot |
//! | S6.7 | R3.4 | `cmd..sub` | Single | Consecutive Dots | None | N/A | None | Incorrect | None | Syntax Error | `(false, false)` | Error: Consecutive dots |
//! | S6.8 | R4 | `cmd ?` | Single | Simple | None | N/A | `?` | Correct (last) | None | None | `(false, false)` | `cmd`, Help requested |
//! | S6.9 | R4, R5.2 | `cmd name::val ?` | Single | Simple | Named | Identifier | `?` | Correct (last) | None | None | `(false, false)` | `cmd`, `name::val`, Help requested |
//! | S6.10 | R4 | `cmd ? arg` | Single | Simple | Positional | Identifier | `?` | Incorrect (not last) | None | Syntax Error | `(false, false)` | Error: `?` not last |
//! | S6.11 | R5.1 | `cmd pos1 pos2` | Single | Simple | Positional | Identifier | None | Correct | None | None | `(false, false)` | `cmd`, `pos1`, `pos2` |
//! | S6.12 | R5.2 | `cmd key::val` | Single | Simple | Named | Identifier | `::` | Correct | None | None | `(false, false)` | `cmd`, `key::val` |
//! | S6.13 | R5.2 | `cmd key::"val with spaces"` | Single | Simple | Named | Quoted String | `::` | Correct | In quotes | None | `(false, false)` | `cmd`, `key::"val with spaces"` |
//! | S6.14 | R5.3 | `cmd name::val pos1` | Single | Simple | Mixed | Identifier | `::` | Correct | None | None | `(false, false)` | `cmd`, `name::val`, `pos1` (allowed) |
//! | S6.15 | R5.3 (Error) | `cmd name::val pos1` | Single | Simple | Mixed | Identifier | `::` | Correct | None | Positional after named | `(true, false)` | Error: Positional after named |
//! | S6.16 | R5.4 | `cmd name::val1 name::val2` | Single | Simple | Named | Identifier | `::` | Correct | None | None | `(false, false)` | `cmd`, `name::val2` (last wins) |
//! | S6.17 | R5.4 (Error) | `cmd name::val1 name::val2` | Single | Simple | Named | Identifier | `::` | Correct | None | Duplicate named arg | `(false, true)` | Error: Duplicate named arg |
//! | S6.18 | Multi-Instruction | `cmd1 arg1 ;; cmd2 name::val` | Multi-Instruction | Simple | Positional, Named | Identifier | `;;` | Correct | None | None | `(false, false)` | Two instructions parsed |
//! | S6.19 | Multi-Instruction (Empty Segment) | `cmd1 ;;;; cmd2` | Multi-Instruction | N/A | N/A | N/A | `;;` | Incorrect | None | Empty Instruction Segment | `(false, false)` | Error: Empty instruction segment |
//! | S6.20 | Multi-Instruction (Trailing Delimiter) | `cmd1 ;;` | Multi-Instruction | N/A | N/A | N/A | `;;` | Incorrect | None | Trailing Delimiter | `(false, false)` | Error: Trailing delimiter |
//! | S6.21 | R2 (Transition by non-identifier) | `cmd !arg` | Single | Simple | Positional | N/A | `!` | Correct | None | Syntax Error | `(false, false)` | Error: Unexpected token `!` |
//! | S6.22 | R2 (Transition by quoted string) | `cmd "arg"` | Single | Simple | Positional | Quoted String | None | Correct | None | None | `(false, false)` | `cmd`, `"arg"` |
//! | S6.23 | R2 (Transition by help operator) | `cmd ?` | Single | Simple | None | N/A | `?` | Correct | None | None | `(false, false)` | `cmd`, Help requested |
//! | S6.24 | R5.2 (Value with `::`) | `cmd msg::"DEPRECATED::message"` | Single | Simple | Named | Quoted String | `::` | Correct | In quotes | None | `(false, false)` | `cmd`, `msg::DEPRECATED::message` |
//! | S6.25 | R5.2 (Value with commas) | `cmd tags::dev,rust,unilang` | Single | Simple | Named | Identifier | `::` | Correct | None | None | `(false, false)` | `cmd`, `tags::dev,rust,unilang` |
//! | S6.26 | R5.2 (Value with key-value pair) | `cmd headers::Content-Type=application/json,Auth-Token=xyz` | Single | Simple | Named | Identifier | `::` | Correct | None | None | `(false, false)` | `cmd`, `headers::Content-Type=application/json,Auth-Token=xyz` |
//! | S6.27 | R1 (Whitespace around dot) | `cmd . sub` | Single | Multi-segment | None | N/A | `.` | Correct | Around dot | None | `(false, false)` | `cmd.sub` |
//! | S6.28 | R1 (Invalid identifier segment) | `cmd.123.sub` | Single | Multi-segment | None | N/A | `.` | Incorrect | None | Syntax Error | `(false, false)` | Error: Invalid identifier `123` |
//! | S6.29 | R1 (Longest possible sequence) | `cmd.sub arg` | Single | Multi-segment | Positional | Identifier | None | Correct | None | None | `(false, false)` | `cmd.sub`, `arg` |
//! | S6.30 | R0 (Multiple consecutive whitespace) | `cmd   arg` | Single | Simple | Positional | Identifier | None | Correct | Multiple | None | `(false, false)` | `cmd`, `arg` (single space separation) |
use unilang_instruction_parser::*;
use unilang_instruction_parser::error::ErrorKind;
use unilang_instruction_parser::UnilangParserOptions;

/// Test Combination: T4.1
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

/// Test Combination: T4.2
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

/// Test Combination: T4.3
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

/// Test Combination: T4.4
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

/// Test Combination: T4.5
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

/// Test Combination: T4.6
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

/// Test Combination: T4.7
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

/// Test Combination: T4.8
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

/// Test Combination: T4.9
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

/// Test Combination: T4.10
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

/// Test Combination: T4.11
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

/// Test Combination: T4.12
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

/// Tests Rule 0 (Whitespace Separation) and Rule 1 (Command Path Identification) with leading/trailing and internal whitespace.
/// Test Combination: S6.1
#[test]
fn s6_1_whitespace_separation_and_command_path() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "  cmd.sub  arg1  ";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string(), "sub".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "arg1".to_string());
}

/// Tests Rule 0 (Whitespace Separation) and Rule 5.1 (Positional Arguments) with a quoted string containing spaces.
/// Test Combination: S6.2
#[test]
fn s6_2_whitespace_in_quoted_positional_arg() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd \"val with spaces\"";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "val with spaces".to_string());
}

/// Tests Rule 1 (Command Path Identification) and Rule 2 (End of Command Path) with a multi-segment path and positional argument.
/// Test Combination: S6.3
#[test]
fn s6_3_multi_segment_path_and_positional_arg_transition() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd.sub.action arg1";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string(), "sub".to_string(), "action".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "arg1".to_string());
}

/// Tests Rule 1 (Command Path Identification), Rule 2 (End of Command Path), and Rule 5.2 (Named Arguments) with a multi-segment path and named argument.
/// Test Combination: S6.4
#[test]
fn s6_4_multi_segment_path_and_named_arg_transition() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd.sub name::val";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string(), "sub".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, "val".to_string());
}

/// Tests Rule 3.1 (Leading Dot) with a command and positional argument.
/// Test Combination: S6.5
#[test]
fn s6_5_leading_dot_command_with_arg() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = ".cmd arg";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "arg".to_string());
}

/// Tests Rule 3.3 (Trailing Dot) as a syntax error.
/// Test Combination: S6.6
#[test]
fn s6_6_trailing_dot_syntax_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd.";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for input '{}', but got Ok: {:?}", input, result.ok());
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::Syntax("Command path cannot end with a '.'".to_string()));
    }
}

/// Tests Rule 3.4 (Consecutive Dots) as a syntax error.
/// Test Combination: S6.7
#[test]
fn s6_7_consecutive_dots_syntax_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd..sub";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for input '{}', but got Ok: {:?}", input, result.ok());
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::Syntax("Consecutive dots in command path".to_string()));
    }
}

/// Tests Rule 4 (Help Operator) with a command and `?` as the final token.
/// Test Combination: S6.8
#[test]
fn s6_8_help_operator_correct_placement() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd ?";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.help_requested);
}

/// Tests Rule 4 (Help Operator) and Rule 5.2 (Named Arguments) with a named argument followed by `?`.
/// Test Combination: S6.9
#[test]
fn s6_9_named_arg_followed_by_help_operator() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd name::val ?";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, "val".to_string());
    assert!(instruction.help_requested);
}

/// Tests Rule 4 (Help Operator) with `?` followed by other tokens (syntax error).
/// Test Combination: S6.10
#[test]
fn s6_10_help_operator_followed_by_other_tokens_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd ? arg";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for input '{}', but got Ok: {:?}", input, result.ok());
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::Syntax("Help operator '?' must be the last token".to_string()));
    }
}

/// Tests Rule 5.1 (Positional Arguments) with multiple positional arguments.
/// Test Combination: S6.11
#[test]
fn s6_11_multiple_positional_arguments() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd pos1 pos2";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 2);
    assert_eq!(instruction.positional_arguments[0].value, "pos1".to_string());
    assert_eq!(instruction.positional_arguments[1].value, "pos2".to_string());
}

/// Tests Rule 5.2 (Named Arguments) with a single named argument.
/// Test Combination: S6.12
#[test]
fn s6_12_single_named_argument() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd key::val";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("key").unwrap().value, "val".to_string());
}

/// Tests Rule 5.2 (Named Arguments) with a named argument whose value is a quoted string with spaces.
/// Test Combination: S6.13
#[test]
fn s6_13_named_arg_quoted_value_with_spaces() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd key::\"val with spaces\"";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("key").unwrap().value, "val with spaces".to_string());
}

/// Tests Rule 5.3 (Positional After Named) when allowed (default behavior).
/// Test Combination: S6.14
#[test]
fn s6_14_positional_after_named_allowed() {
    let parser = Parser::new(UnilangParserOptions::default()); // Default allows positional after named
    let input = "cmd name::val pos1";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, "val".to_string());
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "pos1".to_string());
}

/// Tests Rule 5.3 (Positional After Named) when `error_on_positional_after_named` is true.
/// Test Combination: S6.15
#[test]
fn s6_15_positional_after_named_error() {
    let parser = Parser::new(UnilangParserOptions { error_on_positional_after_named: true, ..Default::default() });
    let input = "cmd name::val pos1";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for input '{}', but got Ok: {:?}", input, result.ok());
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::Syntax("Positional argument after named argument".to_string()));
    }
}

/// Tests Rule 5.4 (Duplicate Named Arguments) when last one wins (default behavior).
/// Test Combination: S6.16
#[test]
fn s6_16_duplicate_named_arg_last_wins() {
    let parser = Parser::new(UnilangParserOptions::default()); // Default: last wins
    let input = "cmd name::val1 name::val2";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, "val2".to_string());
}

/// Tests Rule 5.4 (Duplicate Named Arguments) when `error_on_duplicate_named_arguments` is true.
/// Test Combination: S6.17
#[test]
fn s6_17_duplicate_named_arg_error() {
    let parser = Parser::new(UnilangParserOptions { error_on_duplicate_named_arguments: true, ..Default::default() });
    let input = "cmd name::val1 name::val2";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for input '{}', but got Ok: {:?}", input, result.ok());
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::Syntax("Duplicate named argument 'name'".to_string()));
    }
}

/// Tests multi-instruction parsing with basic commands and arguments.
/// Test Combination: S6.18
#[test]
fn s6_18_multi_instruction_basic() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd1 arg1 ;; cmd2 name::val";
    let result = parser.parse_multiple_instructions(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 2);
    assert_eq!(instructions[0].command_path_slices, vec!["cmd1".to_string()]);
    assert_eq!(instructions[0].positional_arguments.len(), 1);
    assert_eq!(instructions[0].positional_arguments[0].value, "arg1".to_string());
    assert_eq!(instructions[1].command_path_slices, vec!["cmd2".to_string()]);
    assert_eq!(instructions[1].named_arguments.len(), 1);
    assert_eq!(instructions[1].named_arguments.get("name").unwrap().value, "val".to_string());
}

/// Tests multi-instruction parsing with an empty segment due to consecutive delimiters.
/// Test Combination: S6.19
#[test]
fn s6_19_multi_instruction_empty_segment_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd1 ;;;; cmd2";
    let result = parser.parse_multiple_instructions(input);
    assert!(result.is_err(), "Expected error for input '{}', but got Ok: {:?}", input, result.ok());
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::EmptyInstructionSegment);
    }
}

/// Tests multi-instruction parsing with a trailing delimiter.
/// Test Combination: S6.20
#[test]
fn s6_20_multi_instruction_trailing_delimiter_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd1 ;;";
    let result = parser.parse_multiple_instructions(input);
    assert!(result.is_err(), "Expected error for input '{}', but got Ok: {:?}", input, result.ok());
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::TrailingDelimiter);
    }
}

/// Tests Rule 2 (Transition to Arguments) with a non-identifier token.
/// Test Combination: S6.21
#[test]
fn s6_21_transition_by_non_identifier_token() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd !arg";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for input '{}', but got Ok: {:?}", input, result.ok());
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::Syntax("Unexpected token '!' in arguments".to_string()));
    }
}

/// Tests Rule 2 (Transition to Arguments) with a quoted string.
/// Test Combination: S6.22
#[test]
fn s6_22_transition_by_quoted_string() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd \"arg\"";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "arg".to_string());
}

/// Tests Rule 2 (Transition to Arguments) with a help operator.
/// Test Combination: S6.23
#[test]
fn s6_23_transition_by_help_operator() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd ?";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.help_requested);
}

/// Tests Rule 5.2 (Named Arguments) with a value containing `::`.
/// Test Combination: S6.24
#[test]
fn s6_24_named_arg_value_with_double_colon() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd msg::\"DEPRECATED::message\"";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("msg").unwrap().value, "DEPRECATED::message".to_string());
}

/// Tests Rule 5.2 (Named Arguments) with a value containing commas.
/// Test Combination: S6.25
#[test]
fn s6_25_named_arg_value_with_commas() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd tags::dev,rust,unilang";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("tags").unwrap().value, "dev,rust,unilang".to_string());
}

/// Tests Rule 5.2 (Named Arguments) with a value containing key-value pairs.
/// Test Combination: S6.26
#[test]
fn s6_26_named_arg_value_with_key_value_pair() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd headers::Content-Type=application/json,Auth-Token=xyz";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("headers").unwrap().value, "Content-Type=application/json,Auth-Token=xyz".to_string());
}

/// Tests Rule 1 (Command Path Identification) with whitespace around dots.
/// Test Combination: S6.27
#[test]
fn s6_27_command_path_whitespace_around_dot() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd . sub";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string(), "sub".to_string()]);
}

/// Tests Rule 1 (Command Path Identification) with an invalid identifier segment.
/// Test Combination: S6.28
#[test]
fn s6_28_command_path_invalid_identifier_segment() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd.123.sub";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for input '{}', but got Ok: {:?}", input, result.ok());
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::Syntax("Invalid identifier '123' in command path".to_string()));
    }
}

/// Tests Rule 1 (Command Path Identification) for the longest possible sequence.
/// Test Combination: S6.29
#[test]
fn s6_29_command_path_longest_possible_sequence() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd.sub arg";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string(), "sub".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "arg".to_string());
}

/// Tests Rule 0 (Whitespace Separation) with multiple consecutive whitespace characters.
/// Test Combination: S6.30
#[test]
fn s6_30_multiple_consecutive_whitespace() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd   arg";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "arg".to_string());
}