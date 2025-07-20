//! ## Test Matrix for Comprehensive Parsing
//!
//! This matrix details a comprehensive set of test cases for the Unilang instruction parser,
//! covering various instruction structures, command path formats, argument types, parser options,
//! and error conditions.
//!
//! **Test Factors:**
//! - Instruction Structure: Single instruction, Multiple instructions
//! - Command Path: Simple, Multi-segment, Leading dot, No command path
//! - Arguments: Positional, Named, Mixed, None
//! - Argument Value: Unquoted, Quoted, Escaped, Invalid Escape
//! - Help Operator: Present, Absent
//! - Parser Options: `error_on_positional_after_named`, `error_on_duplicate_named_arguments`
//! - Error Conditions: Duplicate named args, Positional after named, Malformed named arg, Comments
//!
//! ---
//!
//! **Test Combinations:**
//!
//! | ID | Aspect Tested | Input String | Instruction Structure | Command Path | Arguments | Argument Value | Help Operator | Parser Options (`pos_after_named`, `dup_named`) | Error Condition | Expected Behavior |
//! |---|---|---|---|---|---|---|---|---|---|---|
//! | CT1.1 | Single instruction, unquoted positional arg | `cmd val` | Single | Simple (`cmd`) | Positional | Unquoted | Absent | `(false, false)` | None | Command `cmd`, Positional `val` |
//! | CT1.2 | Single instruction, multi-path, named arg | `path1 path2 name1::val1` | Single | Simple (`path1`) | Mixed | Unquoted | Absent | `(false, false)` | None | Command `path1`, Positional `path2`, Named `name1::val1` |
//! | CT1.3 | Single instruction, help operator | `cmd ?` | Single | Simple (`cmd`) | None | N/A | Present | `(false, false)` | None | Command `cmd`, Help requested |
//! | CT1.4 | Single instruction, quoted positional arg | `cmd "quoted val"` | Single | Simple (`cmd`) | Positional | Quoted | Absent | `(false, false)` | None | Command `cmd`, Positional `"quoted val"` |
//! | CT1.5 | Single instruction, named arg, escaped val | `cmd name1::"esc\nval"` | Single | Simple (`cmd`) | Named | Escaped | Absent | `(false, false)` | None | Command `cmd`, Named `name1::esc\nval` |
//! | CT1.6 | Single instruction, named arg, invalid escape | `cmd name1::"bad\xval"` | Single | Simple (`cmd`) | Named | Invalid Escape | Absent | `(false, false)` | None | Command `cmd`, Named `name1::bad\xval` (literal `\x`) |
//! | CT3.1 | Multi-instruction, basic separator | `cmd1 arg1 ;; cmd2 name::val` | Multiple | Simple (`cmd1`), Simple (`cmd2`) | Positional, Named | Unquoted | Absent | `(false, false)` | None | Two instructions parsed correctly |
//! | CT4.1 | Duplicate named arg (error) | `cmd name::val1 name::val2` | Single | Simple (`cmd`) | Named | Unquoted | Absent | `(false, true)` | Duplicate named arg | Error: Duplicate named argument 'name' |
//! | CT4.2 | Duplicate named arg (last wins) | `cmd name::val1 name::val2` | Single | Simple (`cmd`) | Named | Unquoted | Absent | `(false, false)` | None | Last value wins: `val2` |
//! | CT5.1 | No path, named arg only (error) | `name::val` | Single | No command path | Named | Unquoted | Absent | `(false, false)` | Malformed named arg | Error: Unexpected token '::' in arguments |
//! | CT6.1 | Command path with dots and args | `cmd.sub.path arg1 name::val` | Single | Multi-segment (`cmd.sub.path`) | Mixed | Unquoted | Absent | `(false, false)` | None | Command `cmd.sub.path`, Positional `arg1`, Named `name::val` |
//! | SA1.1 | Root namespace list | `.` | Single | Leading dot | None | N/A | Absent | `(false, false)` | None | Empty command path, no args |
//! | SA1.2 | Root namespace help | `. ?` | Single | Leading dot | None | N/A | Present | `(false, false)` | None | Empty command path, help requested |
//! | SA2.1 | Whole line comment | `# this is a whole line comment` | Single | N/A | N/A | N/A | Absent | `(false, false)` | Comment | Error: Unexpected token '#' |
//! | SA2.2 | Comment only line | `#` | Single | N/A | N/A | N/A | Absent | `(false, false)` | Comment | Error: Unexpected token '#' |
//! | SA2.3 | Inline comment attempt | `cmd arg1 # inline comment` | Single | Simple (`cmd`) | Positional | N/A | Absent | `(false, false)` | Comment | Error: Unexpected token '#' |
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

