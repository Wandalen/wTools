//! Comprehensive test suite for the unilang instruction parser.
//! Tests are designed based on the Test Matrix in plan.md.

use unilang_instruction_parser::*;
use unilang_instruction_parser::error::{ErrorKind};
// Removed: use unilang_instruction_parser::error::{ErrorKind, SourceLocation};
// Removed: use std::collections::HashMap;

fn default_options() -> UnilangParserOptions {
    UnilangParserOptions::default()
}

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
    let parser = Parser::new(default_options());
    let input = "cmd val";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "CT1.1 Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string(), "val".to_string()], "CT1.1 Path");
    assert!(instruction.positional_arguments.is_empty(), "CT1.1 Positional args should be empty");
    assert!(instruction.named_arguments.is_empty(), "CT1.1 Named args");
    assert!(!instruction.help_requested, "CT1.1 Help requested");
}

// Test Matrix Row: CT1.2
#[test]
fn ct1_2_single_str_multi_path_unquoted_named_arg() {
    let parser = Parser::new(default_options());
    let input = "path1 path2 name1::val1";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "CT1.2 Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["path1".to_string(), "path2".to_string()], "CT1.2 Path");
    assert!(instruction.positional_arguments.is_empty(), "CT1.2 Positional args");
    assert_eq!(instruction.named_arguments.len(), 1, "CT1.2 Named args count");
    let arg1 = instruction.named_arguments.get("name1").expect("CT1.2 Missing name1");
    assert_eq!(arg1.value, "val1".to_string(), "CT1.2 name1 value");
    assert!(!instruction.help_requested, "CT1.2 Help requested");
}

// Test Matrix Row: CT1.3
#[test]
fn ct1_3_single_str_single_path_help_no_args() {
    let parser = Parser::new(default_options());
    let input = "cmd ?";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "CT1.3 Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()], "CT1.3 Path");
    assert!(instruction.positional_arguments.is_empty(), "CT1.3 Positional args");
    assert!(instruction.named_arguments.is_empty(), "CT1.3 Named args");
    assert!(instruction.help_requested, "CT1.3 Help requested should be true");
}

// Test Matrix Row: CT1.4
#[test]
fn ct1_4_single_str_single_path_quoted_pos_arg() {
    let parser = Parser::new(default_options());
    let input = "cmd \"quoted val\"";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "CT1.4 Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()], "CT1.4 Path");
    assert_eq!(instruction.positional_arguments.len(), 1, "CT1.4 Positional args count");
    assert_eq!(instruction.positional_arguments[0].value, "quoted val".to_string(), "CT1.4 Positional arg value");
    assert!(instruction.named_arguments.is_empty(), "CT1.4 Named args");
    assert!(!instruction.help_requested, "CT1.4 Help requested");
}

// Test Matrix Row: CT1.5
#[test]
fn ct1_5_single_str_single_path_named_arg_escaped_val() {
    let parser = Parser::new(default_options());
    let input = "cmd name1::\"esc\\nval\"";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "CT1.5 Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
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
    let parser = Parser::new(default_options());
    let input = "cmd name1::\"bad\\xval\"";
    let result = parser.parse_single_str(input);
    assert!(result.is_err(), "CT1.6 Expected error for invalid escape, got Ok: {:?}", result.ok());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)), "CT1.6 ErrorKind mismatch: {:?}", e.kind);
        assert!(e.to_string().contains("Invalid escape sequence: \\x"), "CT1.6 Error message mismatch: {}", e);
    }
}

// Test Matrix Row: CT2.1
#[test]
fn ct2_1_slice_multi_path_mixed_args() {
    let parser = Parser::new(options_allow_pos_after_named());
    let input_slice: &[&str] = &["path1 path2", "pos1", "name1::val1"];
    let result = parser.parse_slice(input_slice);
    assert!(result.is_ok(), "CT2.1 Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 3, "CT2.1 Expected 3 instructions from slice");

    // Instruction 1: from "path1 path2"
    let instr1 = &instructions[0];
    assert_eq!(instr1.command_path_slices, vec!["path1".to_string(), "path2".to_string()], "CT2.1 Instr1 Path");
    assert!(instr1.positional_arguments.is_empty(), "CT2.1 Instr1 Positional args");
    assert!(instr1.named_arguments.is_empty(), "CT2.1 Instr1 Named args");
    assert!(!instr1.help_requested, "CT2.1 Instr1 Help requested");

    // Instruction 2: from "pos1"
    let instr2 = &instructions[1];
    assert_eq!(instr2.command_path_slices, vec!["pos1".to_string()], "CT2.1 Instr2 Path (pos1 treated as command)");
    assert!(instr2.positional_arguments.is_empty(), "CT2.1 Instr2 Positional args");
    assert!(instr2.named_arguments.is_empty(), "CT2.1 Instr2 Named args");
    assert!(!instr2.help_requested, "CT2.1 Instr2 Help requested");

    // Instruction 3: from "name1::val1"
    let instr3 = &instructions[2];
    assert!(instr3.command_path_slices.is_empty(), "CT2.1 Instr3 Path should be empty");
    assert!(instr3.positional_arguments.is_empty(), "CT2.1 Instr3 Positional args");
    assert_eq!(instr3.named_arguments.len(), 1, "CT2.1 Instr3 Named args count");
    let named_arg = instr3.named_arguments.get("name1").expect("CT2.1 Missing name1 in Instr3");
    assert_eq!(named_arg.value, "val1".to_string(), "CT2.1 name1 value in Instr3");
    assert!(!instr3.help_requested, "CT2.1 Instr3 Help requested");
}

// Test Matrix Row: CT3.1
#[test]
fn ct3_1_single_str_separator_basic() {
    let parser = Parser::new(default_options());
    let input = "cmd1 arg1 ;; cmd2 name::val";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "CT3.1 Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 2, "CT3.1 Instruction count");

    // Instruction 1: "cmd1 arg1" (Path: "cmd1", "arg1")
    let instr1 = &instructions[0];
    assert_eq!(instr1.command_path_slices, vec!["cmd1".to_string(), "arg1".to_string()], "CT3.1 Instr1 Path");
    assert!(instr1.positional_arguments.is_empty(), "CT3.1 Instr1 Positional");
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
    let parser = Parser::new(options_error_on_duplicate_named());
    let input = "cmd name::val1 name::val2";
    let result = parser.parse_single_str(input);
    assert!(result.is_err(), "CT4.1 Expected error for duplicate named, got Ok: {:?}", result.ok());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)), "CT4.1 ErrorKind mismatch: {:?}", e.kind);
        assert!(e.to_string().contains("Duplicate named argument: name"), "CT4.1 Error message mismatch: {}", e);
    }
}

// Test Matrix Row: CT4.2
#[test]
fn ct4_2_single_str_duplicate_named_last_wins() {
    let parser = Parser::new(default_options());
    let input = "cmd name::val1 name::val2";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "CT4.2 Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1, "CT4.2 Named args count");
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, "val2".to_string(), "CT4.2 Last value should win");
}

// Test Matrix Row: CT5.1
#[test]
fn ct5_1_single_str_no_path_named_arg_only() {
    let parser = Parser::new(default_options());
    let input = "name::val";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "CT5.1 Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert!(instruction.command_path_slices.is_empty(), "CT5.1 Path should be empty");
    assert_eq!(instruction.named_arguments.len(), 1, "CT5.1 Named args count");
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, "val".to_string(), "CT5.1 name value");
}