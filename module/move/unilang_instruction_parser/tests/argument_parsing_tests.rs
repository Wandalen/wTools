//! Tests for argument parsing logic.
use unilang_instruction_parser::*;
// use std::collections::HashMap; // Re-enable for named argument tests
use unilang_instruction_parser::error::{ErrorKind, SourceLocation};

fn default_options() -> UnilangParserOptions {
    UnilangParserOptions::default()
}

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

fn options_error_on_duplicate_named() -> UnilangParserOptions {
    UnilangParserOptions {
        error_on_duplicate_named_arguments: true,
        ..Default::default()
    }
}


#[test]
fn command_with_only_positional_args_fully_parsed() {
    let parser = Parser::new(default_options());
    let input = "cmd pos1 pos2";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string(), "pos1".to_string(), "pos2".to_string()]);
    assert!(instruction.positional_arguments.is_empty());
    assert!(instruction.named_arguments.is_empty());
}

#[test]
fn command_with_only_named_args_fully_parsed() {
    let parser = Parser::new(default_options());
    let input = "cmd name1::val1 name2::val2";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.positional_arguments.is_empty());
    assert_eq!(instruction.named_arguments.len(), 2);

    let arg1 = instruction.named_arguments.get("name1").unwrap();
    assert_eq!(arg1.value, "val1".to_string());
    assert_eq!(arg1.name, Some("name1".to_string()));
    assert_eq!(arg1.name_location, Some(SourceLocation::StrSpan { start: 4, end: 9 }));
    assert_eq!(arg1.value_location, SourceLocation::StrSpan { start: 11, end: 15 });

    let arg2 = instruction.named_arguments.get("name2").unwrap();
    assert_eq!(arg2.value, "val2".to_string());
    assert_eq!(arg2.name, Some("name2".to_string()));
    assert_eq!(arg2.name_location, Some(SourceLocation::StrSpan { start: 16, end: 21 }));
    assert_eq!(arg2.value_location, SourceLocation::StrSpan { start: 23, end: 27 });
}

#[test]
fn command_with_mixed_args_positional_first_fully_parsed() {
    let parser = Parser::new(options_allow_positional_after_named());
    let input = "cmd pos1 name1::val1 pos2 name2::val2";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string(), "pos1".to_string()]);

    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "pos2".to_string());
    assert_eq!(instruction.positional_arguments[0].value_location, SourceLocation::StrSpan{start:21, end:25});


    assert_eq!(instruction.named_arguments.len(), 2);
    let named_arg1 = instruction.named_arguments.get("name1").unwrap();
    assert_eq!(named_arg1.value, "val1".to_string());
    assert_eq!(named_arg1.name, Some("name1".to_string()));
    assert_eq!(named_arg1.name_location, Some(SourceLocation::StrSpan{start:9, end:14}));
    assert_eq!(named_arg1.value_location, SourceLocation::StrSpan{start:16, end:20});

    let named_arg2 = instruction.named_arguments.get("name2").unwrap();
    assert_eq!(named_arg2.value, "val2".to_string());
    assert_eq!(named_arg2.name, Some("name2".to_string()));
    assert_eq!(named_arg2.name_location, Some(SourceLocation::StrSpan{start:26, end:31}));
    assert_eq!(named_arg2.value_location, SourceLocation::StrSpan{start:33, end:37});
}

#[test]
fn command_with_mixed_args_positional_after_named_error_when_option_set() {
    let parser = Parser::new(options_error_on_positional_after_named());
    let input = "cmd name1::val1 pos1";
    let result = parser.parse_single_str(input);
    assert!(result.is_err(), "Expected error for positional after named, but got Ok: {:?}", result.ok());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Positional argument encountered after a named argument."), "Error message mismatch: {}", e);
        assert_eq!(e.location, Some(SourceLocation::StrSpan{start: 16, end: 20}));
    }
}

#[test]
fn command_with_mixed_args_positional_after_named_ok_when_option_not_set() {
    let parser = Parser::new(options_allow_positional_after_named());
    let input = "cmd name1::val1 pos1";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "pos1".to_string());
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("name1").unwrap().value, "val1".to_string());
}


#[test]
fn named_arg_with_empty_value_no_quotes_error() {
    let parser = Parser::new(default_options());
    let input = "cmd name::";
    let result = parser.parse_single_str(input);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Expected value for named argument 'name' but found end of instruction"), "Error message mismatch: {}", e);
        assert_eq!(e.location, Some(SourceLocation::StrSpan{start:4, end:8}));
    }
}

