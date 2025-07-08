//! Tests for parser entry points and initial configuration.
use unilang_instruction_parser::*;
use unilang_instruction_parser::error::ErrorKind; // Added for error assertion
use unilang_instruction_parser::UnilangParserOptions;

// Define default_options function


#[test]
fn parse_single_str_empty_input() {
    let parser = Parser::new();
    let result = parser.parse_single_instruction("");
    assert!(result.is_ok());
    assert!(result.unwrap().command_path_slices.is_empty()); // Changed from is_empty() on Vec
}

#[test]
fn parse_single_str_whitespace_input() {
    let options = UnilangParserOptions::default();
    let parser = Parser::new_with_options(options);
    let result = parser.parse_single_instruction("   \t\n  ");
    assert!(result.is_ok());
    assert!(result.unwrap().command_path_slices.is_empty()); // Changed from is_empty() on Vec
}

#[test]
fn parse_single_str_comment_input() {
    let parser = Parser::new();
    let result = parser.parse_single_instruction("# This is a comment");
    assert!(result.is_ok(), "Parse error for comment input: {:?}", result.err());
    assert!(result.unwrap().command_path_slices.is_empty(), "Comment input should result in zero instructions"); // Changed from is_empty() on Vec
}

#[test]
fn parse_single_str_simple_command_placeholder() {
    let options = UnilangParserOptions::default();
    let parser = Parser::new_with_options(options);
    let result = parser.parse_single_instruction("command");
    assert!(result.is_ok(), "Parse error for 'command': {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["command".to_string()]);
}

// #[ignore] // Removed ignore
#[test]
fn parse_single_str_unterminated_quote_passes_to_analyzer() {
    let parser = Parser::new();
    let input = "command \"unterminated";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Expected error for unterminated quote, got Ok: {:?}", result.ok());
    if let Err(e) = result {
        // Depending on how strs_tools passes this, it might be an "Unrecognized" token
        // or a specific error if unilang_parser adds further validation for quote pairing
        // based on classified tokens. For now, a general Syntax error is acceptable.
        assert!(matches!(e.kind, ErrorKind::Syntax(_)), "Expected Syntax error, got {:?}", e.kind);
        // A more specific check could be:
        // assert!(e.to_string().to_lowercase().contains("unterminated quote") || e.to_string().contains("Unexpected token"));
    }
}