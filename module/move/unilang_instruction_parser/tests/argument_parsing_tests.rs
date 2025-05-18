//! Tests for argument parsing logic.
use unilang_instruction_parser::*;
use std::collections::HashMap; // Re-enable for named argument tests
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
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 2);
    assert_eq!(instruction.positional_arguments[0].value, "pos1".to_string());
    assert_eq!(instruction.positional_arguments[0].name, None);
    assert_eq!(instruction.positional_arguments[0].value_location, SourceLocation::StrSpan { start: 4, end: 8 });
    assert_eq!(instruction.positional_arguments[1].value, "pos2".to_string());
    assert_eq!(instruction.positional_arguments[1].name, None);
    assert_eq!(instruction.positional_arguments[1].value_location, SourceLocation::StrSpan { start: 9, end: 13 });
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
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);

    assert_eq!(instruction.positional_arguments.len(), 2);
    assert_eq!(instruction.positional_arguments[0].value, "pos1".to_string());
    assert_eq!(instruction.positional_arguments[0].value_location, SourceLocation::StrSpan{start:4, end:8});
    assert_eq!(instruction.positional_arguments[1].value, "pos2".to_string());
    assert_eq!(instruction.positional_arguments[1].value_location, SourceLocation::StrSpan{start:21, end:25});

    assert_eq!(instruction.named_arguments.len(), 2);
    let named_arg1 = instruction.named_arguments.get("name1").unwrap();
    assert_eq!(named_arg1.value, "val1".to_string());
    assert_eq!(named_arg1.name, Some("name1".to_string()));
    assert_eq!(named_arg1.name_location, Some(SourceLocation::StrSpan{start:9, end:14}));
    assert_eq!(named_arg1.value_location, SourceLocation::StrSpan{start:16, end:20});

    let named_arg2 = instruction.named_arguments.get("name2").unwrap();
    assert_eq!(named_arg2.value, "val2".to_string());
    assert_eq!(named_arg2.name, Some("name2".to_string()));
    assert_eq!(named_arg2.name_location, Some(SourceLocation::StrSpan{start:26, end:31})); // Corrected expected location
    assert_eq!(named_arg2.value_location, SourceLocation::StrSpan{start:33, end:37}); // Corrected expected location (val2 in "name2::val2")
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
    let input = "cmd ::value";
    let result = parser.parse_single_str(input);
    assert!(result.is_err());
     if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Unexpected '::' without preceding argument name"), "Error message mismatch: {}", e);
        assert_eq!(e.location, Some(SourceLocation::StrSpan{start:4, end:6}));
    }
}

#[test]
fn unexpected_operator_in_args() {
    let parser = Parser::new(default_options());
    let input = "cmd arg1 ?";
    let result = parser.parse_single_str(input);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Unexpected help operator '?' amidst arguments."), "Error message mismatch: {}", e);
         assert_eq!(e.location, Some(SourceLocation::StrSpan{start:9, end:10}));
    }
}

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
        assert_eq!(e.location, Some(SourceLocation::StrSpan{start:15, end:19})); // Corrected: location of the *second* "name"
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

/* // This test requires multi-segment path logic, deferred to Increment 5.1
#[test]
fn command_with_path_and_args_complex_fully_parsed() {
    let parser = Parser::new(options_allow_positional_after_named());
    let input = "path sub name::val pos1";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["path".to_string()]);

    assert_eq!(instruction.positional_arguments.len(), 2);
    assert_eq!(instruction.positional_arguments[0].value, "sub".to_string());
    assert_eq!(instruction.positional_arguments[1].value, "pos1".to_string());

    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, "val".to_string());
}
*/

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

#[test]
fn positional_arg_with_quoted_escaped_value_location() {
    let parser = Parser::new(default_options());
    let input = "cmd \"value with \\\"quotes\\\" and \\\\slash\\\\\"";
    let result = parser.parse_single_str(input);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    let arg = &instruction.positional_arguments[0];
    assert_eq!(arg.value, "value with \"quotes\" and \\slash\\".to_string());
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
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 2);
    assert_eq!(instruction.positional_arguments[0].value, "name".to_string());
    assert_eq!(instruction.positional_arguments[1].value, "value".to_string());
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
        assert_eq!(e.location, Some(SourceLocation::StrSpan{start:10, end:11})); // Corrected expected location
    }
}

#[test]
fn help_operator_after_args_is_error() {
    let parser = Parser::new(default_options());
    let input = "cmd arg1 ?";
    let result = parser.parse_single_str(input);
    assert!(result.is_err());
     if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Unexpected help operator '?' amidst arguments."), "Error message mismatch: {}", e);
    }

    let input2 = "cmd name::val ?";
    let result2 = parser.parse_single_str(input2);
    assert!(result2.is_err());
    if let Err(e) = result2 {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
         assert!(e.to_string().contains("Unexpected help operator '?' amidst arguments."), "Error message mismatch: {}", e);
    }
}