#[test]
fn named_arg_missing_name_error() {
    let parser = Parser::new(default_options());
    let input = "::value";
    let result = parser.parse_single_str(input);
    assert!(result.is_err(), "Test 'named_arg_missing_name_error' failed. Expected Err, got Ok for input: '{}'. Result: {:?}", input, result);
     if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)), "ErrorKind mismatch: {:?}", e.kind);
        assert!(e.to_string().contains("Unexpected '::' without preceding argument name"), "Error message mismatch: {}", e);
        assert_eq!(e.location, Some(SourceLocation::StrSpan{start:0, end:2}), "Location mismatch for '::value'");
    }
}

#[test]
fn unexpected_operator_in_args() {
    let parser = Parser::new(default_options());
    let input = "cmd arg1 ?";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "Expected Ok for 'cmd arg1 ?' as help request, got Err: {:?}", result.err());
    let instructions = result.unwrap();
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string(), "arg1".to_string()]);
    assert!(instruction.help_requested);
}

// Ignored due to external bug in strs_tools tokenization of escaped quotes. See strs_tools/task.md#TASK-YYYYMMDD-HHMMSS-UnescapingBug (Task ID to be updated)
// aaa: Kept ignored due to external strs_tools bug (see task.md in strs_tools). Un-ignoring and attempting fix confirmed external dependency.
#[ignore]
#[test]
fn unescaping_works_for_named_arg_value() {
    let parser = Parser::new(default_options());
    let input = "cmd name::\"a\\\\b\\\"c\\\'d\\ne\\tf\"";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1);
    let arg = instruction.named_arguments.get("name").unwrap();
    assert_eq!(arg.value, "a\\b\"c\'d\ne\tf".to_string());
    assert_eq!(arg.name, Some("name".to_string()));
    assert_eq!(arg.name_location, Some(SourceLocation::StrSpan{start:4, end:8}));
    assert_eq!(arg.value_location, SourceLocation::StrSpan{start:10, end:26});
    assert!(instruction.positional_arguments.is_empty());
}

// Ignored due to external bug in strs_tools tokenization of escaped quotes. See strs_tools/task.md#TASK-YYYYMMDD-HHMMSS-UnescapingBug (Task ID to be updated)
// aaa: Kept ignored due to external strs_tools bug (see task.md in strs_tools). Un-ignoring and attempting fix confirmed external dependency.
#[ignore]
#[test]
fn unescaping_works_for_positional_arg_value() {
    let parser = Parser::new(default_options());
    let input = "cmd \"a\\\\b\\\"c\\\'d\\ne\\tf\"";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "a\\b\"c\'d\ne\tf".to_string());
    assert_eq!(instruction.positional_arguments[0].value_location, SourceLocation::StrSpan{start:4, end:20});
}

#[test]
fn duplicate_named_arg_error_when_option_set() {
    let parser = Parser::new(options_error_on_duplicate_named());
    let input = "cmd name::val1 name::val2";
    let result = parser.parse_single_str(input);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Duplicate named argument: name"), "Error message mismatch: {}", e);
        assert_eq!(e.location, Some(SourceLocation::StrSpan{start:15, end:19}));
    }
}

#[test]
fn duplicate_named_arg_last_wins_by_default() {
    let parser = Parser::new(default_options());
    let input = "cmd name::val1 name::val2";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "Parse error for duplicate named (last wins): {:?}", result.err());
    let instructions = result.unwrap();
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, "val2".to_string());
    assert_eq!(instruction.named_arguments.get("name").unwrap().name, Some("name".to_string()));
}

#[test]
fn command_with_path_and_args_complex_fully_parsed() {
    let parser = Parser::new(options_allow_positional_after_named());
    let input = "path sub name::val pos1";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["path".to_string(), "sub".to_string()], "Path should be ['path', 'sub']");

    assert_eq!(instruction.positional_arguments.len(), 1, "Should have 1 positional argument");
    assert_eq!(instruction.positional_arguments[0].value, "pos1".to_string());
    assert_eq!(instruction.positional_arguments[0].value_location, SourceLocation::StrSpan{start:19, end:23});


    assert_eq!(instruction.named_arguments.len(), 1);
    let named_arg = instruction.named_arguments.get("name").unwrap();
    assert_eq!(named_arg.value, "val".to_string());
    assert_eq!(named_arg.name, Some("name".to_string()));
    assert_eq!(named_arg.name_location, Some(SourceLocation::StrSpan{start:9, end:13}));
    assert_eq!(named_arg.value_location, SourceLocation::StrSpan{start:15, end:18});
}

