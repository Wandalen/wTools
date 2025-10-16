//! Tests for the command loader module.
//!
//! This module contains tests for loading command definitions from external
//! files (YAML/JSON) and resolving routine links.
use unilang::
{
  registry::CommandRegistry,
};
// use unilang_parser::SourceLocation; // Temporarily commented out

// Test Matrix for Command Loader
// This matrix covers successful loading of command definitions from valid YAML/JSON strings,
// error handling for invalid YAML/JSON, and basic testing of `routine_link` resolution.

// T1.1: Load a simple command from YAML
// T1.1: Load a simple command from YAML
// T1.2: Load a command with all scalar argument types from YAML
// T1.3: Load a command with collection argument types (List, Map) from YAML
// T1.4: Load a command with complex argument types (JsonString, Object) from YAML
// T1.5: Load a command with `multiple` and `validation_rules` attributes from YAML
// T1.6: Load multiple commands from YAML
// T1.7: Load a command with `routine_link` from YAML (placeholder routine)

// T2.1: Load a simple command from JSON
// T2.2: Load a command with all scalar argument types from JSON
// T2.3: Load a command with collection argument types (List, Map) from JSON
// T2.4: Load a command with complex argument types (JsonString, Object) from JSON
// T2.5: Load a command with `multiple` and `validation_rules` attributes from JSON
// T2.6: Load multiple commands from JSON
// T2.7: Load a command with `routine_link` from JSON (placeholder routine)

// T3.1: Error handling for invalid YAML (syntax error)
// T3.2: Error handling for invalid JSON (syntax error)
// T3.3: Error handling for invalid Kind in YAML
// T3.4: Error handling for invalid Kind in JSON
// T3.5: Error handling for invalid List format in YAML
// T3.6: Error handling for invalid Map format in YAML
// T3.7: Error handling for invalid Enum format in YAML

// qqq: Removed unused `analyze_program` function.

#[ test ]
fn test_load_from_yaml_str_invalid_yaml()
{
  // Test Matrix Row: T3.1
  let yaml_str = r#"
  - name: invalid_command
    description: This is not valid yaml:
    arguments:
    - name: arg1
      kind: String
      attributes:
        optional: false
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      hint: ""
      aliases: []
      tags: []
      interactive: false
      sensitive: false
    namespace: ""
    hint: ""
    status: ""
    version: null
    tags: []
    aliases: []
    permissions: []
    idempotent: false
    deprecation_message: ""
    examples: []
    http_method_hint: ""
    auto_help_enabled: false
  - This line is malformed
  "#;

  let result = CommandRegistry::builder().load_from_yaml_str( yaml_str );

  assert!( result.is_err() );
  // qqq: Check for specific error type/message if possible
}

