//! ## Test Matrix for Syntactic Analyzer Command Tests
//!
//! This matrix outlines test cases for the syntactic analyzer, focusing on how command paths
//! are parsed, how arguments are handled, and the behavior of special operators like `?` and `::`.
//! It also covers multi-instruction parsing and error conditions related to delimiters.
//!
//! **Test Factors:**
//! - Command Path: Multi-segment, Simple
//! - Help Operator: Present, Only help operator, Followed by other tokens
//! - Arguments: Positional, Named, None
//! - Multi-instruction: Multiple commands, Leading semicolon, Trailing semicolon, Multiple consecutive semicolons, Only semicolons
//! - Path Termination: Double colon delimiter
//!
//! ---
//!
//! **Test Combinations:**
//!
//! | ID | Aspect Tested | Input String | Command Path | Help Operator | Arguments | Multi-instruction | Path Termination | Expected Behavior |
//! |---|---|---|---|---|---|---|---|---|
//! | T5.1 | Multi-segment command path | `cmd subcmd another` | Multi-segment | Absent | Positional | N/A | N/A | Command `cmd`, Positional `subcmd`, `another` |
//! | T5.2 | Command with help operator | `cmd ?` | Simple | Present | None | N/A | N/A | Command `cmd`, Help requested |
//! | T5.3 | Command with help operator and multi-segment path | `cmd sub ?` | Simple | Present | Positional | N/A | N/A | Command `cmd`, Positional `sub`, Help requested |
//! | T5.4 | Only help operator | `?` | None | Only help operator | None | N/A | N/A | Help requested |
//! | T5.5 | Multiple commands with path and help | `cmd1 ;; cmd2 sub ? ;; cmd3` | Simple | Present | Positional | Multiple commands | N/A | Three instructions parsed, second with help |
//! | T5.6 | Leading semicolon error | `;; cmd1` | N/A | Absent | N/A | Leading semicolon | N/A | Error: Empty instruction segment |
//! | T5.7 | Trailing semicolon error | `cmd1 ;;` | N/A | Absent | N/A | Trailing semicolon | N/A | Error: Trailing delimiter |
//! | T5.8 | Multiple consecutive semicolons error | `cmd1 ;;;; cmd2` | N/A | Absent | N/A | Multiple consecutive semicolons | N/A | Error: Empty instruction segment |
//! | T5.9 | Only semicolons error | `;;` | N/A | Absent | N/A | Only semicolons | N/A | Error: Empty instruction segment |
//! | T5.10 | Path stops at double colon delimiter | `cmd path arg::val` | Simple | Absent | Positional, Named | N/A | Double colon | Command `cmd`, Positional `path`, Named `arg::val` |
use unilang_parser::*;
use unilang_parser::error::ErrorKind;
use unilang_parser::UnilangParserOptions;

/// Tests that a multi-segment command path is parsed correctly, with subsequent tokens treated as positional arguments.
/// Test Combination: T5.1
#[test]
fn multi_segment_command_path_parsed() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd subcmd another";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "parse_single_instruction failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 2);
    assert_eq!(instruction.positional_arguments[0].value, "subcmd".to_string());
    assert_eq!(instruction.positional_arguments[1].value, "another".to_string());
}

/// Tests that a command followed by a help operator `?` is parsed correctly, setting the `help_requested` flag.
/// Test Combination: T5.2
#[test]
fn command_with_help_operator_parsed() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_single_instruction("cmd ?");
    assert!(result.is_ok(), "parse_single_instruction failed: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.positional_arguments.is_empty()); // Corrected: '?' is not a positional arg
    assert!(instruction.named_arguments.is_empty());
    assert!(instruction.help_requested); // Corrected: '?' sets help_requested flag
}

/// Tests that a command with a multi-segment path followed by a help operator `?` is parsed correctly.
/// Test Combination: T5.3
#[test]
fn command_with_help_operator_and_multi_segment_path() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd sub ?";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "parse_single_instruction failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1); // Corrected: 'sub' is positional, '?' is not
    assert_eq!(instruction.positional_arguments[0].value, "sub".to_string());
    assert!(instruction.named_arguments.is_empty());
    assert!(instruction.help_requested); // Corrected: '?' sets help_requested flag
}

