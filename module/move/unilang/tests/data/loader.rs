//!
//! Tests for the loader module
//!

use unilang::loader::*;
use unilang::data::Kind;

#[test]
fn test_load_command_definitions_from_yaml_str_success()
{
  let yaml_content = r#"
- name: "test_command"
  namespace: ".test"
  description: "A test command"
  hint: "Test hint"
  status: "stable"
  version: "1.0.0"
  tags: ["test"]
  aliases: ["tc"]
  permissions: []
  idempotent: true
  deprecation_message: ""
  http_method_hint: "GET"
  auto_help_enabled: false
  examples: []
  arguments:
    - name: "input"
      kind: "String" 
      description: "Input parameter"
      hint: "Input hint"
      attributes:
        optional: false
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      aliases: []
      tags: []
  routine_link: null
"#;

  let result = load_command_definitions_from_yaml_str(yaml_content);
  assert!(result.is_ok());
  
  let commands = result.unwrap();
  assert_eq!(commands.len(), 1);
  
  let cmd = &commands[0];
  assert_eq!(cmd.name, "test_command");
  assert_eq!(cmd.namespace, ".test");
  assert_eq!(cmd.description, "A test command");
  assert_eq!(cmd.arguments.len(), 1);
  assert_eq!(cmd.arguments[0].name, "input");
  assert!(matches!(cmd.arguments[0].kind, Kind::String));
}

#[test]
fn test_load_command_definitions_from_yaml_str_invalid()
{
  let invalid_yaml = "invalid: yaml: content: {";
  let result = load_command_definitions_from_yaml_str(invalid_yaml);
  assert!(result.is_err());
  assert!(matches!(result.unwrap_err(), unilang::error::Error::Yaml(_)));
}

#[test]
fn test_load_command_definitions_from_json_str_success()
{
  let json_content = r#"[{
    "name": "json_command",
    "namespace": ".json",
    "description": "A JSON test command",
    "hint": "JSON hint",
    "status": "beta",
    "version": "0.9.0",
    "tags": ["json", "test"],
    "aliases": ["jc"],
    "permissions": ["admin"],
    "idempotent": false,
    "deprecation_message": "",
    "http_method_hint": "POST",
    "auto_help_enabled": false,
    "examples": ["json_command input::test"],
    "arguments": [{
      "name": "data",
      "kind": "JsonString",
      "description": "JSON data",
      "hint": "JSON input",
      "attributes": {
        "optional": true,
        "multiple": false,
        "interactive": false,
        "sensitive": false,
        "default": "{}"
      },
      "validation_rules": [],
      "aliases": ["d"],
      "tags": ["required"]
    }],
    "routine_link": null
  }]"#;

  let result = load_command_definitions_from_json_str(json_content);
  assert!(result.is_ok());
  
  let commands = result.unwrap();
  assert_eq!(commands.len(), 1);
  
  let cmd = &commands[0];
  assert_eq!(cmd.name, "json_command");
  assert_eq!(cmd.namespace, ".json");
  assert_eq!(cmd.status, "beta");
  assert_eq!(cmd.tags, vec!["json", "test"]);
  assert_eq!(cmd.permissions, vec!["admin"]);
  assert!(!cmd.idempotent);
  assert_eq!(cmd.arguments[0].attributes.default, Some("{}".to_string()));
}

#[test]
fn test_load_command_definitions_from_json_str_invalid()
{
  let invalid_json = "{invalid json";
  let result = load_command_definitions_from_json_str(invalid_json);
  assert!(result.is_err());
  assert!(matches!(result.unwrap_err(), unilang::error::Error::Json(_)));
}

#[test]
fn test_load_command_definitions_from_yaml_empty()
{
  let empty_yaml = "[]";
  let result = load_command_definitions_from_yaml_str(empty_yaml);
  assert!(result.is_ok());
  assert!(result.unwrap().is_empty());
}

#[test]
fn test_load_command_definitions_from_json_empty()
{
  let empty_json = "[]";
  let result = load_command_definitions_from_json_str(empty_json);
  assert!(result.is_ok());
  assert!(result.unwrap().is_empty());
}

#[test]
fn test_resolve_routine_link_placeholder()
{
  // Test the current placeholder implementation
  let result = resolve_routine_link("some.routine.link");
  assert!(result.is_ok());
  
  // The placeholder routine should be callable
  let routine = result.unwrap();
  let dummy_command = unilang::semantic::VerifiedCommand {
    definition: unilang::data::CommandDefinition::former()
      .name(".test")
      .namespace(String::new())
      .description(String::new())
      .hint(String::new())
      .status(String::new())
      .version(String::new())
      .arguments(vec![])
      .tags(vec![])
      .aliases(vec![])
      .permissions(vec![])
      .idempotent(true)
      .deprecation_message(String::new())
      .http_method_hint(String::new())
      .examples(vec![])
      .routine_link(String::new())
      .end(),
    arguments: std::collections::HashMap::new(),
  };
  let context = unilang::interpreter::ExecutionContext::default();
  let result = routine(dummy_command, context);
  assert!(result.is_ok());
}

#[test]
fn test_load_command_definitions_yaml_with_complex_types()
{
  let yaml_content = r#"
- name: "complex_command"
  namespace: ".complex"
  description: "Command with complex argument types"
  hint: "Complex types test"
  status: "experimental"  
  version: "0.1.0"
  tags: []
  aliases: []
  permissions: []
  idempotent: true
  deprecation_message: ""
  http_method_hint: ""
  auto_help_enabled: false
  examples: []
  arguments:
    - name: "integer_arg"
      kind: "Integer"
      description: "An integer argument"
      hint: "Integer input"
      attributes:
        optional: false
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      aliases: []
      tags: []
    - name: "float_arg"
      kind: "Float"
      description: "A float argument" 
      hint: "Float input"
      attributes:
        optional: true
        multiple: false
        interactive: false
        sensitive: false
        default: "0.0"
      validation_rules: []
      aliases: []
      tags: []
    - name: "bool_arg"
      kind: "Boolean"
      description: "A boolean argument"
      hint: "Boolean input"
      attributes:
        optional: false
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      aliases: []
      tags: []
  routine_link: null
"#;

  let result = load_command_definitions_from_yaml_str(yaml_content);
  assert!(result.is_ok());
  
  let commands = result.unwrap();
  assert_eq!(commands.len(), 1);
  
  let cmd = &commands[0];
  assert_eq!(cmd.arguments.len(), 3);
  assert!(matches!(cmd.arguments[0].kind, Kind::Integer));
  assert!(matches!(cmd.arguments[1].kind, Kind::Float));
  assert!(matches!(cmd.arguments[2].kind, Kind::Boolean));
  assert_eq!(cmd.arguments[1].attributes.default, Some("0.0".to_string()));
}