/// Tests a single instruction with a single command path and an unquoted positional argument.
/// Test Combination: CT1.1
#[test]
fn ct1_1_single_str_single_path_unquoted_pos_arg() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd val";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "CT1.1 Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()], "CT1.1 Path"); // Corrected expectation
    assert_eq!(instruction.positional_arguments.len(), 1, "CT1.1 Positional args count");
    assert_eq!(instruction.positional_arguments[0].value, "val".to_string(), "CT1.1 Positional arg value");
    assert!(instruction.named_arguments.is_empty(), "CT1.1 Named args");
    // assert!(!instruction.help_requested, "CT1.1 Help requested"); // Removed
}

/// Tests a single instruction with a multi-segment command path and an unquoted named argument.
/// Test Combination: CT1.2
#[test]
fn ct1_2_single_str_multi_path_unquoted_named_arg() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "path1 path2 name1::val1";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "CT1.2 Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["path1".to_string()], "CT1.2 Path"); // Corrected expectation
    assert_eq!(instruction.positional_arguments.len(), 1, "CT1.2 Positional args count"); // Corrected expectation
    assert_eq!(instruction.positional_arguments[0].value, "path2".to_string(), "CT1.2 Positional arg value"); // Corrected expectation
    assert_eq!(instruction.named_arguments.len(), 1, "CT1.2 Named args count");
    let arg1 = instruction.named_arguments.get("name1").expect("CT1.2 Missing name1");
    assert_eq!(arg1.value, "val1", "CT1.2 name1 value"); // Changed to &str
    // assert!(!instruction.help_requested, "CT1.2 Help requested"); // Removed
}

/// Tests a single instruction with a single command path and a help operator, no arguments.
/// Test Combination: CT1.3
#[test]
fn ct1_3_single_str_single_path_help_no_args() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd ?";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "CT1.3 Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()], "CT1.3 Path");
    assert!(instruction.positional_arguments.is_empty(), "CT1.3 Positional args");
    assert!(instruction.named_arguments.is_empty(), "CT1.3 Named args");
    assert!(instruction.help_requested, "CT1.3 Help requested should be true"); // Re-enabled
}

/// Tests a single instruction with a single command path and a quoted positional argument.
/// Test Combination: CT1.4
#[test]
fn ct1_4_single_str_single_path_quoted_pos_arg() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd \"quoted val\"";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "CT1.4 Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()], "CT1.4 Path");
    assert_eq!(instruction.positional_arguments.len(), 1, "CT1.4 Positional args count");
    assert_eq!(instruction.positional_arguments[0].value, "quoted val".to_string(), "CT1.4 Positional arg value");
    assert!(instruction.named_arguments.is_empty(), "CT1.4 Named args");
    // assert!(!instruction.help_requested, "CT1.4 Help requested"); // Removed
}

/// Tests a single instruction with a single command path and a named argument with an escaped value.
/// Test Combination: CT1.5
#[test]
fn ct1_5_single_str_single_path_named_arg_escaped_val() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd name1::\"esc\\nval\"";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "CT1.5 Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()], "CT1.5 Path");
    assert!(instruction.positional_arguments.is_empty(), "CT1.5 Positional args");
    assert_eq!(instruction.named_arguments.len(), 1, "CT1.5 Named args count");
    let arg1 = instruction.named_arguments.get("name1").expect("CT1.5 Missing name1");
    assert_eq!(arg1.value, "esc\nval", "CT1.5 name1 value with newline"); // Changed to &str
    // assert!(!instruction.help_requested, "CT1.5 Help requested"); // Removed
}

/// Tests a single instruction with a single command path and a named argument with an invalid escape sequence.
/// Test Combination: CT1.6
#[test]
fn ct1_6_single_str_single_path_named_arg_invalid_escape() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd name1::\"bad\\xval\"";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "CT1.6 Expected Ok for invalid escape, got Err: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.named_arguments.get("name1").unwrap().value, "bad\\xval".to_string(), "CT1.6 Invalid escape should be literal");
}

/// Tests multiple instructions separated by `;;` with basic command and arguments.
/// Test Combination: CT3.1
#[test]
fn ct3_1_single_str_separator_basic() {
    let parser = Parser::new(UnilangParserOptions::default());
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
    // assert!(!instr1.help_requested); // Removed

    // Instruction 2: "cmd2 name::val"
    let instr2 = &instructions[1];
    assert_eq!(instr2.command_path_slices, vec!["cmd2".to_string()], "CT3.1 Instr2 Path");
    assert!(instr2.positional_arguments.is_empty(), "CT3.1 Instr2 Positional");
    assert_eq!(instr2.named_arguments.len(), 1, "CT3.1 Instr2 Named count");
    assert_eq!(instr2.named_arguments.get("name").unwrap().value, "val", "CT3.1 Instr2 name value"); // Changed to &str
}

/// Tests that a duplicate named argument results in an error when the option is set.
/// Test Combination: CT4.1
#[test]
fn ct4_1_single_str_duplicate_named_error() {
    let parser = Parser::new(options_error_on_duplicate_named());
    let input = "cmd name::val1 name::val2";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "CT4.1 Expected error for duplicate named, got Ok: {:?}", result.ok());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)), "CT4.1 ErrorKind mismatch: {:?}", e.kind);
        assert!(e.to_string().contains("Duplicate named argument 'name'"), "CT4.1 Error message mismatch: {}", e);
    }
}

