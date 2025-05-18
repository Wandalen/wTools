//! Tests for argument parsing logic.
use unilang_instruction_parser::*;
use std::collections::HashMap;
use std::borrow::Cow;
use unilang_instruction_parser::error::ErrorKind;

fn default_options() -> UnilangParserOptions {
    UnilangParserOptions::default()
}

#[test]
fn command_with_only_positional_args_fully_parsed() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd pos1 pos2");
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 2);
    assert_eq!(instruction.positional_arguments[0].value, Cow::<'static, str>::Owned(String::from("pos1")));
    assert_eq!(instruction.positional_arguments[1].value, Cow::<'static, str>::Owned(String::from("pos2")));
    assert!(instruction.named_arguments.is_empty());
}

#[test]
fn command_with_only_named_args_fully_parsed() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd name1::val1 name2::val2");
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.positional_arguments.is_empty());
    assert_eq!(instruction.named_arguments.len(), 2);
    assert_eq!(instruction.named_arguments.get("name1").unwrap().value, Cow::<'static, str>::Owned("val1".to_string()));
    assert_eq!(instruction.named_arguments.get("name2").unwrap().value, Cow::<'static, str>::Owned("val2".to_string()));
}

#[test]
fn command_with_mixed_args_positional_first_fully_parsed() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd pos1 name1::val1 pos2 name2::val2");
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);

    assert_eq!(instruction.positional_arguments.len(), 2);
    assert_eq!(instruction.positional_arguments[0].value, Cow::<'static, str>::Owned("pos1".to_string()));
    assert_eq!(instruction.positional_arguments[1].value, Cow::<'static, str>::Owned("pos2".to_string()));

    assert_eq!(instruction.named_arguments.len(), 2);
    assert_eq!(instruction.named_arguments.get("name1").unwrap().value, Cow::<'static, str>::Owned("val1".to_string()));
    assert_eq!(instruction.named_arguments.get("name2").unwrap().value, Cow::<'static, str>::Owned("val2".to_string()));
}

#[test]
fn named_arg_with_empty_value_no_quotes_error() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd name::");
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Expected value for named argument but found end of instruction"));
    }
}

#[test]
fn named_arg_missing_name_error() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd ::value");
    assert!(result.is_err());
     if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Unexpected '::' without preceding argument name"));
    }
}

#[test]
fn unexpected_operator_in_args() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd arg1 ?");
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Unexpected token in arguments: '?'"));
    }
}

#[test]
fn unescaping_placeholder_test_named() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd name::\"a\\\\b\\\"c\\\'d\\ne\\tf\"");
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, Cow::<'static, str>::Owned("a\\b\"c\'d\ne\tf".to_string()));
    assert!(instruction.positional_arguments.is_empty());
}

#[test]
fn duplicate_named_arg_error() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd name::val1 name::val2");
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        assert!(e.to_string().contains("Duplicate named argument: name"));
    }
}

#[test]
fn command_with_path_and_args_complex_fully_parsed() {
    let parser = Parser::new(default_options());
    // Path parser takes "path" then "sub". Arg parser takes "name::val" and "pos1".
    let result = parser.parse_single_str("path sub name::val pos1");
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["path".to_string(), "sub".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, Cow::<'static, str>::Owned("pos1".to_string()));
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, Cow::<'static, str>::Owned("val".to_string()));
}

#[test]
fn named_arg_with_quoted_escaped_value() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd key::\"value with \\\"quotes\\\" and \\\\slash\\\\\"");
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.positional_arguments.is_empty());
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(
        instruction.named_arguments.get("key").unwrap().value,
        Cow::<'static, str>::Owned("value with \"quotes\" and \\slash\\".to_string())
    );
}

#[test]
fn positional_arg_with_quoted_escaped_value() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd \"value with \\\"quotes\\\" and \\\\slash\\\\\"");
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(
        instruction.positional_arguments[0].value,
        Cow::<'static, str>::Owned("value with \"quotes\" and \\slash\\".to_string())
    );
    assert!(instruction.named_arguments.is_empty());
}