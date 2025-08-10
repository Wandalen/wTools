use std::collections::HashMap;
use unilang::{
  data::{ArgumentDefinition, CommandDefinition, Kind, OutputData, ErrorData, ArgumentAttributes},
  registry::CommandRegistry,
  semantic::{SemanticAnalyzer, VerifiedCommand},
  interpreter::ExecutionContext,
};
use unilang_parser::{SourceLocation};

// Test Matrix for Runtime Command Registration
//
// Factors:
// - Command Registration: Success, Failure (e.g., duplicate command)
// - Command Execution: Valid arguments, Invalid arguments, Missing arguments
// - Routine Linkage: Correct routine invoked
//
// Combinations:
//
// | ID    | Scenario                               | Expected Outcome                               | Notes                                     |
// |-------|----------------------------------------|------------------------------------------------|-------------------------------------------|
// | T1.1  | Register and execute a simple command  | Command executes successfully                  | Basic registration and execution          |
// | T1.2  | Register command with arguments        | Arguments are correctly bound and used         | Argument parsing and binding              |
// | T1.3  | Attempt to register duplicate command  | Registration fails with an error               | Duplicate command handling                |
// | T1.4  | Execute non-existent command           | Semantic analysis error: Command not found     | Error handling for unknown commands       |
// | T1.5  | Execute command with missing argument  | Semantic analysis error: Missing argument      | Error handling for missing arguments      |
// | T1.6  | Execute command with invalid arg type  | Semantic analysis error: Invalid argument type | Error handling for type mismatches        |

/// Dummy routine for testing.
#[allow(clippy::unnecessary_wraps)]
fn dummy_routine(_verified_command: VerifiedCommand, _context: ExecutionContext) -> Result<OutputData, ErrorData> {
  Ok(OutputData {
    content: "Dummy routine executed!".to_string(),
    format: "text".to_string(),
  })
}

/// Dummy routine for testing arguments.
#[allow(clippy::needless_pass_by_value)]
fn arg_test_routine(verified_command: VerifiedCommand, _context: ExecutionContext) -> Result<OutputData, ErrorData> {
  let arg1 = verified_command
    .arguments
    .get("arg1")
    .ok_or_else(|| ErrorData::new(
      "UNILANG_ARGUMENT_MISSING".to_string(),
      "Argument 'arg1' not found".to_string(),
    ))?
    .as_integer()
    .ok_or_else(|| ErrorData::new(
      "UNILANG_TYPE_MISMATCH".to_string(),
      "Argument 'arg1' is not an integer".to_string(),
    ))?;
  Ok(OutputData {
    content: format!("Arg1: {arg1}"),
    format: "text".to_string(),
  })
}

fn analyze_and_run(
  command_name: &str,
  positional_args: Vec<unilang_parser::Argument>,
  named_args: HashMap<String, unilang_parser::Argument>,
  registry: &CommandRegistry,
) -> Result<Vec<OutputData>, unilang::error::Error> {
  let instructions = vec![unilang_parser::GenericInstruction {
    command_path_slices: command_name.split('.').map(std::string::ToString::to_string).collect(),
    named_arguments: named_args.into_iter().collect(),
    positional_arguments: positional_args,
    help_requested: false,
    overall_location: SourceLocation::StrSpan { start: 0, end: 0 }, // Placeholder
  }];
  let analyzer = SemanticAnalyzer::new(&instructions, registry);
  let verified_commands = analyzer.analyze()?;
  let mut context = ExecutionContext::default();
  let interpreter = unilang::interpreter::Interpreter::new(&verified_commands, registry);
  interpreter.run(&mut context)
}

#[test]
fn test_register_and_execute_simple_command() {
  // Test Matrix Row: T1.1
  let mut registry = CommandRegistry::new();
  let command_def = CommandDefinition {
    name: "simple_cmd".to_string(),
    description: "A simple test command".to_string(),
    arguments: vec![],
    routine_link: Some("dummy_routine".to_string()),
    namespace: ".test".to_string(),
    hint: "Simple command hint".to_string(),
    status: "stable".to_string(),
    version: "1.0.0".to_string(),
    tags: vec!["test".to_string()],
    aliases: vec!["sc".to_string()],
    permissions: vec!["public".to_string()],
    idempotent: true,
    deprecation_message: String::new(),
    examples: vec![],
    http_method_hint: String::new(),
  };
  registry.command_add_runtime(&command_def, Box::new(dummy_routine)).unwrap();

  let result = analyze_and_run("test.simple_cmd", vec![], std::collections::HashMap::new(), &registry);
  assert!(result.is_ok());
  assert_eq!(result.unwrap()[0].content, "Dummy routine executed!");
}

#[test]
fn test_register_command_with_arguments() {
  // Test Matrix Row: T1.2
  let mut registry = CommandRegistry::new();
  let command_def = CommandDefinition {
    name: "arg_cmd".to_string(),
    description: "A command with arguments".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "arg1".to_string(),
      description: "An integer argument".to_string(),
      kind: Kind::Integer,
      attributes: ArgumentAttributes {
        optional: false,
        multiple: false,
        interactive: false,
        sensitive: false,
        ..Default::default()
      },
      validation_rules: vec![],
      hint: "Integer argument hint".to_string(),
      aliases: vec![],
      tags: vec![],
    }],
    routine_link: Some("arg_test_routine".to_string()),
    namespace: ".test".to_string(),
    hint: "Arg command hint".to_string(),
    status: "stable".to_string(),
    version: "1.0.0".to_string(),
    tags: vec!["test".to_string()],
    aliases: vec!["ac".to_string()],
    permissions: vec!["public".to_string()],
    idempotent: true,
    deprecation_message: String::new(),
    examples: vec![],
    http_method_hint: String::new(),
  };
  registry
    .command_add_runtime(&command_def, Box::new(arg_test_routine))
    .unwrap();

  let mut named_args = std::collections::HashMap::new();
  named_args.insert(
    "arg1".to_string(),
    unilang_parser::Argument {
      name: Some("arg1".to_string()),
      value: "123".to_string(),
      name_location: Some(SourceLocation::StrSpan { start: 0, end: 0 }),
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    },
  );
  let result = analyze_and_run("test.arg_cmd", vec![], named_args, &registry);
  assert!(result.is_ok());
  assert_eq!(result.unwrap()[0].content, "Arg1: 123");
}

