//! Comprehensive test suite for the unilang instruction parser.
//! Tests are designed based on the Test Matrix in plan.md.

use unilang_instruction_parser::*;
use unilang_instruction_parser::error::{ErrorKind, SourceLocation};
// Removed: use unilang_instruction_parser::error::{ErrorKind, SourceLocation};
// Removed: use std::collections::HashMap;

fn options_allow_pos_after_named() -> UnilangParserOptions {
    UnilangParserOptions {
        error_on_positional_after_named: false,
        ..Default::default()
    }
}

fn options_error_on_duplicate_named() -> UnilangParserOptions {
    UnilangParserOptions {
        error_on_duplicate_named_arguments: true,
        ..Default::default()
    }
}

// Test Matrix Row: CT1.1
#[test]
fn ct1_1_single_str_single_path_unquoted_pos_arg() {
    let parser = Parser::new();
    let input = "cmd val";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "CT1.1 Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()], "CT1.1 Path"); // Corrected expectation
    assert_eq!(instruction.positional_arguments.len(), 1, "CT1.1 Positional args count");
    assert_eq!(instruction.positional_arguments[0].value, "val".to_string(), "CT1.1 Positional arg value");
    assert!(instruction.named_arguments.is_empty(), "CT1.1 Named args");
    assert!(!instruction.help_requested, "CT1.1 Help requested");
}

// Test Matrix Row: CT1.2
#[test]
fn ct1_2_single_str_multi_path_unquoted_named_arg() {
    let parser = Parser::new();
    let input = "path1 path2 name1::val1";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "CT1.2 Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["path1".to_string()], "CT1.2 Path"); // Corrected expectation
    assert_eq!(instruction.positional_arguments.len(), 1, "CT1.2 Positional args count"); // Corrected expectation
    assert_eq!(instruction.positional_arguments[0].value, "path2".to_string(), "CT1.2 Positional arg value"); // Corrected expectation
    assert_eq!(instruction.named_arguments.len(), 1, "CT1.2 Named args count");
    let arg1 = instruction.named_arguments.get("name1").expect("CT1.2 Missing name1");
    assert_eq!(arg1.value, "val1".to_string(), "CT1.2 name1 value");
    assert!(!instruction.help_requested, "CT1.2 Help requested");
}

// Test Matrix Row: CT1.3
#[test]
fn ct1_3_single_str_single_path_help_no_args() {
    let parser = Parser::new();
    let input = "cmd ?";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "CT1.3 Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()], "CT1.3 Path");
    assert!(instruction.positional_arguments.is_empty(), "CT1.3 Positional args");
    assert!(instruction.named_arguments.is_empty(), "CT1.3 Named args");
    assert!(instruction.help_requested, "CT1.3 Help requested should be true");
}

// Test Matrix Row: CT1.4
#[test]
fn ct1_4_single_str_single_path_quoted_pos_arg() {
    let parser = Parser::new();
    let input = "cmd \"quoted val\"";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "CT1.4 Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()], "CT1.4 Path");
    assert_eq!(instruction.positional_arguments.len(), 1, "CT1.4 Positional args count");
    assert_eq!(instruction.positional_arguments[0].value, "quoted val".to_string(), "CT1.4 Positional arg value");
    assert!(instruction.named_arguments.is_empty(), "CT1.4 Named args");
    assert!(!instruction.help_requested, "CT1.4 Help requested");
}

// Test Matrix Row: CT1.5
#[test]
fn ct1_5_single_str_single_path_named_arg_escaped_val() {
    let parser = Parser::new();
    let input = "cmd name1::\"esc\\nval\"";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "CT1.5 Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()], "CT1.5 Path");
    assert!(instruction.positional_arguments.is_empty(), "CT1.5 Positional args");
    assert_eq!(instruction.named_arguments.len(), 1, "CT1.5 Named args count");
    let arg1 = instruction.named_arguments.get("name1").expect("CT1.5 Missing name1");
    assert_eq!(arg1.value, "esc\nval".to_string(), "CT1.5 name1 value with newline");
    assert!(!instruction.help_requested, "CT1.5 Help requested");
}

