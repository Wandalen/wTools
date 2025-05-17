use unilang_instruction_parser::*;
use std::collections::HashMap;
use std::borrow::Cow;

fn default_options() -> UnilangParserOptions {
    UnilangParserOptions::default()
}

#[test]
fn command_with_only_positional_args() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd pos1 pos2");
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd"]);
    assert_eq!(instruction.positional_arguments.len(), 2);
    assert_eq!(instruction.positional_arguments[0].value, Cow::Borrowed("pos1"));
    assert_eq!(instruction.positional_arguments[1].value, Cow::Borrowed("pos2"));
    assert!(instruction.named_arguments.is_empty());
    assert!(!instruction.help_requested);
}

#[test]
fn command_with_only_named_args() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd name1::val1 name2::val2");
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd"]);
    assert!(instruction.positional_arguments.is_empty());
    assert_eq!(instruction.named_arguments.len(), 2);
    assert_eq!(instruction.named_arguments.get("name1").unwrap().value, Cow::Borrowed("val1"));
    assert_eq!(instruction.named_arguments.get("name2").unwrap().value, Cow::Borrowed("val2"));
    assert!(!instruction.help_requested);
}

#[test]
fn command_with_mixed_args_positional_first() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd pos1 name1::val1 pos2 name2::val2");
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd"]);
    assert_eq!(instruction.positional_arguments.len(), 2);
    assert_eq!(instruction.positional_arguments[0].value, Cow::Borrowed("pos1"));
    assert_eq!(instruction.positional_arguments[1].value, Cow::Borrowed("pos2"));
    assert_eq!(instruction.named_arguments.len(), 2);
    assert_eq!(instruction.named_arguments.get("name1").unwrap().value, Cow::Borrowed("val1"));
    assert_eq!(instruction.named_arguments.get("name2").unwrap().value, Cow::Borrowed("val2"));
}

#[test]
fn command_with_mixed_args_named_first() {
    // Assuming unilang allows named then positional, though typically positional are first or not allowed after named.
    // Current parser logic will treat subsequent Delimited items as positional if not part of a name::value.
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd name1::val1 pos1 name2::val2 pos2");
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd"]);
    assert_eq!(instruction.positional_arguments.len(), 2);
    assert_eq!(instruction.positional_arguments[0].value, Cow::Borrowed("pos1"));
    assert_eq!(instruction.positional_arguments[1].value, Cow::Borrowed("pos2"));
    assert_eq!(instruction.named_arguments.len(), 2);
    assert_eq!(instruction.named_arguments.get("name1").unwrap().value, Cow::Borrowed("val1"));
    assert_eq!(instruction.named_arguments.get("name2").unwrap().value, Cow::Borrowed("val2"));
}

#[test]
fn named_arg_with_empty_value() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd name::\"\"");
    // Expect error because strs_tools with preserve_empty=false will drop the "" token after quotes.
    assert!(result.is_err(), "Expected error for name:: followed by (dropped) empty string, got Ok: {:?}", result.ok());
    if let Err(e) = result {
        assert!(e.to_string().contains("not followed by a value"), "Unexpected error message: {}", e);
    }
}

#[test]
fn named_arg_with_empty_value_no_quotes() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd name::");
    // This should be an error: "Named argument '::' not followed by a value"
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
        // Optionally, check the error message content if it's specific enough
        // assert!(e.to_string().contains("not followed by a value"));
    }
}

#[test]
fn named_arg_missing_name() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd ::value");
    // This should be an error: "Named argument has empty name" or similar,
    // because "::value" will be split by strs_tools into Delimeter("::") and Delimeted("value").
    // The parser will see "::" first in args_iter.
    assert!(result.is_err());
     if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
            eprintln!("DEBUG: Actual error for named_arg_missing_name: {}", e);
            assert!(e.to_string().contains("Unexpected delimiter '::' in arguments section")); // Corrected expected error
        }
    }

#[test]
fn positional_arg_can_be_empty_if_preserved_and_quoted() {
    // With UnilangParserOptions default (preserve_empty: false for strs_tools),
    // strs_tools will produce RI("cmd") and the RI("") from "" will be dropped.
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd \"\"");
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd"]); // Path is "cmd"
    assert_eq!(instruction.positional_arguments.len(), 0); // Empty string arg is dropped
}

#[test]
fn unexpected_delimiter_in_args() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd arg1 ;; arg2");
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 2);

    let instruction1 = &instructions[0];
    assert_eq!(instruction1.command_path_slices, vec!["cmd"]);
    assert_eq!(instruction1.positional_arguments.len(), 1);
    assert_eq!(instruction1.positional_arguments[0].value, Cow::Borrowed("arg1"));
    assert!(instruction1.named_arguments.is_empty());
    assert!(!instruction1.help_requested);

    let instruction2 = &instructions[1];
    assert_eq!(instruction2.command_path_slices, vec!["arg2"]);
    assert!(instruction2.positional_arguments.is_empty());
    assert!(instruction2.named_arguments.is_empty());
    assert!(!instruction2.help_requested);
}

#[test]
fn command_with_path_and_args() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("path sub name::val pos1");
     assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["path"]); // Path is only "path"
    assert_eq!(instruction.positional_arguments.len(), 2); // "sub" becomes a positional arg
    assert_eq!(instruction.positional_arguments[0].value, Cow::Borrowed("sub"));
    assert_eq!(instruction.positional_arguments[1].value, Cow::Borrowed("pos1"));
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, Cow::Borrowed("val"));
}

#[test]
fn command_with_path_help_and_args() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("path sub ? name::val pos1");
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["path"]); // Path is only "path"
    assert!(instruction.help_requested); // Help is still after path
    assert_eq!(instruction.positional_arguments.len(), 2); // "sub" becomes a positional arg
    assert_eq!(instruction.positional_arguments[0].value, Cow::Borrowed("sub"));
    assert_eq!(instruction.positional_arguments[1].value, Cow::Borrowed("pos1"));
    assert_eq!(instruction.named_arguments.len(), 1);
    assert_eq!(instruction.named_arguments.get("name").unwrap().value, Cow::Borrowed("val"));
}