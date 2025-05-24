//! Tests for syntactic analysis, focusing on command grouping and boundaries.
use unilang_instruction_parser::*;
use unilang_instruction_parser::error::ErrorKind; // For error assertion

fn default_options() -> UnilangParserOptions {
    UnilangParserOptions::default()
}

#[test]
fn single_command_path_parsed() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd");
    assert!(result.is_ok(), "parse_single_str failed: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1, "Expected 1 instruction for 'cmd'");
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.named_arguments.is_empty());
    assert!(instruction.positional_arguments.is_empty());
    assert!(!instruction.help_requested);
}

#[test]
fn multi_segment_command_path_parsed() {
    let parser = Parser::new(default_options());
    let input = "cmd subcmd another";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "parse_single_str failed for input '{}': {:?}", input, result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string(), "subcmd".to_string(), "another".to_string()]);
    assert!(instructions[0].positional_arguments.is_empty());
    assert!(!instructions[0].help_requested);
}

#[test]
fn command_with_help_operator_parsed() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd ?");
    assert!(result.is_ok(), "parse_single_str failed: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    assert_eq!(instructions[0].command_path_slices, vec!["cmd".to_string()]);
    assert!(instructions[0].help_requested);
    assert!(instructions[0].positional_arguments.is_empty());
}

#[test]
fn command_with_help_operator_and_multi_segment_path() {
    let parser = Parser::new(default_options());
    let input = "cmd sub ?";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "parse_single_str failed for input '{}': {:?}", input, result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    assert_eq!(instructions[0].command_path_slices, vec!["cmd".to_string(), "sub".to_string()]);
    assert!(instructions[0].help_requested);
    assert!(instructions[0].positional_arguments.is_empty());
}

#[test]
fn only_help_operator() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("?");
    assert!(result.is_ok(), "parse_single_str failed for '?': {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    assert!(instructions[0].command_path_slices.is_empty());
    assert!(instructions[0].help_requested);
    assert!(instructions[0].positional_arguments.is_empty());
}


#[test]
fn multiple_commands_separated_by_semicolon_path_and_help_check() {
    let parser = Parser::new(default_options());
    let input = "cmd1 ;; cmd2 sub ? ;; cmd3";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "parse_single_str failed for input '{}': {:?}", input, result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 3);

    assert_eq!(instructions[0].command_path_slices, vec!["cmd1".to_string()]);
    assert!(!instructions[0].help_requested);

    assert_eq!(instructions[1].command_path_slices, vec!["cmd2".to_string(), "sub".to_string()]);
    assert!(instructions[1].help_requested);

    assert_eq!(instructions[2].command_path_slices, vec!["cmd3".to_string()]);
    assert!(!instructions[2].help_requested);
}

#[test]
fn leading_semicolon_error() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str(";; cmd1");
    assert!(result.is_err(), "Expected error for leading ';;'");
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Empty instruction segment"));
    }
}

#[test]
fn trailing_semicolon_error_if_empty_segment_is_error() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd1 ;;");
    assert!(result.is_err(), "Expected error for trailing ';;' if empty segments are errors");
     if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::TrailingDelimiter)); // Updated to expect TrailingDelimiter
        assert!(e.to_string().contains("Empty instruction segment"));
    }
}

#[test]
fn multiple_consecutive_semicolons_error() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd1 ;;;; cmd2");
    assert!(result.is_err(), "Expected error for 'cmd1 ;;;; cmd2'");
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Empty instruction segment"));
    }
}

#[test]
fn only_semicolons_error() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str(";;");
    assert!(result.is_err(), "Expected error for ';;'");
     if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Empty instruction segment"));
    }
    let result_double = parser.parse_single_str(";;;;");
    assert!(result_double.is_err(), "Expected error for ';;;;'");
    if let Err(e) = result_double {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Empty instruction segment"));
    }
}

#[test]
fn single_command_slice_input_path_check() {
    let parser = Parser::new(default_options());
    let input: &[&str] = &["cmd", "arg"];
    let result = parser.parse_slice(input);
    assert!(result.is_ok(), "parse_slice failed for input '{:?}': {:?}", input, result.err());
    let instructions = result.unwrap();
    // Each string in the slice (not containing ";;") forms its own instruction.
    assert_eq!(instructions.len(), 2, "Expected 2 instructions from &[\"cmd\", \"arg\"]");

    let instr1 = &instructions[0];
    assert_eq!(instr1.command_path_slices, vec!["cmd".to_string()], "Instr1 path");
    assert!(instr1.positional_arguments.is_empty(), "Instr1 positional");
    assert!(instr1.named_arguments.is_empty(), "Instr1 named");
    assert!(!instr1.help_requested, "Instr1 help");

    let instr2 = &instructions[1];
    assert_eq!(instr2.command_path_slices, vec!["arg".to_string()], "Instr2 path (arg treated as command)");
    assert!(instr2.positional_arguments.is_empty(), "Instr2 positional");
    assert!(instr2.named_arguments.is_empty(), "Instr2 named");
    assert!(!instr2.help_requested, "Instr2 help");
}

#[test]
fn multiple_commands_slice_input_path_check() {
    let parser = Parser::new(default_options());
    let input: &[&str] = &["cmd1 path1", ";;", "cmd2", "?", ";;", "cmd3"];
    let result = parser.parse_slice(input);
    assert!(result.is_ok(), "parse_slice failed for input '{:?}': {:?}", input, result.err());
    let instructions = result.unwrap();
    // Expected:
    // 1. from "cmd1 path1" -> path ["cmd1", "path1"]
    // 2. from ";;" -> boundary
    // 3. from "cmd2" -> path ["cmd2"]
    // 4. from "?" -> path [], help true
    // 5. from ";;" -> boundary
    // 6. from "cmd3" -> path ["cmd3"]
    assert_eq!(instructions.len(), 4, "Expected 4 instructions from the slice input");

    assert_eq!(instructions[0].command_path_slices, vec!["cmd1".to_string(), "path1".to_string()], "Instr1 Path");
    assert!(!instructions[0].help_requested, "Instr1 Help");

    assert_eq!(instructions[1].command_path_slices, vec!["cmd2".to_string()], "Instr2 Path");
    assert!(!instructions[1].help_requested, "Instr2 Help should be false as '?' is next segment");

    assert!(instructions[2].command_path_slices.is_empty(), "Instr3 Path (from '?')");
    assert!(instructions[2].help_requested, "Instr3 Help (from '?')");

    assert_eq!(instructions[3].command_path_slices, vec!["cmd3".to_string()], "Instr4 Path");
    assert!(!instructions[3].help_requested, "Instr4 Help");
}

// Test for path ending before a delimiter like '::'
#[test]
fn path_stops_at_double_colon_delimiter() {
    let parser = Parser::new(default_options());
    let input = "cmd path arg::val";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    assert_eq!(instructions[0].command_path_slices, vec!["cmd".to_string(), "path".to_string()]);
    assert_eq!(instructions[0].named_arguments.len(), 1);
    assert!(instructions[0].named_arguments.contains_key("arg"));
    assert_eq!(instructions[0].named_arguments.get("arg").unwrap().value, "val");
    assert!(instructions[0].positional_arguments.is_empty());
}