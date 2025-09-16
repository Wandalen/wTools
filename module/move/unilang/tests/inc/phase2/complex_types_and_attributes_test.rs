use std::collections::HashMap;
use unilang::data::{ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes, ValidationRule};
use unilang_parser::{SourceLocation};
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang::types::Value;

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
fn test_json_string_argument_type() {
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "json_arg".to_string(),
      description: "A JSON string argument".to_string(),
      kind: Kind::JsonString,
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

  // Test Matrix Row: T1.1
  let json_str = r#"{ "key": "value", "num": 123 }"#;
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: json_str.to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("json_arg").unwrap();
  assert_eq!(*arg, Value::JsonString(json_str.to_string()));

  // Test Matrix Row: T1.2
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "not a json".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_err());
  let error = result.err().unwrap();
  assert!(matches!( error, unilang::error::Error::Execution( data ) if data.code == "UNILANG_TYPE_MISMATCH" ));
}

#[test]
fn test_object_argument_type() {
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "object_arg".to_string(),
      description: "An object argument".to_string(),
      kind: Kind::Object,
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

  // Test Matrix Row: T1.3
  let object_str = r#"{ "key": "value", "num": 123 }"#;
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: object_str.to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("object_arg").unwrap();
  assert_eq!(*arg, Value::Object(serde_json::from_str(object_str).unwrap()));

  // Test Matrix Row: T1.4
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "not an object".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_err());
  let error = result.err().unwrap();
  assert!(matches!( error, unilang::error::Error::Execution( data ) if data.code == "UNILANG_TYPE_MISMATCH" ));
}

#[test]
fn test_multiple_argument() {
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "multiple_arg".to_string(),
      description: "A multiple string argument".to_string(),
      kind: Kind::String,
      attributes: ArgumentAttributes {
        optional: false,
        multiple: true,
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

  // Test Matrix Row: T1.5
  let result = analyze_program(
    ".test.command",
    vec![
      unilang_parser::Argument {
        name: None,
        value: "val1".to_string(),
        name_location: None,
        value_location: SourceLocation::StrSpan { start: 0, end: 0 },
      },
      unilang_parser::Argument {
        name: None,
        value: "val2".to_string(),
        name_location: None,
        value_location: SourceLocation::StrSpan { start: 0, end: 0 },
      },
    ],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("multiple_arg").unwrap();
  assert_eq!(*arg, Value::List(vec![Value::String("val1".to_string()), Value::String("val2".to_string())]));
}

#[test]
fn test_validated_argument() {
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "validated_arg".to_string(),
      description: "A validated integer argument".to_string(),
      kind: Kind::Integer,
      attributes: ArgumentAttributes {
        optional: false,
        multiple: false,
        interactive: false,
        sensitive: false,
        ..Default::default()
      },
      validation_rules: vec![
        ValidationRule::Min(10.0),
        ValidationRule::Max(100.0)
      ],
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

  // Test Matrix Row: T1.6 (valid)
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "50".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_ok());

  // Test Matrix Row: T1.7 (min violation)
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "5".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_err());
  let error = result.err().unwrap();
  assert!(matches!( error, unilang::error::Error::Execution( data ) if data.code == "UNILANG_VALIDATION_RULE_FAILED" ));

  // Test Matrix Row: T1.8 (max violation)
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "150".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_err());
  let error = result.err().unwrap();
  assert!(matches!( error, unilang::error::Error::Execution( data ) if data.code == "UNILANG_VALIDATION_RULE_FAILED" ));
}

#[test]
fn test_default_argument() {
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "default_arg".to_string(),
      description: "An argument with a default value".to_string(),
      kind: Kind::String,
      attributes: ArgumentAttributes {
        optional: true,
        multiple: false,
        interactive: false,
        sensitive: false,
        default: Some("default_value_string".to_string()),
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

  // Test Matrix Row: T1.9 (no value provided, use default)
  let result = analyze_program(".test.command", vec![], HashMap::new(), &registry);
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("default_arg").unwrap();
  assert_eq!(*arg, Value::String("default_value_string".to_string()));

  // Test Matrix Row: T1.10 (value provided, override default)
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "provided_value".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("default_arg").unwrap();
  assert_eq!(*arg, Value::String("provided_value".to_string()));
}
