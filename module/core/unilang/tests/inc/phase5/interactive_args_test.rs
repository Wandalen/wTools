//!
//! Tests for interactive argument signaling (M5.2, M5.3)
//!
//! This test verifies that the `SemanticAnalyzer` correctly returns
//! `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` for missing interactive arguments.
//!

use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes };
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang_parser::{ GenericInstruction, SourceLocation };

#[test]
#[allow(clippy::too_many_lines)]
fn test_interactive_argument_signaling()
{
  // Create a command with an interactive argument
  #[allow(deprecated)]
  #[allow(deprecated)]
    let mut registry = CommandRegistry::new();
  
  let command_def = CommandDefinition
  {
    name: "config.set".to_string(),
    description: "Set a configuration value".to_string(),
    arguments: vec!
    [
      ArgumentDefinition
      {
        name: "key".to_string(),
        description: "Configuration key".to_string(),
        kind: Kind::String,
        attributes: ArgumentAttributes {
          optional: false,
          multiple: false,
          interactive: false, // Regular required argument
          sensitive: false,
          ..Default::default()
        },
        validation_rules: vec![],
        hint: String::new(),
        aliases: vec![],
        tags: vec![],
      },
      ArgumentDefinition
      {
        name: "value".to_string(),
        description: "Configuration value".to_string(),
        kind: Kind::String,
        attributes: ArgumentAttributes {
          optional: false,
          multiple: false,
          interactive: true, // Interactive argument - should trigger special error
          sensitive: true,
          ..Default::default()
        },
        validation_rules: vec![],
        hint: String::new(),
        aliases: vec![],
        tags: vec![],
      },
    ],
    routine_link: None,
    auto_help_enabled: false,
    namespace: String::new(),
    hint: String::new(),
    status: String::new(),
    version: String::new(),
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
    deprecation_message: String::new(),
    examples: vec![],
    http_method_hint: String::new(),
  };

  registry.register(command_def);

  // Test case 1: Missing interactive argument should return UNILANG_ARGUMENT_INTERACTIVE_REQUIRED
  let instruction = GenericInstruction
  {
    command_path_slices: vec!["config".to_string(), "set".to_string()],
    named_arguments: std::collections::BTreeMap::from([
      ("key".to_string(), vec![unilang_parser::Argument {
        name: Some("key".to_string()),
        value: "theme".to_string(),
        name_location: Some(SourceLocation::StrSpan { start: 0, end: 3 }),
        value_location: SourceLocation::StrSpan { start: 5, end: 10 },
      }])
    ]),
    positional_arguments: vec![],
    help_requested: false,
    overall_location: SourceLocation::StrSpan { start: 0, end: 20 },
  };

  let instructions = vec![instruction];
  let analyzer = SemanticAnalyzer::new(&instructions, &registry);
  let error = analyzer.analyze().unwrap_err();
  
  // Verify that we get the specific interactive argument error
  assert!(matches!(
    error,
    unilang::error::Error::Execution(data) if data.code == unilang::data::ErrorCode::ArgumentInteractiveRequired
  ));

  // Test case 2: All arguments provided should succeed
  let instruction_complete = GenericInstruction
  {
    command_path_slices: vec!["config".to_string(), "set".to_string()],
    named_arguments: std::collections::BTreeMap::from([
      ("key".to_string(), vec![unilang_parser::Argument {
        name: Some("key".to_string()),
        value: "theme".to_string(),
        name_location: Some(SourceLocation::StrSpan { start: 0, end: 3 }),
        value_location: SourceLocation::StrSpan { start: 5, end: 10 },
      }]),
      ("value".to_string(), vec![unilang_parser::Argument {
        name: Some("value".to_string()),
        value: "dark".to_string(),
        name_location: Some(SourceLocation::StrSpan { start: 12, end: 17 }),
        value_location: SourceLocation::StrSpan { start: 19, end: 23 },
      }])
    ]),
    positional_arguments: vec![],
    help_requested: false,
    overall_location: SourceLocation::StrSpan { start: 0, end: 30 },
  };

  let instructions_with_all_args = vec![instruction_complete];
  let analyzer_complete = SemanticAnalyzer::new(&instructions_with_all_args, &registry);
  let result = analyzer_complete.analyze();
  
  // This should succeed since both arguments are provided
  assert!(result.is_ok());

  // Test case 3: Missing non-interactive required argument should return UNILANG_ARGUMENT_MISSING  
  let instruction_missing_regular = GenericInstruction
  {
    command_path_slices: vec!["config".to_string(), "set".to_string()],
    named_arguments: std::collections::BTreeMap::from([
      ("value".to_string(), vec![unilang_parser::Argument {
        name: Some("value".to_string()),
        value: "dark".to_string(),
        name_location: Some(SourceLocation::StrSpan { start: 0, end: 5 }),
        value_location: SourceLocation::StrSpan { start: 7, end: 11 },
      }])
    ]),
    positional_arguments: vec![],
    help_requested: false,
    overall_location: SourceLocation::StrSpan { start: 0, end: 20 },
  };

  let instructions_with_missing_args = vec![instruction_missing_regular];
  let analyzer_missing_regular = SemanticAnalyzer::new(&instructions_with_missing_args, &registry);
  let error_regular = analyzer_missing_regular.analyze().unwrap_err();
  
  // Should get regular missing argument error (not interactive)
  assert!(matches!(
    error_regular,
    unilang::error::Error::Execution(data) if data.code == unilang::data::ErrorCode::ArgumentMissing
  ));
}

#[test]
fn test_interactive_optional_argument()
{
  // Test that optional interactive arguments don't trigger the error
  #[allow(deprecated)]
  #[allow(deprecated)]
    let mut registry = CommandRegistry::new();
  
  let command_def = CommandDefinition
  {
    name: "optional.interactive".to_string(),
    description: "Command with optional interactive argument".to_string(),
    arguments: vec!
    [
      ArgumentDefinition
      {
        name: "password".to_string(),
        description: "Optional password".to_string(),
        kind: Kind::String,
        attributes: ArgumentAttributes {
          optional: true, // Optional + interactive should not trigger error when missing
          multiple: false,
          interactive: true,
          sensitive: true,
          default: Some("default_pass".to_string()),
        },
        validation_rules: vec![],
        hint: String::new(),
        aliases: vec![],
        tags: vec![],
      },
    ],
    routine_link: None,
    auto_help_enabled: false,
    namespace: String::new(),
    hint: String::new(),
    status: String::new(),
    version: String::new(),
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
    deprecation_message: String::new(),
    examples: vec![],
    http_method_hint: String::new(),
  };

  registry.register(command_def);

  let instruction = GenericInstruction
  {
    command_path_slices: vec!["optional".to_string(), "interactive".to_string()],
    named_arguments: std::collections::BTreeMap::new(),
    positional_arguments: vec![],
    help_requested: false,
    overall_location: SourceLocation::StrSpan { start: 0, end: 20 },
  };

  let instructions = vec![instruction];
  let analyzer = SemanticAnalyzer::new(&instructions, &registry);
  let result = analyzer.analyze();
  
  // Should succeed because the argument is optional (uses default value)
  assert!(result.is_ok());
}