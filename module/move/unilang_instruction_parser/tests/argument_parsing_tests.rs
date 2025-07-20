//! Tests for argument parsing logic.
use unilang_instruction_parser::*;
// use std::collections::HashMap; // Re-enable for named argument tests
use unilang_instruction_parser::error::{ErrorKind, SourceLocation};



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



#[test]
fn unescaping_works_for_named_arg_value() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd name::\"a\\\\b\\\"c'd\""; // Removed invalid escape sequence \'
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, "a\\b\"c'd");
}

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