// Ignored due to external bug in strs_tools tokenization of escaped quotes. See strs_tools/task.md#TASK-YYYYMMDD-HHMMSS-UnescapingBug (Task ID to be updated)
// aaa: Kept ignored due to external strs_tools bug (see task.md in strs_tools). Un-ignoring and attempting fix confirmed external dependency.
#[ignore]
#[test]
fn named_arg_with_quoted_escaped_value_location() {
    let parser = Parser::new(default_options());
    let input = "cmd key::\"value with \\\"quotes\\\" and \\\\slash\\\\\"";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.positional_arguments.is_empty());
    assert_eq!(instruction.named_arguments.len(), 1);
    let arg = instruction.named_arguments.get("key").unwrap();
    assert_eq!(arg.value, "value with \"quotes\" and \\slash\\".to_string());
    assert_eq!(arg.name, Some("key".to_string()));
    assert_eq!(arg.name_location, Some(SourceLocation::StrSpan{start:4, end:7}));
    assert_eq!(arg.value_location, SourceLocation::StrSpan{start:9, end:42});
}

// Ignored due to external bug in strs_tools tokenization of escaped quotes. See strs_tools/task.md#TASK-YYYYMMDD-HHMMSS-UnescapingBug (Task ID to be updated)
// aaa: Kept ignored due to external strs_tools bug (see task.md in strs_tools). Un-ignoring and attempting fix confirmed external dependency.
#[ignore]
#[test]
fn positional_arg_with_quoted_escaped_value_location() {
    let parser = Parser::new(default_options());
    let input = "cmd \"a\\\\b\\\"c\\\'d\\ne\\tf\"";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    let arg = &instruction.positional_arguments[0];
    assert_eq!(arg.value, "a\\b\"c\'d\ne\tf".to_string());
    assert_eq!(arg.value_location, SourceLocation::StrSpan{start:4, end:37});
    assert!(instruction.named_arguments.is_empty());
}

#[test]
fn malformed_named_arg_name_value_no_delimiter() {
    let parser = Parser::new(default_options());
    let input = "cmd name value";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string(), "name".to_string(), "value".to_string()]);
    assert!(instruction.positional_arguments.is_empty());
    assert!(instruction.named_arguments.is_empty());
}

#[test]
fn malformed_named_arg_name_delimiter_operator() {
    let parser = Parser::new(default_options());
    let input = "cmd name::?";
    let result = parser.parse_single_str(input);
    assert!(result.is_err(), "Expected error for named arg value as operator, but got Ok: {:?}", result.ok());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Expected value for named argument 'name' but found Operator(\"?\")"), "Error message mismatch: {}", e);
        assert_eq!(e.location, Some(SourceLocation::StrSpan{start:10, end:11}));
    }
}

#[test]
fn help_operator_after_args_is_error() {
    let parser = Parser::new(default_options());
    // This case is now handled by `unexpected_operator_in_args` which expects Ok & help_requested=true
    // let input = "cmd arg1 ?";
    // let result = parser.parse_single_str(input);
    // assert!(result.is_ok(), "Expected Ok for 'cmd arg1 ?' as help request, got Err: {:?}", result.err());
    // let instructions = result.unwrap();
    // let instruction = &instructions[0];
    // assert_eq!(instruction.command_path_slices, vec!["cmd".to_string(), "arg1".to_string()]);
    // assert!(instruction.help_requested);
    // assert!(instruction.positional_arguments.is_empty());
    // assert!(instruction.named_arguments.is_empty());

    let input2 = "cmd name::val ?"; // Path "cmd", named "name:val", then '?' is unexpected by arg parser.
    let result2 = parser.parse_single_str(input2);
    assert!(result2.is_err(), "Expected Err for 'cmd name::val ?', got Ok: {:?}", result2.ok());
    if let Err(e) = result2 {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Unexpected help operator '?' amidst arguments."), "Error message mismatch for input2: {}", e);
        assert_eq!(e.location, Some(SourceLocation::StrSpan{start:14, end:15})); // Location of '?'
    }
}

// Temporary tests for Sub-Increment 5.1.2 & 5.1.3 (Now removed)
// ...