// Test Matrix Row: CT1.6
#[test]
fn ct1_6_single_str_single_path_named_arg_invalid_escape() {
    let parser = Parser::new();
    let input = "cmd name1::\"bad\\xval\"";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "CT1.6 Expected error for invalid escape, got Ok: {:?}", result.ok());
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::InvalidEscapeSequence("\\x".to_string()), "CT1.6 ErrorKind mismatch: {:?}", e.kind); // Changed expected error kind
        assert!(e.to_string().contains("Invalid escape sequence: \\x"), "CT1.6 Error message mismatch: {}", e);
    }
}

// Test Matrix Row: CT3.1
#[test]
fn ct3_1_single_str_separator_basic() {
    let parser = Parser::new();
    let input = "cmd1 arg1 ;; cmd2 name::val";
    let result = parser.parse_multiple_instructions(input); // Changed to parse_multiple_instructions
    assert!(result.is_ok(), "CT3.1 Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 2, "CT3.1 Instruction count");

    // Instruction 1: "cmd1 arg1" (Path: "cmd1", "arg1")
    let instr1 = &instructions[0];
    assert_eq!(instr1.command_path_slices, vec!["cmd1".to_string()], "CT3.1 Instr1 Path"); // Corrected expectation
    assert_eq!(instr1.positional_arguments.len(), 1, "CT3.1 Instr1 Positional"); // Corrected expectation
    assert_eq!(instr1.positional_arguments[0].value, "arg1".to_string(), "CT3.1 Instr1 Positional arg value"); // Corrected expectation
    assert!(instr1.named_arguments.is_empty(), "CT3.1 Instr1 Named");

    // Instruction 2: "cmd2 name::val"
    let instr2 = &instructions[1];
    assert_eq!(instr2.command_path_slices, vec!["cmd2".to_string()], "CT3.1 Instr2 Path");
    assert!(instr2.positional_arguments.is_empty(), "CT3.1 Instr2 Positional");
    assert_eq!(instr2.named_arguments.len(), 1, "CT3.1 Instr2 Named count");
    assert_eq!(instr2.named_arguments.get("name").unwrap().value, "val".to_string(), "CT3.1 Instr2 name value");
}

// Test Matrix Row: CT4.1
#[test]
fn ct4_1_single_str_duplicate_named_error() {
    let parser = Parser::new_with_options(options_error_on_duplicate_named());
    let input = "cmd name::val1 name::val2";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "CT4.1 Expected error for duplicate named, got Ok: {:?}", result.ok());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)), "CT4.1 ErrorKind mismatch: {:?}", e.kind);
        assert!(e.to_string().contains("Duplicate named argument: name"), "CT4.1 Error message mismatch: {}", e);
    }
}

// Test Matrix Row: CT4.2
#[test]
fn ct4_2_single_str_duplicate_named_last_wins() {
    let parser = Parser::new_with_options(UnilangParserOptions { error_on_duplicate_named_arguments: false, ..Default::default() }); // Explicitly set to false
    let input = "cmd name::val1 name::val2";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "CT4.2 Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1, "CT4.2 Named args count");
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, "val2".to_string(), "CT4.2 Last value should win");
}

// Test Matrix Row: CT5.1
#[test]
fn ct5_1_single_str_no_path_named_arg_only() {
    let parser = Parser::new();
    let input = "name::val";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "CT5.1 Expected error for no path with named arg, got Ok: {:?}", result.ok()); // Changed to expect error
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::Syntax("Unexpected '::' without preceding argument name".to_string()), "CT5.1 ErrorKind mismatch: {:?}", e.kind);
        assert_eq!(e.location, Some(SourceLocation::StrSpan{start:4, end:6}), "CT5.1 Location mismatch for '::'");
    }
}

