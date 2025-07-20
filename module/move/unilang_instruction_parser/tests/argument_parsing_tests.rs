//! ## Test Matrix for Argument Parsing
//!
//! This matrix details the test cases for parsing arguments, covering positional, named, and mixed argument scenarios,
//! as well as various parser options and malformed inputs.
//!
//! **Test Factors:**
//! - Argument Type: Positional, Named, Mixed
//! - Argument Order: Positional first, Named first, Positional after Named
//! - Parser Options: `error_on_positional_after_named` (true/false), `error_on_duplicate_named_arguments` (true/false)
//! - Argument Value: Normal, Quoted, Escaped, Empty
//! - Argument Format: Correct, Malformed (missing delimiter, missing value, missing name)
//! - Duplicate Named Arguments: Yes/No
//!
//! ---
//!
//! **Test Combinations:**
//!
//! | ID   | Aspect Tested | Input Example | Argument Type | Argument Order | Parser Options (`pos_after_named`, `dup_named`) | Argument Value | Argument Format | Duplicate Named | Expected Behavior |
//! |------|---------------|---------------|---------------|----------------|-------------------------------------------------|----------------|-----------------|-----------------|-------------------|
//! | T1.1 | Positional args | `cmd pos1 pos2` | Positional | N/A | `(false, false)` | Normal | Correct | No | Command `cmd`, Positional `pos1`, `pos2` |
//! | T1.2 | Named args | `cmd name1::val1 name2::val2` | Named | N/A | `(false, false)` | Normal | Correct | No | Command `cmd`, Named `name1::val1`, `name2::val2` |
//! | T1.3 | Mixed args (pos first) | `cmd pos1 name1::val1 pos2` | Mixed | Positional first | `(false, false)` | Normal | Correct | No | Command `cmd`, Positional `pos1`, `pos2`, Named `name1::val1` |
//! | T1.4 | Positional after named (error) | `cmd name1::val1 pos1` | Mixed | Named first | `(true, false)` | Normal | Correct | No | Error: Positional after named |
//! | T1.5 | Positional after named (ok) | `cmd name1::val1 pos1` | Mixed | Named first | `(false, false)` | Normal | Correct | No | Command `cmd`, Positional `pos1`, Named `name1::val1` |
//! | T1.6 | Named arg empty value (no quotes) | `cmd name::` | Named | N/A | `(false, false)` | Empty | Malformed (missing value) | No | Error: Expected value for named arg |
//! | T1.7 | Malformed named arg (delimiter as value) | `cmd name::?` | Named | N/A | `(false, false)` | Operator | Malformed (delimiter as value) | No | Error: Expected value for named arg |
//! | T1.8 | Named arg missing name | `::value` | Named | N/A | `(false, false)` | Normal | Malformed (missing name) | No | Error: Unexpected token '::' |
//! | T1.9 | Unescaping named arg value | `cmd name::"a\\\\b\\\"c'd"` | Named | N/A | `(false, false)` | Escaped | Correct | No | Value unescaped: `a\b"c'd` |
//! | T1.10 | Unescaping positional arg value | `cmd "a\\\\b\\\"c'd\\ne\\tf"` | Positional | N/A | `(false, false)` | Escaped | Correct | No | Value unescaped: `a\b"c'd\ne\tf` |
//! | T1.11 | Duplicate named arg (error) | `cmd name::val1 name::val2` | Named | N/A | `(false, true)` | Normal | Correct | Yes | Error: Duplicate named arg |
//! | T1.12 | Duplicate named arg (last wins) | `cmd name::val1 name::val2` | Named | N/A | `(false, false)` | Normal | Correct | Yes | Last value wins: `val2` |
//! | T1.13 | Complex mixed args | `path sub name::val pos1` | Mixed | Positional first | `(false, false)` | Normal | Correct | No | Command `path`, Positional `sub`, `pos1`, Named `name::val` |
//! | T1.14 | Named arg with quoted escaped value location | `cmd key::"value with \\"quotes\\" and \\\\slash\\\\"` | Named | N/A | `(false, false)` | Escaped | Correct | No | Value unescaped: `value with "quotes" and \slash\` |
//! | T1.15 | Positional arg with quoted escaped value location | `cmd "a\\\\b\\\"c'd\\ne\\tf"` | Positional | N/A | `(false, false)` | Escaped | Correct | No | Value unescaped: `a\b"c'd\ne\tf` |
//! | T1.16 | Malformed named arg (no delimiter) | `cmd name value` | Positional | N/A | `(false, false)` | Normal | Malformed (no delimiter) | No | Treated as positional args |
use unilang_instruction_parser::*;
// use std::collections::HashMap; // Re-enable for named argument tests
use unilang_instruction_parser::error::ErrorKind;



