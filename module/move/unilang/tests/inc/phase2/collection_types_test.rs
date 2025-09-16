use std::collections::HashMap;
use unilang::data::{ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes};
use unilang_parser::{SourceLocation};
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;

// Test Matrix for Collection Types
//
// Factors:
// - Kind: List, Map
// - Delimiters: Default, Custom
// - Expected Outcome: Correct Kind parsing
//
// Combinations:
//
// | ID    | Kind String           | Expected Kind                                     | Notes                                     |
// |-------|-----------------------|---------------------------------------------------|-------------------------------------------|
// | T1.1  | List(String)          | Kind::List(String, None)                          | Basic list of strings                     |
// | T1.2  | List(Integer,;)       | Kind::List(Integer, Some(';'))                   | List of integers with custom delimiter    |
// | T1.3  | Map(String,Integer)   | Kind::Map(String, Integer, None, None)           | Basic map of string to integer            |
// | T1.4  | Map(String,String,;,=)| Kind::Map(String, String, Some(';'), Some('='))  | Map with custom entry and key-value delimiters |

fn setup_test_environment(command: CommandDefinition) -> CommandRegistry {
  let mut registry = CommandRegistry::new();
  registry.register(command);
  registry
}

fn analyze_program(
  command_name: &str,
  positional_args: Vec<unilang_parser::Argument>,
  named_args: HashMap<String, unilang_parser::Argument>,
  registry: &CommandRegistry,
) -> Result<Vec<unilang::semantic::VerifiedCommand>, unilang::error::Error> {
  // eprintln!( "--- analyze_program debug ---" );
  // eprintln!( "Command Name: '{}'", command_name );
  // eprintln!( "Positional Args: {:?}", positional_args );
  // eprintln!( "Named Args: {:?}", named_args );

  let instructions = vec![unilang_parser::GenericInstruction {
    command_path_slices: command_name.split('.').map(std::string::ToString::to_string).collect(),
    named_arguments: named_args.into_iter().collect(),
    positional_arguments: positional_args,
    help_requested: false,
    overall_location: SourceLocation::StrSpan { start: 0, end: 0 }, // Placeholder
  }];
  // eprintln!( "Manually Constructed Instructions: {:?}", instructions );
  let analyzer = SemanticAnalyzer::new(&instructions, registry);
  
  // eprintln!( "Analyzer Result: {:?}", result );
  // eprintln!( "--- analyze_program end ---" );
  analyzer.analyze()
}

#[test]
fn test_list_string_kind() {
  // Test Matrix Row: T1.1
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "list_arg".to_string(),
      description: "A list of strings".to_string(),
      kind: Kind::List(Box::new(Kind::String), None),
      attributes: ArgumentAttributes {
        optional: false,
        multiple: false,
        interactive: false,
        sensitive: false,
        ..Default::default()
      },
      validation_rules: vec![],
      hint: String::new(),
      aliases: vec![],
      tags: vec![],
    }],
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
  let registry = setup_test_environment(command);
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "a,b,c".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("list_arg").unwrap();
  assert_eq!(*arg, unilang::types::Value::List(vec![unilang::types::Value::String("a".to_string()), unilang::types::Value::String("b".to_string()), unilang::types::Value::String("c".to_string())]));
}

#[test]
fn test_list_integer_custom_delimiter_kind() {
  // Test Matrix Row: T1.2
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "list_arg".to_string(),
      description: "A list of integers with custom delimiter".to_string(),
      kind: Kind::List(Box::new(Kind::Integer), Some(';')),
      attributes: ArgumentAttributes {
        optional: false,
        multiple: false,
        interactive: false,
        sensitive: false,
        ..Default::default()
      },
      validation_rules: vec![],
      hint: String::new(),
      aliases: vec![],
      tags: vec![],
    }],
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
  let registry = setup_test_environment(command);
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "1;2;3".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("list_arg").unwrap();
  assert_eq!(*arg, unilang::types::Value::List(vec![unilang::types::Value::Integer(1), unilang::types::Value::Integer(2), unilang::types::Value::Integer(3)]));
}

#[test]
fn test_map_string_integer_kind() {
  // Test Matrix Row: T1.3
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "map_arg".to_string(),
      description: "A map of string to integer".to_string(),
      kind: Kind::Map(Box::new(Kind::String), Box::new(Kind::Integer), None, Some(':')),
      attributes: ArgumentAttributes {
        optional: false,
        multiple: false,
        interactive: false,
        sensitive: false,
        ..Default::default()
      },
      validation_rules: vec![],
      hint: String::new(),
      aliases: vec![],
      tags: vec![],
    }],
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
  let registry = setup_test_environment(command);
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "a:1,b:2".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("map_arg").unwrap();
  let mut expected_map = std::collections::HashMap::new();
  expected_map.insert("a".to_string(), unilang::types::Value::Integer(1));
  expected_map.insert("b".to_string(), unilang::types::Value::Integer(2));
  assert_eq!(*arg, unilang::types::Value::Map(expected_map));
}

#[test]
fn test_map_string_string_custom_delimiters_kind() {
  // Test Matrix Row: T1.4
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "map_arg".to_string(),
      description: "A map of string to string with custom delimiters".to_string(),
      kind: Kind::Map(Box::new(Kind::String), Box::new(Kind::String), Some(';'), Some('=')),
      attributes: ArgumentAttributes {
        optional: false,
        multiple: false,
        interactive: false,
        sensitive: false,
        ..Default::default()
      },
      validation_rules: vec![],
      hint: String::new(),
      aliases: vec![],
      tags: vec![],
    }],
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
  let registry = setup_test_environment(command);
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "a=1;b=2".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("map_arg").unwrap();
  let mut expected_map = std::collections::HashMap::new();
  expected_map.insert("a".to_string(), unilang::types::Value::String("1".to_string()));
  expected_map.insert("b".to_string(), unilang::types::Value::String("2".to_string()));
  assert_eq!(*arg, unilang::types::Value::Map(expected_map));
}
