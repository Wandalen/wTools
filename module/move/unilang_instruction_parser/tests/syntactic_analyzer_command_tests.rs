//! Tests for syntactic analysis, focusing on command grouping and boundaries.
use unilang_instruction_parser::*;
// use std::borrow::Cow; // Removed unused import
use unilang_instruction_parser::error::ErrorKind; // For error assertion

fn default_options() -> UnilangParserOptions {
    UnilangParserOptions::default()
}

// Helper to check for a dummy instruction from the stub
// `parse_single_instruction_from_rich_items`.
// The stub creates a path with the first item's string if it's Identifier/UnquotedValue.
fn assert_is_dummy_instruction_from_first_item_if_any<'a>( instruction: &GenericInstruction<'a>, first_item_str_opt: Option<&'a str> )
{
    if let Some(expected_path_slice) = first_item_str_opt {
        assert_eq!(instruction.command_path_slices, vec![expected_path_slice.to_string()]);
    } else {
        // If no items or first item not suitable, stub might use a default dummy path
        assert_eq!(instruction.command_path_slices, vec!["dummy_cmd_path_inc3".to_string()]);
    }
    assert!(instruction.named_arguments.is_empty());
    assert!(instruction.positional_arguments.is_empty());
    assert!(!instruction.help_requested);
}


#[test]
fn single_command_no_semicolon() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd");
    assert!(result.is_ok(), "parse_single_str failed: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1, "Expected 1 instruction for 'cmd'");
    assert_is_dummy_instruction_from_first_item_if_any(&instructions[0], Some("cmd"));
}

#[test]
fn multiple_commands_separated_by_semicolon_dummy_check() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("cmd1 ;; cmd2 ;; cmd3");
    assert!(result.is_ok(), "parse_single_str failed: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 3);

    assert_is_dummy_instruction_from_first_item_if_any(&instructions[0], Some("cmd1"));
    assert_is_dummy_instruction_from_first_item_if_any(&instructions[1], Some("cmd2"));
    assert_is_dummy_instruction_from_first_item_if_any(&instructions[2], Some("cmd3"));
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
        assert!(matches!(e.kind, ErrorKind::Syntax(_)));
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
fn single_command_slice_input_dummy_check() {
    let parser = Parser::new(default_options());
    let result = parser.parse_slice(&["cmd", "arg"]);
    assert!(result.is_ok(), "parse_slice failed: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    assert_is_dummy_instruction_from_first_item_if_any(&instructions[0], Some("cmd"));
}

#[test]
fn multiple_commands_slice_input_dummy_check() {
    let parser = Parser::new(default_options());
    let input: &[&str] = &["cmd1", ";;", "cmd2", ";;", "cmd3"];
    let result = parser.parse_slice(input);
    assert!(result.is_ok(), "parse_slice failed: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 3);
    assert_is_dummy_instruction_from_first_item_if_any(&instructions[0], Some("cmd1"));
    assert_is_dummy_instruction_from_first_item_if_any(&instructions[1], Some("cmd2"));
    assert_is_dummy_instruction_from_first_item_if_any(&instructions[2], Some("cmd3"));
}


// TODO: The following tests are for future increments (Path, Help, Args) and are commented out for now.
// They need to be re-evaluated when parse_single_instruction_from_rich_items is implemented.

// #[test]
// fn multi_segment_command_path() {
//     let parser = Parser::new(default_options());
//     let result = parser.parse_single_str("cmd subcmd another");
//     assert!(result.is_ok(), "parse_single_str failed: {:?}", result.err());
//     let instructions = result.unwrap();
//     assert_eq!(instructions.len(), 1);
//     assert_eq!(instructions[0].command_path_slices, vec!["cmd".to_string(), "subcmd".to_string(), "another".to_string()]);
//     assert!(!instructions[0].help_requested);
// }
//
// #[test]
// fn command_with_help_operator() {
//     let parser = Parser::new(default_options());
//     let result = parser.parse_single_str("cmd ?");
//     assert!(result.is_ok(), "parse_single_str failed: {:?}", result.err());
//     let instructions = result.unwrap();
//     assert_eq!(instructions.len(), 1);
//     assert_eq!(instructions[0].command_path_slices, vec!["cmd".to_string()]);
//     assert!(instructions[0].help_requested);
// }
//
// #[test]
// fn command_with_help_operator_and_path() {
//     let parser = Parser::new(default_options());
//     let result = parser.parse_single_str("cmd sub ?");
//     assert!(result.is_ok(), "parse_single_str failed: {:?}", result.err());
//     let instructions = result.unwrap();
//     assert_eq!(instructions.len(), 1);
//     assert_eq!(instructions[0].command_path_slices, vec!["cmd".to_string(), "sub".to_string()]);
//     assert!(instructions[0].help_requested);
// }

// #[test]
// fn command_path_ends_at_non_delimeted_item() {
//     let parser = Parser::new(default_options());
//     let result = parser.parse_single_str("cmd :: arg1");
//     assert!(result.is_err(), "parse_single_str unexpectedly succeeded: {:?}", result.ok());
//     let err = result.unwrap_err();
//     assert!(matches!(err.kind, ErrorKind::Syntax(_)));
// }