#[ test ]
fn test_load_from_json_str_invalid_json()
{
  // Test Matrix Row: T3.2
  let json_str = r#"
  [
   {
    "name": "invalid_command_json",
    "description": "This is not valid json",
    "arguments": [
     { "name": "arg1", "kind": "String", "attributes": { "optional": false, "multiple": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "", "aliases": [], "tags": [] }
    ],
    "namespace": "",
    "hint": "",
    "status": "",
    "version": null,
    "tags": [],
    "aliases": [],
    "permissions": [],
    "idempotent": false,
    "deprecation_message": "",
    "examples": [],
    "http_method_hint": "",
    "auto_help_enabled": false
   },
   { This is malformed json }
  ]
  "#;

  let result = CommandRegistry::builder().load_from_json_str( json_str );

  assert!( result.is_err() );
  // qqq: Check for specific error type/message if possible
}

#[ test ]
fn test_load_from_yaml_str_invalid_kind()
{
  // Test Matrix Row: T3.3
  let yaml_str = r#"
  - name: command_with_invalid_kind
    description: Command with an invalid kind
    arguments:
    - name: arg1
      kind: NonExistentKind
      attributes:
        optional: false
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      hint: ""
      aliases: []
      tags: []
      interactive: false
      sensitive: false
    namespace: ""
    hint: ""
    status: ""
    version: null
    tags: []
    aliases: []
    permissions: []
    idempotent: false
    deprecation_message: ""
    examples: []
    http_method_hint: ""
    auto_help_enabled: false
  "#;

  let result = CommandRegistry::builder().load_from_yaml_str( yaml_str );

  assert!( result.is_err() );
  // qqq: Check for specific error type/message if possible
}

#[ test ]
fn test_load_from_json_str_invalid_kind()
{
  // Test Matrix Row: T3.4
  let json_str = r#"
  [
   {
    "name": "command_with_invalid_kind_json",
    "description": "Command with an invalid kind from JSON",
    "arguments": [
     { "name": "arg1", "kind": "NonExistentKind", "attributes": { "optional": false, "multiple": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "", "aliases": [], "tags": [] }
    ],
    "namespace": "",
    "hint": "",
    "status": "",
    "version": null,
    "tags": [],
    "aliases": [],
    "permissions": [],
    "idempotent": false,
    "deprecation_message": "",
    "examples": [],
    "http_method_hint": "",
    "auto_help_enabled": false
   }
  ]
  "#;

  let result = CommandRegistry::builder().load_from_json_str( json_str );

  assert!( result.is_err() );
  // qqq: Check for specific error type/message if possible
}

#[ test ]
fn test_load_from_yaml_str_invalid_list_format()
{
  // Test Matrix Row: T3.5
  let yaml_str = r#"
  - name: command_with_invalid_list
    description: Command with an invalid list kind
    arguments:
    - name: arg1
      kind: List()
      attributes:
        optional: false
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      hint: ""
      aliases: []
      tags: []
      interactive: false
      sensitive: false
    namespace: ""
    hint: ""
    status: ""
    version: null
    tags: []
    aliases: []
    permissions: []
    idempotent: false
    deprecation_message: ""
    examples: []
    http_method_hint: ""
    auto_help_enabled: false
  "#;

  let result = CommandRegistry::builder().load_from_yaml_str( yaml_str );

  assert!( result.is_err() );
  // qqq: Check for specific error type/message if possible
}

#[ test ]
fn test_load_from_yaml_str_invalid_map_format()
{
  // Test Matrix Row: T3.6
  let yaml_str = r#"
  - name: command_with_invalid_map
    description: Command with an invalid map kind
    arguments:
    - name: arg1
      kind: Map(String)
      attributes:
        optional: false
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      hint: ""
      aliases: []
      tags: []
      interactive: false
      sensitive: false
    namespace: ""
    hint: ""
    status: ""
    version: null
    tags: []
    aliases: []
    permissions: []
    idempotent: false
    deprecation_message: ""
    examples: []
    http_method_hint: ""
    auto_help_enabled: false
  "#;

  let result = CommandRegistry::builder().load_from_yaml_str( yaml_str );

  assert!( result.is_err() );
  // qqq: Check for specific error type/message if possible
}

#[ test ]
fn test_load_from_yaml_str_invalid_enum_format()
{
  // Test Matrix Row: T3.7
  let yaml_str = r#"
  - name: command_with_invalid_enum
    description: Command with an invalid enum kind
    arguments:
    - name: arg1
      kind: Enum()
      attributes:
        optional: false
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      hint: ""
      aliases: []
      tags: []
      interactive: false
      sensitive: false
    namespace: ""
    hint: ""
    status: ""
    version: null
    tags: []
    aliases: []
    permissions: []
    idempotent: false
    deprecation_message: ""
    examples: []
    http_method_hint: ""
    auto_help_enabled: false
  "#;

  let result = CommandRegistry::builder().load_from_yaml_str( yaml_str );

  assert!( result.is_err() );
  // qqq: Check for specific error type/message if possible
}
