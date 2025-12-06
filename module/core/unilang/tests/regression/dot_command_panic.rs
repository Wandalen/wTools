//!
//! Tests for dot command behavior to prevent regression of panic issue.
//! 
//! This test specifically covers the issue where entering just "." would cause
//! a panic due to an empty `command_path_slices` vector.
//!

#![ allow( deprecated ) ]

use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang::error::Error;
use unilang_parser::{ Parser, UnilangParserOptions };

#[test]
fn test_dot_command_shows_help_instead_of_panicking()
{
  // This test specifically covers the bug where "." caused a panic
  // Now it should return a help listing instead
  
    let mut registry = CommandRegistry::new();
  
  // Add a test command
  let test_command = unilang::data::CommandDefinition::former()
    .name(".test")
    .namespace("")
    .description("A test command")
    .end();

  registry.register(test_command).expect("Failed to register test command");
  
  // Parse a single dot - this used to cause panic
  let program = ".";
  let parser = Parser::new(UnilangParserOptions::default());
  let instruction = parser.parse_single_instruction(program)
    .expect("Should parse single dot without error");
  let instructions = &[instruction];
  
  let analyzer = SemanticAnalyzer::new(instructions, &registry);
  let result = analyzer.analyze();
  
  // Should return an error with help content, not panic
  assert!(result.is_err(), "Dot command should return help error, not success");

  if let Err(Error::Execution(error_data)) = result {
    assert_eq!(error_data.code, unilang::data::ErrorCode::HelpRequested, "Should return HELP_REQUESTED error code");
    assert!(error_data.message.contains("Available commands"), "Should contain help text");
    assert!(error_data.message.contains(".test"), "Should list the test command");
  } else {
    panic!("Expected Execution error with help content");
  }
}

#[test]
fn test_dot_command_with_minimal_commands()
{
  // Test dot command with only built-in commands (like .version)
    let registry = CommandRegistry::new();
  
  let program = ".";
  let parser = Parser::new(UnilangParserOptions::default());
  let instruction = parser.parse_single_instruction(program)
    .expect("Should parse single dot without error");
  let instructions = &[instruction];
  
  let analyzer = SemanticAnalyzer::new(instructions, &registry);
  let result = analyzer.analyze();
  
  // Should return help showing available commands (including mandatory global help)
  assert!(result.is_err(), "Dot command should return help error");

  if let Err(Error::Execution(error_data)) = result {
    assert_eq!(error_data.code, unilang::data::ErrorCode::HelpRequested);
    // NOTE: With mandatory help enforcement, .help command is always available
    assert!(error_data.message.contains("Available Commands") ||
            error_data.message.contains(".help"),
            "Should show available commands or mention .help command");
  } else {
    panic!("Expected Execution error with help content");
  }
}

#[test] 
fn test_dot_command_lists_multiple_commands()
{
    let mut registry = CommandRegistry::new();
  
  // Add multiple test commands
  let cmd1 = unilang::data::CommandDefinition::former()
    .name(".first")
    .namespace(".test")
    .description("First test command")
    .end();
    
  let cmd2 = unilang::data::CommandDefinition::former()
    .name(".second")
    .namespace(".test")
    .description("Second test command")
    .end();

  registry.register(cmd1).expect("Failed to register first command");
  registry.register(cmd2).expect("Failed to register second command");
  
  let program = ".";
  let parser = Parser::new(UnilangParserOptions::default());
  let instruction = parser.parse_single_instruction(program)
    .expect("Should parse single dot without error");
  let instructions = &[instruction];
  
  let analyzer = SemanticAnalyzer::new(instructions, &registry);
  let result = analyzer.analyze();
  
  if let Err(Error::Execution(error_data)) = result {
    assert_eq!(error_data.code, unilang::data::ErrorCode::HelpRequested);
    assert!(error_data.message.contains(".test.first"), "Should list first command");
    assert!(error_data.message.contains(".test.second"), "Should list second command");
    assert!(error_data.message.contains("First test command"), "Should show first description");
    assert!(error_data.message.contains("Second test command"), "Should show second description");
  } else {
    panic!("Expected help listing with multiple commands");
  }
}

#[test]
fn test_empty_command_path_edge_case()
{
  // This tests the specific edge case that was causing the panic:
  // When command_path_slices is empty, accessing index 0 panicked
  
    let registry = CommandRegistry::new();
  
  // Create a GenericInstruction with empty command_path_slices 
  // (this simulates what the parser produces for ".")
  let empty_instruction = unilang_parser::GenericInstruction {
    command_path_slices: vec![], // This was causing the panic
    named_arguments: std::collections::BTreeMap::new(),
    positional_arguments: vec![],
    help_requested: false,
    overall_location: unilang_parser::SourceLocation::StrSpan { start: 0, end: 1 },
  };
  
  let instructions = [empty_instruction];
  let analyzer = SemanticAnalyzer::new(&instructions, &registry);
  
  // This should not panic anymore
  let result = analyzer.analyze();
  
  // Should return help instead of panicking
  assert!(result.is_err());
  if let Err(Error::Execution(error_data)) = result {
    assert_eq!(error_data.code, unilang::data::ErrorCode::HelpRequested);
  }
}