fn options_error_on_positional_after_named() -> UnilangParserOptions {
    UnilangParserOptions {
        error_on_positional_after_named: true,
        ..Default::default()
    }
}

fn options_allow_positional_after_named() -> UnilangParserOptions {
    UnilangParserOptions {
        error_on_positional_after_named: false,
        ..Default::default()
    }
}

fn options_allow_duplicate_named() -> UnilangParserOptions {
    UnilangParserOptions {
        error_on_duplicate_named_arguments: false,
        ..Default::default()
    }
}


/// Tests that a command with only positional arguments is fully parsed.
/// Test Combination: T1.1
#[test]
fn command_with_only_positional_args_fully_parsed() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd pos1 pos2";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instruction = result.unwrap();

    // Command path should only be "cmd" as spaces separate command from args
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 2);
    assert_eq!(instruction.positional_arguments[0].value, "pos1".to_string());
    assert_eq!(instruction.positional_arguments[1].value, "pos2".to_string());
    assert!(instruction.named_arguments.is_empty());
}

/// Tests that a command with only named arguments is fully parsed.
/// Test Combination: T1.2
#[test]
fn command_with_only_named_args_fully_parsed() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd name1::val1 name2::val2";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instruction = result.unwrap();

    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.positional_arguments.is_empty());
    assert_eq!(instruction.named_arguments.len(), 2);

    let arg1 = instruction.named_arguments.get("name1").unwrap();
    assert_eq!(arg1.value, "val1");

    let arg2 = instruction.named_arguments.get("name2").unwrap();
    assert_eq!(arg2.value, "val2");
}

/// Tests that a command with mixed arguments (positional first) is fully parsed.
/// Test Combination: T1.3
#[test]
fn command_with_mixed_args_positional_first_fully_parsed() {
    let parser = Parser::new(options_allow_positional_after_named());
    let input = "cmd pos1 name1::val1 pos2 name2::val2";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instruction = result.unwrap();

    // Command path should only be "cmd" as spaces separate command from args
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);

    assert_eq!(instruction.positional_arguments.len(), 2);
    assert_eq!(instruction.positional_arguments[0].value, "pos1".to_string());
    assert_eq!(instruction.positional_arguments[1].value, "pos2".to_string());

    assert_eq!(instruction.named_arguments.len(), 2);
    let named_arg1 = instruction.named_arguments.get("name1").unwrap();
    assert_eq!(named_arg1.value, "val1");

    let named_arg2 = instruction.named_arguments.get("name2").unwrap();
    assert_eq!(named_arg2.value, "val2");
}

/// Tests that a positional argument after a named argument results in an error when the option is set.
/// Test Combination: T1.4
#[test]
fn command_with_mixed_args_positional_after_named_error_when_option_set() {
    let parser = Parser::new(options_error_on_positional_after_named());
    let input = "cmd name1::val1 pos1";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for positional after named, but got Ok: {:?}", result.ok());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Positional argument after named argument"), "Error message mismatch: {}", e);
    }
}

/// Tests that a positional argument after a named argument is allowed when the option is not set.
/// Test Combination: T1.5
#[test]
fn command_with_mixed_args_positional_after_named_ok_when_option_not_set() {
    let parser = Parser::new(options_allow_positional_after_named());
    let input = "cmd name1::val1 pos1";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instruction = result.unwrap();

    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "pos1".to_string());
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("name1").unwrap().value, "val1");
}