/// Tests parsing an input consisting only of the help operator `?`.
/// Test Combination: T5.4
#[test]
fn only_help_operator() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_single_instruction("?");
    assert!(result.is_ok(), "parse_single_instruction failed for '?': {:?}", result.err());
    let instruction = result.unwrap();
    assert!(instruction.command_path_slices.is_empty());
    assert!(instruction.positional_arguments.is_empty()); // Corrected: '?' is not a positional arg
    assert!(instruction.named_arguments.is_empty());
    assert!(instruction.help_requested); // Corrected: '?' sets help_requested flag
}


/// Tests parsing multiple commands separated by `;;`, including a command with a path and help operator.
/// Test Combination: T5.5
#[test]
fn multiple_commands_separated_by_semicolon_path_and_help_check() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd1 ;; cmd2 sub ? ;; cmd3";
    let result = parser.parse_multiple_instructions(input);
    assert!(result.is_ok(), "parse_multiple_instructions failed for input '{}': {:?}", input, result.err());
    let instructions = result.unwrap(); // This will still be a Vec<GenericInstruction> for parse_multiple_instructions
    assert_eq!(instructions.len(), 3);

    assert_eq!(instructions[0].command_path_slices, vec!["cmd1".to_string()]);

    assert_eq!(instructions[1].command_path_slices, vec!["cmd2".to_string()]);
    assert_eq!(instructions[1].positional_arguments.len(), 1); // Corrected: 'sub' is positional, '?' is not
    assert_eq!(instructions[1].positional_arguments[0].value, "sub".to_string());
    assert!(instructions[1].help_requested); // Corrected: '?' sets help_requested flag

    assert_eq!(instructions[2].command_path_slices, vec!["cmd3".to_string()]);
}

/// Tests that a leading semicolon `;;` results in an `EmptyInstructionSegment` error.
/// Test Combination: T5.6
#[test]
fn leading_semicolon_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_multiple_instructions(";; cmd1"); // Changed to parse_multiple_instructions
    assert!(result.is_err(), "Expected error for leading ';;'");
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::EmptyInstructionSegment));
        assert!(e.to_string().contains("Empty instruction segment"));
    }
}

/// Tests that a trailing semicolon `;;` results in a `TrailingDelimiter` error.
/// Test Combination: T5.7
#[test]
fn trailing_semicolon_error_if_empty_segment_is_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd1 ;;";
    let result = parser.parse_multiple_instructions(input); // Changed to parse_multiple_instructions
    assert!(result.is_err(), "Expected error for trailing ';;' if empty segments are errors");
     if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::TrailingDelimiter)); // Updated to expect TrailingDelimiter
        assert!(e.to_string().contains("Trailing delimiter")); // Updated error message
    }
}

/// Tests that multiple consecutive semicolons `;;;;` result in an `EmptyInstructionSegment` error.
/// Test Combination: T5.8
#[test]
fn multiple_consecutive_semicolons_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_multiple_instructions("cmd1 ;;;; cmd2"); // Changed to parse_multiple_instructions
    assert!(result.is_err(), "Expected error for 'cmd1 ;;;; cmd2'");
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::EmptyInstructionSegment));
        assert!(e.to_string().contains("Empty instruction segment"));
    }
}

/// Tests that an input consisting only of semicolons `;;` or `;;;;` results in an `EmptyInstructionSegment` error.
/// Test Combination: T5.9
#[test]
fn only_semicolons_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_multiple_instructions(";;"); // Changed to parse_multiple_instructions
    assert!(result.is_err(), "Expected error for ';;'");
     if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::EmptyInstructionSegment));
        assert!(e.to_string().contains("Empty instruction segment"));
    }
    let result_double = parser.parse_multiple_instructions(";;;;"); // Changed to parse_multiple_instructions
    assert!(result_double.is_err(), "Expected error for ';;;;'");
    if let Err(e) = result_double {
        assert!(matches!(e.kind, ErrorKind::EmptyInstructionSegment));
        assert!(e.to_string().contains("Empty instruction segment"));
    }
}

/// Tests that the command path correctly stops at a double colon `::` delimiter, treating subsequent tokens as arguments.
/// Test Combination: T5.10
#[test]
fn path_stops_at_double_colon_delimiter() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd path arg::val";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "path".to_string());
    assert_eq!(instruction.named_arguments.len(), 1);
    assert!(instruction.named_arguments.contains_key("arg"));
    assert_eq!(instruction.named_arguments.get("arg").unwrap().value, "val");
}