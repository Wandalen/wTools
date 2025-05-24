//! Tests for parser entry points and initial configuration.
use unilang_instruction_parser::*;
use unilang_instruction_parser::error::ErrorKind; // Added for error assertion
use unilang_instruction_parser::UnilangParserOptions;

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
    assert!(result.is_ok(), "Parse error for comment input: {:?}", result.err());
    assert!(result.unwrap().is_empty(), "Comment input should result in zero instructions");
}

#[test]
fn parse_single_str_simple_command_placeholder() {
    let options = UnilangParserOptions::default();
    let parser = Parser::new(options);
    let result = parser.parse_single_str("command");
    assert!(result.is_ok(), "Parse error for 'command': {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1, "Expected one instruction for 'command'");
    assert_eq!(instructions[0].command_path_slices, vec!["command".to_string()]);
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
    assert!(result.is_ok(), "Parse error for slice comment input: {:?}", result.err());
    assert!(result.unwrap().is_empty(), "Slice comment input should result in zero instructions");
}

#[test]
fn parse_slice_simple_command_placeholder() {
    let parser = Parser::new(default_options());
    let result = parser.parse_slice(&["cmd1", "cmd2"]);
    assert!(result.is_ok(), "Parse error for slice &[\"cmd1\", \"cmd2\"]: {:?}", result.err());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 2, "Expected two instructions for slice &[\"cmd1\", \"cmd2\"]");
    assert_eq!(instructions[0].command_path_slices, vec!["cmd1".to_string()]);
    assert_eq!(instructions[1].command_path_slices, vec!["cmd2".to_string()]);
}

// #[ignore] // Removed ignore
#[test]
fn parse_single_str_unterminated_quote_passes_to_analyzer() {
    let parser = Parser::new(default_options());
    let input = "command \"unterminated";
    let result = parser.parse_single_str(input);
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

// #[ignore] // Removed ignore
#[test]
fn parse_slice_unterminated_quote_passes_to_analyzer() {
    let parser = Parser::new(default_options());
    let input = &["command", "\"unterminated", "another"];
    let result = parser.parse_slice(input);
    assert!(result.is_err(), "Expected error for unterminated quote in slice, got Ok: {:?}", result.ok());
     if let Err(e) = result {
        assert!(matches!(e.kind, ErrorKind::Syntax(_)), "Expected Syntax error for slice, got {:?}", e.kind);
        // Check that the error location points to the problematic segment
        if let Some(SourceLocation::SliceSegment{ segment_index, .. }) = e.location {
            assert_eq!(segment_index, 1, "Error should be in segment 1");
        } else {
            panic!("Error location for slice should be SliceSegment, got {:?}", e.location);
        }
    }
}