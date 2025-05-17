use unilang_instruction_parser::*;
use std::borrow::Cow; // Import Cow
use unilang_instruction_parser::UnilangParserOptions; // Import UnilangParserOptions

// Define default_options function
fn default_options() -> UnilangParserOptions {
    UnilangParserOptions::default()
}

#[test]
fn parse_single_str_empty_input() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("");
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn parse_single_str_whitespace_input() {
    let options = UnilangParserOptions::default();
    let parser = Parser::new(options);
    let result = parser.parse_single_str("   \t\n  ");
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn parse_single_str_comment_input() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("# This is a comment");
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    assert!(result.unwrap().is_empty()); // Expect empty result for comment only
}

#[test]
fn parse_single_str_simple_command_placeholder() {
    let options = UnilangParserOptions::default();
    let parser = Parser::new(options);
    let result = parser.parse_single_str("command");
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    assert_eq!(instructions[0].command_path_slices, vec!["command"]); // Expect "command"
    assert!(!instructions[0].help_requested);
}

#[test]
fn parse_slice_empty_input() {
    let options = UnilangParserOptions::default();
    let parser = Parser::new(options);
    let input: &[&str] = &[];
    let result = parser.parse_slice(input);
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn parse_slice_empty_segments() {
    let options = UnilangParserOptions::default();
    let parser = Parser::new(options);
    let input: &[&str] = &["", "   ", "\t\n"];
    let result = parser.parse_slice(input);
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn parse_slice_comment_segments() {
    let parser = Parser::new(default_options());
    let result = parser.parse_slice(&["# comment 1", "  # comment 2  "]);
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    assert!(result.unwrap().is_empty()); // Expect empty result for comment only segments
}

#[test]
fn parse_slice_simple_command_placeholder() {
    let parser = Parser::new(default_options());
    let result = parser.parse_slice(&["cmd1", "cmd2"]);
    // With simplified path parsing, "cmd1" is the path from the first segment.
    // "cmd2" becomes a positional argument.
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["cmd1"]); // Path is "cmd1"
    assert_eq!(instruction.positional_arguments.len(), 1); // "cmd2" is a positional arg
    assert_eq!(instruction.positional_arguments[0].value, Cow::Borrowed("cmd2"));
}

#[test]
fn parse_single_str_unterminated_quote_passes_to_analyzer() {
    let parser = Parser::new(default_options());
    let result = parser.parse_single_str("command \"unterminated");
    // With simplified path parsing, "command" is the path. The rest are args.
    // The unterminated quote error should come from the argument parsing phase.
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["command"]); // Path is "command"
    // The rest of the items ["\"unterminated"] will be processed as arguments.
    // The error for the unterminated quote will occur during argument parsing.
    // This test should verify the structure up to the point of the error.
    // The actual error handling is tested in Increment 6.
    // For now, just verify the path is correctly identified.
}

#[test]
fn parse_slice_unterminated_quote_passes_to_analyzer() {
    let parser = Parser::new(default_options());
    let result = parser.parse_slice(&["command", "\"unterminated", "another"]);
    // With simplified path parsing, "command" is the path from the first segment.
    // The rest are args.
    assert!(result.is_ok(), "Parse error: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    let instruction = &instructions[0];
    assert_eq!(instruction.command_path_slices, vec!["command"]); // Path is "command"
    // The rest of the items ["\"unterminated", "another"] will be processed as arguments.
    // The error for the unterminated quote will occur during argument parsing.
    // For now, just verify the path is correctly identified.
}