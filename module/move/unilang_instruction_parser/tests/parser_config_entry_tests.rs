//! ## Test Matrix for Parser Entry Points and Configuration
//!
//! This matrix outlines test cases for the `Parser`'s entry points (`parse_single_instruction`)
//! and its initial configuration, focusing on various basic input types.
//!
//! **Test Factors:**
//! - `Input String`: Different forms of input (empty, whitespace, comment, simple command, unterminated quote).
//! - `Parser Options`: The configuration used for the parser (currently only `Default`).
//!
//! ---
//!
//! **Test Combinations:**
//!
//! | ID   | Aspect Tested        | Input String              | Parser Options | Expected Behavior                                     |
//! |------|----------------------|---------------------------|----------------|-------------------------------------------------------|
//! | T1.1 | Empty input          | `""`                      | Default        | `Ok`, empty instruction (no command, args, or help)   |
//! | T1.2 | Whitespace input     | `"   \t\n  "`             | Default        | `Ok`, empty instruction (no command, args, or help)   |
//! | T1.3 | Comment input        | `"# This is a comment"`   | Default        | `Err(Syntax("Unexpected token '#'" ))`                |
//! | T1.4 | Simple command       | `"command"`               | Default        | `Ok`, command path `["command"]`                      |
//! | T1.5 | Unterminated quote   | `"command \"unterminated"`| Default        | `Ok`, command path `["command"]`, positional arg `["unterminated"]` |

use unilang_instruction_parser::*;
use unilang_instruction_parser::error::ErrorKind; // Added for error assertion
use unilang_instruction_parser::UnilangParserOptions;

// Define default_options function


/// Tests parsing an empty input string.
/// Test Combination: T1.1
#[test]
fn parse_single_str_empty_input() {
    let parser = Parser::new(UnilangParserOptions::default());
    let result = parser.parse_single_instruction("");
    assert!(result.is_ok(), "Expected Ok for empty input, got Err: {:?}", result.err());
    let instruction = result.unwrap();
    assert!(instruction.command_path_slices.is_empty());
    assert!(instruction.positional_arguments.is_empty());
    assert!(instruction.named_arguments.is_empty());
    assert!(!instruction.help_requested);
}

/// Tests parsing an input string consisting only of whitespace.
/// Test Combination: T1.2
#[test]
fn parse_single_str_whitespace_input() {
    let options = UnilangParserOptions::default();
    let parser = Parser::new(options);
    let result = parser.parse_single_instruction("   \t\n  ");
    assert!(result.is_ok(), "Expected Ok for whitespace input, got Err: {:?}", result.err());
    let instruction = result.unwrap();
    assert!(instruction.command_path_slices.is_empty());
    assert!(instruction.positional_arguments.is_empty());
    assert!(instruction.named_arguments.is_empty());
    assert!(!instruction.help_requested);
}

/// Tests parsing an input string that starts with a comment character.
/// Test Combination: T1.3
#[test]
fn parse_single_str_comment_input() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "# This is a comment";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_err(), "Parse error for comment input: {:?}", result.err());
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::Syntax("Unexpected token '#' in arguments".to_string()));
    }
}

/// Tests parsing a simple command with no arguments or operators.
/// Test Combination: T1.4
#[test]
fn parse_single_str_simple_command_placeholder() {
    let options = UnilangParserOptions::default();
    let parser = Parser::new(options);
    let result = parser.parse_single_instruction("command");
    assert!(result.is_ok(), "Parse error for 'command': {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["command".to_string()]);
}

/// Tests parsing an input with an unterminated quoted string.
/// Test Combination: T1.5
#[test]
fn parse_single_str_unterminated_quote_passes_to_analyzer() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = "command \"unterminated";
    let result = parser.parse_single_instruction(input);
    assert!(result.is_ok(), "Expected Ok for unterminated quote, got Err: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["command".to_string()]);
    assert_eq!(instruction.positional_arguments.len(), 1);
    assert_eq!(instruction.positional_arguments[0].value, "unterminated".to_string());
}