#[test]
fn test_register_duplicate_command() {
  // Test Matrix Row: T1.3
  let mut registry = CommandRegistry::new();
  let command_def = CommandDefinition {
    name: "duplicate_cmd".to_string(),
    description: "A command to be duplicated".to_string(),
    arguments: vec![],
    routine_link: None,
    namespace: ".test".to_string(),
    hint: "Duplicate command hint".to_string(),
    status: "stable".to_string(),
    version: "1.0.0".to_string(),
    tags: vec!["test".to_string()],
    aliases: vec!["dc".to_string()],
    permissions: vec!["public".to_string()],
    idempotent: true,
    deprecation_message: String::new(),
    examples: vec![],
    http_method_hint: String::new(),
  };
  registry.command_add_runtime(&command_def, Box::new(dummy_routine)).unwrap();

  let result = registry.command_add_runtime(&command_def, Box::new(dummy_routine));
  assert!(result.is_err());
  assert!(matches!( result.unwrap_err(), unilang::error::Error::Execution( data ) if data.code == "UNILANG_COMMAND_ALREADY_EXISTS" ));
}

#[test]
fn test_execute_non_existent_command() {
  // Test Matrix Row: T1.4
  let registry = CommandRegistry::new();
  let result = analyze_and_run("non_existent_cmd", vec![], std::collections::HashMap::new(), &registry);
  assert!(result.is_err());
  assert!(matches!( result.unwrap_err(), unilang::error::Error::Execution( data ) if data.code == "UNILANG_COMMAND_NOT_FOUND" ));
}

#[test]
fn test_execute_command_with_missing_argument() {
  // Test Matrix Row: T1.5
  let mut registry = CommandRegistry::new();
  let command_def = CommandDefinition {
    name: "missing_arg_cmd".to_string(),
    description: "A command with a missing argument".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "required_arg".to_string(),
      description: "A required argument".to_string(),
      kind: Kind::String,
      attributes: ArgumentAttributes {
        optional: false,
        multiple: false,
        interactive: false,
        sensitive: false,
        ..Default::default()
      },
      validation_rules: vec![],
      hint: "Required argument hint".to_string(),
      aliases: vec![],
      tags: vec![],
    }],
    routine_link: Some("dummy_routine".to_string()),
    namespace: ".test".to_string(),
    hint: "Missing arg command hint".to_string(),
    status: "stable".to_string(),
    version: "1.0.0".to_string(),
    tags: vec!["test".to_string()],
    aliases: vec!["mac".to_string()],
    permissions: vec!["public".to_string()],
    idempotent: true,
    deprecation_message: String::new(),
    examples: vec![],
    http_method_hint: String::new(),
  };
  registry.command_add_runtime(&command_def, Box::new(dummy_routine)).unwrap();

  let result = analyze_and_run("test.missing_arg_cmd", vec![], std::collections::HashMap::new(), &registry);
  assert!(result.is_err());
  assert!(matches!( result.unwrap_err(), unilang::error::Error::Execution( data ) if data.code == "UNILANG_ARGUMENT_MISSING" ));
}

#[test]
fn test_execute_command_with_invalid_arg_type() {
  // Test Matrix Row: T1.6
  let mut registry = CommandRegistry::new();
  let command_def = CommandDefinition {
    name: "invalid_type_cmd".to_string(),
    description: "A command with an invalid argument type".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "int_arg".to_string(),
      description: "An integer argument".to_string(),
      kind: Kind::Integer,
      attributes: ArgumentAttributes {
        optional: false,
        multiple: false,
        interactive: false,
        sensitive: false,
        ..Default::default()
      },
      validation_rules: vec![],
      hint: "Integer argument hint".to_string(),
      aliases: vec![],
      tags: vec![],
    }],
    routine_link: Some("dummy_routine".to_string()),
    namespace: ".test".to_string(),
    hint: "Invalid type command hint".to_string(),
    status: "stable".to_string(),
    version: "1.0.0".to_string(),
    tags: vec!["test".to_string()],
    aliases: vec!["itc".to_string()],
    permissions: vec!["public".to_string()],
    idempotent: true,
    deprecation_message: String::new(),
    examples: vec![],
    http_method_hint: String::new(),
  };
  registry.command_add_runtime(&command_def, Box::new(dummy_routine)).unwrap();

  let mut named_args = std::collections::HashMap::new();
  named_args.insert(
    "int_arg".to_string(),
    unilang_parser::Argument {
      name: Some("int_arg".to_string()),
      value: "not_an_integer".to_string(),
      name_location: Some(SourceLocation::StrSpan { start: 0, end: 0 }),
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    },
  );
  let result = analyze_and_run("test.invalid_type_cmd", vec![], named_args, &registry);
  assert!(result.is_err());
  assert!(matches!( result.unwrap_err(), unilang::error::Error::Execution( data ) if data.code == "UNILANG_TYPE_MISMATCH" ));
}
