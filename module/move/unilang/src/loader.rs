//!
//! Handles loading command definitions from external files (YAML/JSON).
//!

/// Internal namespace.
mod private
{
  use crate::
  {
    data::{ CommandDefinition, OutputData },
    error::Error,
    registry::CommandRoutine,
  };

///
/// Loads command definitions from a YAML string.
///
/// # Errors
///
/// Returns an `Error::Yaml` if the YAML string is invalid.
///
pub fn load_command_definitions_from_yaml_str( yaml_str : &str ) -> Result< Vec< CommandDefinition >, Error >
{
  let definitions : Vec< CommandDefinition > = serde_yaml::from_str( yaml_str ).map_err( Error::Yaml )?;
  Ok( definitions )
}

///
/// Loads command definitions from a JSON string.
///
/// # Errors
///
/// Returns an `Error::Json` if the JSON string is invalid.
///
pub fn load_command_definitions_from_json_str( json_str : &str ) -> Result< Vec< CommandDefinition >, Error >
{
  let definitions : Vec< CommandDefinition > = serde_json::from_str( json_str ).map_err( Error::Json )?;
  Ok( definitions )
}

///
/// Resolves a routine link string to a `CommandRoutine`.
///
/// This is a placeholder for now. In a later increment, this will handle
/// dynamic loading of routines from shared libraries or Rust modules.
///
/// # Errors
///
/// Returns an `Error::Execution` if the link is not recognized or if
/// dynamic loading fails (in future increments).
///
pub fn resolve_routine_link( _link : &str ) -> Result< CommandRoutine, Error >
{
  // qqq: This is a placeholder. Actual dynamic loading will be implemented in a later increment.
  // For now, return a dummy routine or an error if the link is not recognized.
  // For testing purposes, we can return a routine that just prints the link.
  Ok( Box::new( move | _args, _context |
  {
    // println!( "Dummy routine executed for link: {}", link );
    Ok( OutputData
    {
      content : String::new(),
      format : String::new(),
    })
  }) )
}

}

#[cfg(test)]
mod tests
{
  use super::*;
  use crate::data::Kind;

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
    assert!(matches!(result.unwrap_err(), crate::error::Error::Yaml(_)));
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
    assert!(matches!(result.unwrap_err(), crate::error::Error::Json(_)));
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
    let dummy_command = crate::semantic::VerifiedCommand {
      definition: crate::data::CommandDefinition::former()
        .name("test")
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
        .form(),
      arguments: std::collections::HashMap::new(),
    };
    let context = crate::interpreter::ExecutionContext::default();
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
}

mod_interface::mod_interface!
{
  exposed use private::load_command_definitions_from_yaml_str;
  exposed use private::load_command_definitions_from_json_str;
  exposed use private::resolve_routine_link;
  
  prelude use private::load_command_definitions_from_yaml_str;
  prelude use private::load_command_definitions_from_json_str;
}
