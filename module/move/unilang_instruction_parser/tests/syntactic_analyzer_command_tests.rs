//! Tests for syntactic analysis, focusing on command grouping and boundaries.
use unilang_instruction_parser::*;
use unilang_instruction_parser::error::ErrorKind; // For error assertion



#[test]
fn single_command_path_parsed() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_single_instruction("cmd");
    assert!(result.is_ok(), "parse_single_instruction failed: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path, vec!["cmd".to_string()]);
    assert!(instruction.named_arguments.is_empty());
    assert!(instruction.arguments.is_empty());
    // assert!(!instruction.help_requested); // Removed
}

#[test]
fn multi_segment_command_path_parsed() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd subcmd another";
    let result = parser.parse_single_instruction(input); // Changed to parse_single_instruction
    assert!(result.is_ok(), "parse_single_instruction failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path, vec!["cmd".to_string(), "subcmd".to_string(), "another".to_string()]);
    assert!(instruction.arguments.is_empty());
    // assert!(!instruction.help_requested); // Removed
}

#[test]
fn command_with_help_operator_parsed() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_single_instruction("cmd ?");
    assert!(result.is_ok(), "parse_single_instruction failed: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path, vec!["cmd".to_string()]);
    // assert!(instruction.help_requested); // Removed
    assert_eq!(instruction.arguments, vec!["?".to_string()]); // ? is now an argument
    assert!(instruction.named_arguments.is_empty());
}

#[test]
fn command_with_help_operator_and_multi_segment_path() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd sub ?";
    let result = parser.parse_single_instruction(input); // Changed to parse_single_instruction
    assert!(result.is_ok(), "parse_single_instruction failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path, vec!["cmd".to_string(), "sub".to_string()]);
    // assert!(instruction.help_requested); // Removed
    assert_eq!(instruction.arguments, vec!["?".to_string()]); // ? is now an argument
    assert!(instruction.named_arguments.is_empty());
}

#[test]
fn only_help_operator() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_single_instruction("?");
    assert!(result.is_ok(), "parse_single_instruction failed for '?': {:?}", result.err());
    let instruction = result.unwrap();
    assert!(instruction.command_path.is_empty());
    // assert!(instruction.help_requested); // Removed
    assert_eq!(instruction.arguments, vec!["?".to_string()]); // ? is now an argument
    assert!(instruction.named_arguments.is_empty());
}


#[test]
fn multiple_commands_separated_by_semicolon_path_and_help_check() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd1 ;; cmd2 sub ? ;; cmd3";
    let result = parser.parse_multiple_instructions(input);
    assert!(result.is_ok(), "parse_multiple_instructions failed for input '{}': {:?}", input, result.err());
    let instructions = result.unwrap(); // This will still be a Vec<GenericInstruction> for parse_multiple_instructions
    assert_eq!(instructions.len(), 3);

    assert_eq!(instructions[0].command_path, vec!["cmd1".to_string()]);
    // assert!(!instructions[0].help_requested); // Removed

    assert_eq!(instructions[1].command_path, vec!["cmd2".to_string(), "sub".to_string()]);
    // assert!(instructions[1].help_requested); // Removed
    assert_eq!(instructions[1].arguments, vec!["?".to_string()]); // ? is now an argument

    assert_eq!(instructions[2].command_path, vec!["cmd3".to_string()]);
    // assert!(!instructions[2].help_requested); // Removed
}

#[test]
fn leading_semicolon_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_single_instruction(";; cmd1");
    assert!(result.is_err(), "Expected error for leading ';;'");
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::EmptyInstructionSegment));
        assert!(e.to_string().contains("Empty instruction segment"));
    }
}

#[test]
fn trailing_semicolon_error_if_empty_segment_is_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd1 ;;";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for trailing ';;' if empty segments are errors");
     if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::TrailingDelimiter)); // Updated to expect TrailingDelimiter
        assert!(e.to_string().contains("Empty instruction segment"));
    }
}

#[test]
fn multiple_consecutive_semicolons_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_single_instruction("cmd1 ;;;; cmd2");
    assert!(result.is_err(), "Expected error for 'cmd1 ;;;; cmd2'");
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::EmptyInstructionSegment));
        assert!(e.to_string().contains("Empty instruction segment"));
    }
}

#[test]
fn only_semicolons_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_single_instruction(";;");
    assert!(result.is_err(), "Expected error for ';;'");
     if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::EmptyInstructionSegment));
        assert!(e.to_string().contains("Empty instruction segment"));
    }
    let result_double = parser.parse_single_instruction(";;;;");
    assert!(result_double.is_err(), "Expected error for ';;;;'");
    if let Err(e) = result_double {
        assert!(matches!(e.kind, ErrorKind::EmptyInstructionSegment));
        assert!(e.to_string().contains("Empty instruction segment"));
    }
}

// Removed parse_slice tests: single_command_slice_input_path_check and multiple_commands_slice_input_path_check

#[test]
fn path_stops_at_double_colon_delimiter() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd path arg::val";
    let result = parser.parse_single_instruction(input); // Changed to parse_single_instruction
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path, vec!["cmd".to_string(), "path".to_string()]);
    assert_eq!(instruction.named_arguments.len(), 1);
    assert!(instruction.named_arguments.contains_key("arg"));
    assert_eq!(instruction.named_arguments.get("arg").unwrap(), "val");
    assert!(instruction.arguments.is_empty());
}