/// Tests that a named argument with an empty value (no quotes) results in an error.
/// Test Combination: T1.6
#[test]
fn named_arg_with_empty_value_no_quotes_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd name::";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Expected value for named argument 'name' but found end of instruction"), "Error message mismatch: {}", e);
    }
}

/// Tests that a malformed named argument (delimiter as value) results in an error.
/// Test Combination: T1.7
#[test]
fn malformed_named_arg_name_delimiter_operator() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd name::?";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::Syntax("Expected value for named argument 'name'".to_string()));
    }
}

/// Tests that a named argument missing its name results in an error.
/// Test Combination: T1.8
#[test]
fn named_arg_missing_name_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "::value";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Unexpected token '::' in arguments"));
    }
}



/// Tests that unescaping works correctly for a named argument value.
/// Test Combination: T1.9
#[test]
fn unescaping_works_for_named_arg_value() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd name::\"a\\\\b\\\"c'd\""; // Removed invalid escape sequence \'
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, "a\\b\"c'd");
}

/// Tests that unescaping works correctly for a positional argument value.
/// Test Combination: T1.10
#[test]
fn unescaping_works_for_positional_arg_value() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd \"a\\\\b\\\"c'd\\ne\\tf\""; // Removed invalid escape sequence \'
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "a\\b\"c'd\ne\tf");
}

/// Tests that a duplicate named argument results in an error when the option is set.
/// Test Combination: T1.11
#[test]
fn duplicate_named_arg_error_when_option_set() {
    let parser = Parser::new(UnilangParserOptions { error_on_duplicate_named_arguments: true, ..Default::default() });
    let input = "cmd name::val1 name::val2";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Duplicate named argument 'name'"), "Error message mismatch: {}", e);
    }
}

/// Tests that the last value wins for duplicate named arguments when the option is not set.
/// Test Combination: T1.12
#[test]
fn duplicate_named_arg_last_wins_by_default() {
    let parser = Parser::new(options_allow_duplicate_named()); // Use the new options
    let input = "cmd name::val1 name::val2";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse error for duplicate named (last wins): {:?}", result.err());
    let instruction = result.unwrap();

    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1, "CT4.2 Named args count");
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, "val2");
}

/// Tests a complex instruction with command path and mixed arguments.
/// Test Combination: T1.13
#[test]
fn command_with_path_and_args_complex_fully_parsed() {
    let parser = Parser::new(options_allow_positional_after_named());
    let input = "path sub name::val pos1";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instruction = result.unwrap();

    assert_eq!(instruction.command_path_slices, vec!["path".to_string()]);

    assert_eq!(instruction.positional_arguments.len(), 2);
    assert_eq!(instruction.positional_arguments[0].value, "sub".to_string());
    assert_eq!(instruction.positional_arguments[1].value, "pos1".to_string());

    let named_arg = instruction.named_arguments.get("name").unwrap();
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(named_arg.value, "val");
}

/// Tests that a named argument with a quoted and escaped value is parsed correctly, including its location.
/// Test Combination: T1.14
#[test]
fn named_arg_with_quoted_escaped_value_location() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd key::\"value with \\\"quotes\\\" and \\\\slash\\\\\"";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instruction = result.unwrap();

    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1);
    let arg = instruction.named_arguments.get("key").unwrap();
    assert_eq!(arg.value, "value with \"quotes\" and \\slash\\");
}

/// Tests that a positional argument with a quoted and escaped value is parsed correctly, including its location.
/// Test Combination: T1.15
#[test]
fn positional_arg_with_quoted_escaped_value_location() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd \"a\\\\b\\\"c'd\\ne\\tf\""; // Removed invalid escape
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "a\\b\"c'd\ne\tf");
}

/// Tests that a malformed named argument (missing delimiter) is treated as positional arguments.
/// Test Combination: T1.16
#[test]
fn malformed_named_arg_name_value_no_delimiter() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd name value";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 2);
    assert_eq!(instruction.positional_arguments[0].value, "name".to_string());
    assert_eq!(instruction.positional_arguments[1].value, "value".to_string());
    assert!(instruction.named_arguments.is_empty());
}