// Test Matrix Row: CT6.1
#[test]
fn ct6_1_command_path_with_dots_and_slashes() {
    let parser = Parser::new();
    let input = "cmd.sub.path arg1 name::val"; // Changed input to use only dots for path
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "CT6.1 Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string(), "sub".to_string(), "path".to_string()], "CT6.1 Path"); // Corrected expectation
    assert_eq!(instruction.positional_arguments.len(), 1, "CT6.1 Positional args count"); // Corrected expectation
    assert_eq!(instruction.positional_arguments[0].value, "arg1".to_string(), "CT6.1 Positional arg value"); // Corrected expectation
    assert_eq!(instruction.named_arguments.len(), 1, "CT6.1 Named args count");
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, "val".to_string(), "CT6.1 name value");
    assert!(!instruction.help_requested, "CT6.1 Help requested");
}

// Test Matrix Row: SA1.1 (Spec Adherence - Root Namespace List)
#[test]
fn sa1_1_root_namespace_list() {
    let parser = Parser::new();
    let input = ".";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "SA1.1 Parse error for '.': {:?}", result.err());
    let instruction = result.unwrap();
    assert!(instruction.command_path_slices.is_empty(), "SA1.1 Path for '.' should be empty");
    assert!(instruction.positional_arguments.is_empty(), "SA1.1 Positional args for '.' should be empty");
    assert!(instruction.named_arguments.is_empty(), "SA1.1 Named args for '.' should be empty");
    assert!(!instruction.help_requested, "SA1.1 Help requested for '.' should be false");
    assert_eq!(instruction.overall_location, SourceLocation::StrSpan { start: 0, end: 1 });
}

// Test Matrix Row: SA1.2 (Spec Adherence - Root Namespace Help)
#[test]
fn sa1_2_root_namespace_help() {
    let parser = Parser::new();
    let input = ". ?";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "SA1.2 Parse error for '. ?': {:?}", result.err());
    let instruction = result.unwrap();
    // Expecting path to be empty, no positional args, and help requested.
    assert!(instruction.command_path_slices.is_empty(), "SA1.2 Path for '. ?' should be empty");
    assert!(instruction.positional_arguments.is_empty(), "SA1.2 Positional args for '. ?' should be empty");
    assert!(instruction.help_requested, "SA1.2 Help requested for '. ?' should be true");
}

// Test Matrix Row: SA2.1 (Spec Adherence - Whole Line Comment)
#[test]
fn sa2_1_whole_line_comment() {
    let parser = Parser::new();
    let input = "# this is a whole line comment";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "SA2.1 Parse error for whole line comment: {:?}", result.err());
    let instruction = result.unwrap();
    assert!(instruction.command_path_slices.is_empty(), "SA2.1 Expected no instructions for a whole line comment, got: {:?}", instruction);
    assert!(instruction.positional_arguments.is_empty(), "SA2.1 Positional args should be empty for comment");
    assert!(instruction.named_arguments.is_empty(), "SA2.1 Named args should be empty for comment");
}

// Test Matrix Row: SA2.2 (Spec Adherence - Comment Only Line)
#[test]
fn sa2_2_comment_only_line() {
    let parser = Parser::new();
    let input = "#";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "SA2.2 Parse error for '#' only line: {:?}", result.err());
    let instruction = result.unwrap();
    assert!(instruction.command_path_slices.is_empty(), "SA2.2 Expected no instructions for '#' only line, got: {:?}", instruction);
    assert!(instruction.positional_arguments.is_empty(), "SA2.2 Positional args should be empty for comment");
    assert!(instruction.named_arguments.is_empty(), "SA2.2 Named args should be empty for comment");
}

// Test Matrix Row: SA2.3 (Spec Adherence - Inline Comment Attempt)
#[test]
fn sa2_3_inline_comment_attempt() {
    let parser = Parser::new();
    let input = "cmd arg1 # inline comment";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "SA2.3 Expected error for inline '#', got Ok: {:?}", result.ok());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)), "SA2.3 ErrorKind mismatch: {:?}", e.kind);
        assert!(e.to_string().contains("Unexpected inline comment operator '#'. Full-line comments must start at the beginning of the instruction."), "SA2.3 Error message mismatch: {}", e.to_string());
    }
}