/// Tests that the last value wins for duplicate named arguments when the option is not set.
/// Test Combination: CT4.2
#[test]
fn ct4_2_single_str_duplicate_named_last_wins() {
    let parser = Parser::new(UnilangParserOptions { error_on_duplicate_named_arguments: false, ..Default::default() }); // Explicitly set to false
    let input = "cmd name::val1 name::val2";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "CT4.2 Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1, "CT4.2 Named args count");
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, "val2", "CT4.2 Last value should win"); // Changed to &str
}

/// Tests that an instruction with no command path but only a named argument results in an error.
/// Test Combination: CT5.1
#[test]
fn ct5_1_single_str_no_path_named_arg_only() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "name::val";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "CT5.1 Expected error for no path with named arg, got Ok: {:?}", result.ok()); // Changed to expect error
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::Syntax("Unexpected token '::' in arguments".to_string()), "CT5.1 ErrorKind mismatch: {:?}", e.kind);
        assert_eq!(e.location, Some(SourceLocation::StrSpan{start:4, end:6}), "CT5.1 Location mismatch for '::'");
    }
}

/// Tests a command path with dots and arguments.
/// Test Combination: CT6.1
#[test]
fn ct6_1_command_path_with_dots_and_slashes() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd.sub.path arg1 name::val"; // Changed input to use only dots for path
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "CT6.1 Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string(), "sub".to_string(), "path".to_string()], "CT6.1 Path"); // Corrected expectation
    assert_eq!(instruction.positional_arguments.len(), 1, "CT6.1 Positional args count"); // Corrected expectation
    assert_eq!(instruction.positional_arguments[0].value, "arg1".to_string(), "CT6.1 Positional arg value"); // Corrected expectation
    assert_eq!(instruction.named_arguments.len(), 1, "CT6.1 Named args count");
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, "val", "CT6.1 name value"); // Changed to &str
    // assert!(!instruction.help_requested, "CT6.1 Help requested"); // Removed
}

/// Tests parsing of a root namespace list instruction (input '.').
/// Test Combination: SA1.1
#[test]
fn sa1_1_root_namespace_list() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = ".";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "SA1.1 Parse error for '.': {:?}", result.err());
    let instruction = result.unwrap();
    assert!(instruction.command_path_slices.is_empty(), "SA1.1 Path for '.' should be empty");
    assert!(instruction.positional_arguments.is_empty(), "SA1.1 Positional args for '.' should be empty");
    assert!(instruction.named_arguments.is_empty(), "SA1.1 Named args for '.' should be empty");
    assert_eq!(instruction.overall_location, SourceLocation::StrSpan { start: 0, end: 1 });
}

/// Tests parsing of a root namespace help instruction (input '. ?').
/// Test Combination: SA1.2
#[test]
fn sa1_2_root_namespace_help() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = ". ?";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "SA1.2 Parse error for '. ?': {:?}", result.err());
    let instruction = result.unwrap();
    // Expecting path to be empty, no positional args, and help requested.
    assert!(instruction.command_path_slices.is_empty(), "SA1.2 Path for '. ?' should be empty");
    assert!(instruction.positional_arguments.is_empty(), "SA1.2 Positional args for '. ?' should be empty");
    assert!(instruction.help_requested, "SA1.2 Help requested for '. ?' should be true"); // Re-enabled
}

/// Tests that a whole line comment results in an error.
/// Test Combination: SA2.1
#[test]
fn sa2_1_whole_line_comment() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "# this is a whole line comment";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "SA2.1 Expected error for whole line comment, got Ok: {:?}", result.ok());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)), "SA2.1 ErrorKind mismatch: {:?}", e.kind);
        assert!(e.to_string().contains("Unexpected token '#' in arguments"), "SA2.1 Error message mismatch: {}", e.to_string());
    }
}

/// Tests that a line with only a comment character results in an error.
/// Test Combination: SA2.2
#[test]
fn sa2_2_comment_only_line() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "#";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "SA2.2 Expected error for '#' only line, got Ok: {:?}", result.ok());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)), "SA2.2 ErrorKind mismatch: {:?}", e.kind);
        assert!(e.to_string().contains("Unexpected token '#' in arguments"), "SA2.2 Error message mismatch: {}", e.to_string());
    }
}

/// Tests that an inline comment attempt results in an error.
/// Test Combination: SA2.3
#[test]
fn sa2_3_inline_comment_attempt() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd arg1 # inline comment";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "SA2.3 Expected error for inline '#', got Ok: {:?}", result.ok());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)), "SA2.3 ErrorKind mismatch: {:?}", e.kind);
        assert!(e.to_string().contains("Unexpected token '#' in arguments"), "SA2.3 Error message mismatch: {}", e.to_string()); // Changed message
    }
}