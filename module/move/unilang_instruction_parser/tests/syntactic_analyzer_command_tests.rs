use unilang_instruction_parser::*; // Assuming lib.rs re-exports necessary types
use std::borrow::Cow; // Import Cow

fn default_options() -> UnilangParserOptions {
    UnilangParserOptions::default()
}

#[test]
fn single_command_path() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd");
    assert!(result.is_ok(), "parse_single_str failed: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    assert_eq!(instructions[0].command_path_slices, vec!["cmd"]);
    assert!(!instructions[0].help_requested);
    assert!(matches!(instructions[0].overall_location, SourceLocation::StrSpan { .. } | SourceLocation::SliceSegment { .. }));
}

#[test]
fn multi_segment_command_path() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd subcmd another");
    assert!(result.is_ok(), "parse_single_str failed: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    // With simplified path parsing, only the first delimited item is the path.
    assert_eq!(instructions[0].command_path_slices, vec!["cmd"]);
    // The subsequent items become positional arguments.
    assert_eq!(instructions[0].positional_arguments.len(), 2);
    assert_eq!(instructions[0].positional_arguments[0].value, Cow::Borrowed("subcmd"));
    assert_eq!(instructions[0].positional_arguments[1].value, Cow::Borrowed("another"));
    assert!(!instructions[0].help_requested);
}

#[test]
fn command_with_help_operator() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd ?");
    assert!(result.is_ok(), "parse_single_str failed: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    assert_eq!(instructions[0].command_path_slices, vec!["cmd"]);
    assert!(instructions[0].help_requested);
}

#[test]
fn command_with_help_operator_and_path() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd sub ?");
    assert!(result.is_ok(), "parse_single_str failed: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    // With simplified path parsing, only the first delimited item is the path.
    assert_eq!(instructions[0].command_path_slices, vec!["cmd"]);
    // "sub" becomes a positional argument.
    assert_eq!(instructions[0].positional_arguments.len(), 1);
    assert_eq!(instructions[0].positional_arguments[0].value, Cow::Borrowed("sub"));
    assert!(instructions[0].help_requested);
}

#[test]
fn multiple_commands_separated_by_semicolon() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd1 ;; cmd2 sub ? ;; cmd3");
    assert!(result.is_ok(), "parse_single_str failed: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 3);

    // Instruction 1: "cmd1"
    assert_eq!(instructions[0].command_path_slices, vec!["cmd1"]);
    assert!(instructions[0].positional_arguments.is_empty());
    assert!(instructions[0].named_arguments.is_empty());
    assert!(!instructions[0].help_requested);

    // Instruction 2: "cmd2 sub ?"
    // Path is "cmd2", "sub" is positional arg, help requested
    assert_eq!(instructions[1].command_path_slices, vec!["cmd2"]);
    assert_eq!(instructions[1].positional_arguments.len(), 1);
    assert_eq!(instructions[1].positional_arguments[0].value, Cow::Borrowed("sub"));
    assert!(instructions[1].named_arguments.is_empty());
    assert!(instructions[1].help_requested);

    // Instruction 3: "cmd3"
    assert_eq!(instructions[2].command_path_slices, vec!["cmd3"]);
    assert!(instructions[2].positional_arguments.is_empty());
    assert!(instructions[2].named_arguments.is_empty());
    assert!(!instructions[2].help_requested);
}

#[test]
fn multiple_commands_slice_input() {
    let parser = Parser::new(default_options());
    let input: &[&str] = &["cmd1", ";;", "cmd2 sub ?", ";;", "cmd3"];
    let result = parser.parse_slice(input);
    assert!(result.is_ok(), "parse_slice failed: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 3);

    // Instruction 1: "cmd1"
    assert_eq!(instructions[0].command_path_slices, vec!["cmd1"]);
    assert!(instructions[0].positional_arguments.is_empty());
    assert!(instructions[0].named_arguments.is_empty());
    assert!(!instructions[0].help_requested);
    assert!(matches!(instructions[0].overall_location, SourceLocation::SliceSegment { segment_index: 0, .. }));

    // Instruction 2: "cmd2 sub ?"
    // Path is "cmd2", "sub" is positional arg, help requested
    assert_eq!(instructions[1].command_path_slices, vec!["cmd2"]);
    assert_eq!(instructions[1].positional_arguments.len(), 1);
    assert_eq!(instructions[1].positional_arguments[0].value, Cow::Borrowed("sub"));
    assert!(instructions[1].named_arguments.is_empty());
    assert!(instructions[1].help_requested);
    assert!(matches!(instructions[1].overall_location, SourceLocation::SliceSegment { segment_index: 2, .. })); // ";;" is item at index 1

    // Instruction 3: "cmd3"
    assert_eq!(instructions[2].command_path_slices, vec!["cmd3"]);
    assert!(instructions[2].positional_arguments.is_empty());
    assert!(instructions[2].named_arguments.is_empty());
    assert!(!instructions[2].help_requested);
    assert!(matches!(instructions[2].overall_location, SourceLocation::SliceSegment { segment_index: 4, .. })); // ";;" is item at index 3
}

#[test]
fn leading_semicolon_is_empty_instruction_group() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str(";; cmd1");
    assert!(result.is_ok(), "parse_single_str failed: {:?}", result.err());
    let instructions = result.unwrap();
    // The first group before "cmd1" is empty due to leading ";;", so it's skipped.
    assert_eq!(instructions.len(), 1);
    assert_eq!(instructions[0].command_path_slices, vec!["cmd1"]);
}

#[test]
fn trailing_semicolon_is_ok() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd1 ;;");
    assert!(result.is_ok(), "parse_single_str failed: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1); // The empty group after "cmd1" is skipped.
    assert_eq!(instructions[0].command_path_slices, vec!["cmd1"]);
}

#[test]
fn multiple_consecutive_semicolons() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd1 ;;;; cmd2"); // Equivalent to cmd1 ;; cmd2 with empty groups
    assert!(result.is_ok(), "parse_single_str failed: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 2); // Empty groups between ";;" are skipped
    assert_eq!(instructions[0].command_path_slices, vec!["cmd1"]);
    assert_eq!(instructions[1].command_path_slices, vec!["cmd2"]);
}

#[test]
fn only_help_operator_no_command() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("?");
    assert!(result.is_ok());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    assert!(instructions[0].command_path_slices.is_empty());
    assert!(instructions[0].help_requested);
}

#[test]
fn command_path_ends_at_non_delimeted_item() {
    let parser = Parser::new(default_options());
    // With simplified path parsing, "cmd" is the path. "::" is an unexpected delimiter in arguments.
    let result = parser.parse_single_str("cmd :: arg1");
    assert!(result.is_err(), "parse_single_str unexpectedly succeeded: {:?}", result.ok());
    let err = result.unwrap_err();
    assert!(matches!(err.kind, ErrorKind::Syntax(_)));
    assert!(err.to_string().contains("Unexpected delimiter '::' in arguments section"));
    // Location assertion will be added in Increment 6
}