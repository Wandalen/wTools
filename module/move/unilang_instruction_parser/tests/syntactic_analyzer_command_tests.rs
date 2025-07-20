//! Tests for syntactic analysis, focusing on command grouping and boundaries.
use unilang_instruction_parser::*;
use unilang_instruction_parser::error::ErrorKind; // For error assertion



#[test]
fn single_command_path_parsed() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_single_instruction("cmd");
    assert!(result.is_ok(), "parse_single_instruction failed: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.named_arguments.is_empty());
    assert!(instruction.positional_arguments.is_empty());
}

#[test]
fn multi_segment_command_path_parsed() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd subcmd another";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "parse_single_instruction failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 2);
    assert_eq!(instruction.positional_arguments[0].value, "subcmd".to_string());
    assert_eq!(instruction.positional_arguments[1].value, "another".to_string());
}

#[test]
fn command_with_help_operator_parsed() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_single_instruction("cmd ?");
    assert!(result.is_ok(), "parse_single_instruction failed: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert!(instruction.positional_arguments.is_empty()); // Corrected: '?' is not a positional arg
    assert!(instruction.named_arguments.is_empty());
    assert!(instruction.help_requested); // Corrected: '?' sets help_requested flag
}

#[test]
fn command_with_help_operator_and_multi_segment_path() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd sub ?";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "parse_single_instruction failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1); // Corrected: 'sub' is positional, '?' is not
    assert_eq!(instruction.positional_arguments[0].value, "sub".to_string());
    assert!(instruction.named_arguments.is_empty());
    assert!(instruction.help_requested); // Corrected: '?' sets help_requested flag
}

#[test]
fn only_help_operator() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_single_instruction("?");
    assert!(result.is_ok(), "parse_single_instruction failed for '?': {:?}", result.err());
    let instruction = result.unwrap();
    assert!(instruction.command_path_slices.is_empty());
    assert!(instruction.positional_arguments.is_empty()); // Corrected: '?' is not a positional arg
    assert!(instruction.named_arguments.is_empty());
    assert!(instruction.help_requested); // Corrected: '?' sets help_requested flag
}


#[test]
fn multiple_commands_separated_by_semicolon_path_and_help_check() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "cmd1 ;; cmd2 sub ? ;; cmd3";
    let result = parser.parse_multiple_instructions(input);
    assert!(result.is_ok(), "parse_multiple_instructions failed for input '{}': {:?}", input, result.err());
    let instructions = result.unwrap(); // This will still be a Vec<GenericInstruction> for parse_multiple_instructions
    assert_eq!(instructions.len(), 3);

    assert_eq!(instructions[0].command_path_slices, vec!["cmd1".to_string()]);

    assert_eq!(instructions[1].command_path_slices, vec!["cmd2".to_string()]);
    assert_eq!(instructions[1].positional_arguments.len(), 1); // Corrected: 'sub' is positional, '?' is not
    assert_eq!(instructions[1].positional_arguments[0].value, "sub".to_string());
    assert!(instructions[1].help_requested); // Corrected: '?' sets help_requested flag

    assert_eq!(instructions[2].command_path_slices, vec!["cmd3".to_string()]);
}

#[test]
fn leading_semicolon_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_multiple_instructions(";; cmd1"); // Changed to parse_multiple_instructions
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
    let result = parser.parse_multiple_instructions(input); // Changed to parse_multiple_instructions
    assert!(result.is_err(), "Expected error for trailing ';;' if empty segments are errors");
     if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::TrailingDelimiter)); // Updated to expect TrailingDelimiter
        assert!(e.to_string().contains("Trailing delimiter")); // Updated error message
    }
}

#[test]
fn multiple_consecutive_semicolons_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_multiple_instructions("cmd1 ;;;; cmd2"); // Changed to parse_multiple_instructions
    assert!(result.is_err(), "Expected error for 'cmd1 ;;;; cmd2'");
    if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::EmptyInstructionSegment));
        assert!(e.to_string().contains("Empty instruction segment"));
    }
}

#[test]
fn only_semicolons_error() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_multiple_instructions(";;"); // Changed to parse_multiple_instructions
    assert!(result.is_err(), "Expected error for ';;'");
     if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::EmptyInstructionSegment));
        assert!(e.to_string().contains("Empty instruction segment"));
    }
    let result_double = parser.parse_multiple_instructions(";;;;"); // Changed to parse_multiple_instructions
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
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Parse failed for input '{}': {:?}", input, result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "path".to_string());
    assert_eq!(instruction.named_arguments.len(), 1);
    assert!(instruction.named_arguments.contains_key("arg"));
    assert_eq!(instruction.named_arguments.get("arg").unwrap().value, "val");
}