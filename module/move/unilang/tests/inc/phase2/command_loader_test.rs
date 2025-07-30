//! Tests for the command loader module.
//!
//! This module contains tests for loading command definitions from external
//! files (YAML/JSON) and resolving routine links.
use unilang::
{
  data::Kind,
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
fn test_load_from_yaml_str_simple_command()
{
  // Test Matrix Row: T1.1
  let yaml_str = r#"
    - name: hello
      description: Says hello
      arguments: []
      routine_link: dummy_hello_routine
      namespace: .system
      hint: Says hello
      status: stable
      version: 1.0.0
      tags: [ "greeting" ]
      aliases: [ "hi" ]
      permissions: [ "public" ]
      idempotent: true
      deprecation_message: ""
      examples: []
      http_method_hint: ""
  "#;

  let registry = CommandRegistry::builder().load_from_yaml_str( yaml_str ).unwrap().build();

  assert!( registry.commands.contains_key( ".system.hello" ) );
  let command = registry.commands.get( ".system.hello" ).unwrap();
  assert_eq!( command.name, "hello" );
  assert_eq!( command.description, "Says hello" );
  assert!( command.arguments.is_empty() );
  assert_eq!( command.routine_link, Some( "dummy_hello_routine".to_string() ) );
  assert_eq!( command.namespace, ".system".to_string() );
  assert_eq!( command.hint, "Says hello" );
  assert_eq!( command.status, "stable" );
  assert_eq!( command.version, "1.0.0".to_string() );
  assert_eq!( command.tags, vec![ "greeting".to_string() ] );
  assert_eq!( command.aliases, vec![ "hi".to_string() ] );
  assert_eq!( command.permissions, vec![ "public".to_string() ] );
  assert_eq!( command.idempotent, true );
  assert!( registry.get_routine( ".system.hello" ).is_some() );
}

#[ test ]
fn test_load_from_yaml_str_all_scalar_types()
{
  // Test Matrix Row: T1.2
  let yaml_str = r#"
    - name: scalar_command
      description: Command with scalar arguments
      arguments:
        - name: arg_string
          description: A string argument
          kind: String
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: String hint
          default_value: null
          aliases: []
          tags: []
        - name: arg_integer
          description: An integer argument
          kind: Integer
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: Integer hint
          default_value: null
          aliases: []
          tags: []
        - name: arg_float
          description: A float argument
          kind: Float
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: Float hint
          default_value: null
          aliases: []
          tags: []
        - name: arg_boolean
          description: A boolean argument
          kind: Boolean
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: Boolean hint
          default_value: null
          aliases: []
          tags: []
        - name: arg_path
          description: A path argument
          kind: Path
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: Path hint
          default_value: null
          aliases: []
          tags: []
        - name: arg_file
          description: A file argument
          kind: File
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: File hint
          default_value: null
          aliases: []
          tags: []
        - name: arg_directory
          description: A directory argument
          kind: Directory
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: Directory hint
          default_value: null
          aliases: []
          tags: []
        - name: arg_enum
          description: An enum argument
          kind: Enum(one,two,three)
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: Enum hint
          default_value: null
          aliases: []
          tags: []
        - name: arg_url
          description: A URL argument
          kind: Url
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: Url hint
          default_value: null
          aliases: []
          tags: []
        - name: arg_datetime
          description: A DateTime argument
          kind: DateTime
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: DateTime hint
          default_value: null
          aliases: []
          tags: []
        - name: arg_pattern
          description: A Pattern argument
          kind: Pattern
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: Pattern hint
          default_value: null
          aliases: []
          tags: []
      namespace: .test
      hint: Scalar command hint
      status: experimental
      version: 0.1.0
      tags: [ "test", "scalar" ]
      aliases: [ "s_cmd" ]
      permissions: [ "dev" ]
      idempotent: false
      deprecation_message: ""
      examples: []
      http_method_hint: ""
  "#;

  let registry = CommandRegistry::builder().load_from_yaml_str( yaml_str ).unwrap().build();

  assert!( registry.commands.contains_key( ".test.scalar_command" ) );
  let command = registry.commands.get( ".test.scalar_command" ).unwrap();
  assert_eq!( command.arguments.len(), 11 );
  assert_eq!( command.arguments[ 0 ].kind, Kind::String );
  assert_eq!( command.arguments[ 1 ].kind, Kind::Integer );
  assert_eq!( command.arguments[ 2 ].kind, Kind::Float );
  assert_eq!( command.arguments[ 3 ].kind, Kind::Boolean );
  assert_eq!( command.arguments[ 4 ].kind, Kind::Path );
  assert_eq!( command.arguments[ 5 ].kind, Kind::File );
  assert_eq!( command.arguments[ 6 ].kind, Kind::Directory );
  assert_eq!(
    command.arguments[ 7 ].kind,
    Kind::Enum( vec![ "one".to_string(), "two".to_string(), "three".to_string() ])
  );
  assert_eq!( command.arguments[ 8 ].kind, Kind::Url );
  assert_eq!( command.arguments[ 9 ].kind, Kind::DateTime );
  assert_eq!( command.arguments[ 10 ].kind, Kind::Pattern );

  assert_eq!( command.namespace, ".test".to_string() );
  assert_eq!( command.hint, "Scalar command hint" );
  assert_eq!( command.status, "experimental" );
  assert_eq!( command.version, "0.1.0".to_string() );
  assert_eq!( command.tags, vec![ "test".to_string(), "scalar".to_string() ] );
  assert_eq!( command.aliases, vec![ "s_cmd".to_string() ] );
  assert_eq!( command.permissions, vec![ "dev".to_string() ] );
  assert_eq!( command.idempotent, false );

  assert_eq!( command.arguments[ 0 ].hint, "String hint" );
  assert_eq!( command.arguments[ 0 ].attributes.is_default_arg, false );
  assert_eq!( command.arguments[ 0 ].default_value, None );
  assert_eq!( command.arguments[ 0 ].aliases, Vec::< String >::new() );
  assert_eq!( command.arguments[ 0 ].tags, Vec::< String >::new() );
  assert_eq!( command.arguments[ 0 ].attributes.interactive, false );
  assert_eq!( command.arguments[ 0 ].attributes.sensitive, false );
}

#[ test ]
fn test_load_from_yaml_str_collection_types()
{
  // Test Matrix Row: T1.3
  let yaml_str = r#"
    - name: collection_command
      description: Command with collection arguments
      arguments:
        - name: arg_list_string
          description: A list of strings
          kind: List(String)
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: List string hint
          default_value: null
          aliases: []
          tags: []
        - name: arg_list_integer_custom_delimiter
          description: A list of integers with custom delimiter
          kind: List(Integer,;)
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: List integer hint
          default_value: null
          aliases: []
          tags: []
        - name: arg_map_string_integer
          description: A map of string to integer
          kind: Map(String,Integer)
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: Map string integer hint
          default_value: null
          aliases: []
          tags: []
        - name: arg_map_string_string_custom_delimiters
          description: A map of string to string with custom delimiters
          kind: Map(String,String,;,=)
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: Map string string hint
          default_value: null
          aliases: []
          tags: []
      namespace: .test
      hint: Collection command hint
      status: stable
      version: 1.0.0
      tags: [ "test", "collection" ]
      aliases: [ "c_cmd" ]
      permissions: [ "public" ]
      idempotent: true
      deprecation_message: ""
      examples: []
      http_method_hint: ""
  "#;

  let registry = CommandRegistry::builder().load_from_yaml_str( yaml_str ).unwrap().build();

  assert!( registry.commands.contains_key( ".test.collection_command" ) );
  let command = registry.commands.get( ".test.collection_command" ).unwrap();
  assert_eq!( command.arguments.len(), 4 );
  assert_eq!( command.arguments[ 0 ].kind, Kind::List( Box::new( Kind::String ), None ) );
  assert_eq!( command.arguments[ 1 ].kind, Kind::List( Box::new( Kind::Integer ), Some( ';' ) ) );
  assert_eq!(
    command.arguments[ 2 ].kind,
    Kind::Map( Box::new( Kind::String ), Box::new( Kind::Integer ), None, None )
  );
  assert_eq!(
    command.arguments[ 3 ].kind,
    Kind::Map( Box::new( Kind::String ), Box::new( Kind::String ), Some( ';' ), Some( '=' ) )
  );

  assert_eq!( command.namespace, ".test".to_string() );
  assert_eq!( command.hint, "Collection command hint" );
  assert_eq!( command.status, "stable" );
  assert_eq!( command.version, "1.0.0".to_string() );
  assert_eq!( command.tags, vec![ "test".to_string(), "collection".to_string() ] );
  assert_eq!( command.aliases, vec![ "c_cmd".to_string() ] );
  assert_eq!( command.permissions, vec![ "public".to_string() ] );
  assert_eq!( command.idempotent, true );

  assert_eq!( command.arguments[ 0 ].hint, "List string hint" );
  assert_eq!( command.arguments[ 0 ].attributes.is_default_arg, false );
  assert_eq!( command.arguments[ 0 ].default_value, None );
  assert_eq!( command.arguments[ 0 ].aliases, Vec::< String >::new() );
  assert_eq!( command.arguments[ 0 ].tags, Vec::< String >::new() );
  assert_eq!( command.arguments[ 0 ].attributes.interactive, false );
  assert_eq!( command.arguments[ 0 ].attributes.sensitive, false );
}

#[ test ]
fn test_load_from_yaml_str_complex_types_and_attributes()
{
  // Test Matrix Row: T1.4, T1.5
  let yaml_str = r#"
    - name: complex_command
      description: Command with complex types and attributes
      arguments:
        - name: arg_json_string
          description: A JSON string argument
          kind: JsonString
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: Json string hint
          default_value: null
          aliases: []
          tags: []
        - name: arg_object
          description: An object argument
          kind: Object
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: Object hint
          default_value: null
          aliases: []
          tags: []
        - name: arg_multiple
          description: A multiple string argument
          kind: String
          attributes:
            optional: false
            multiple: true
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: []
          hint: Multiple string hint
          default_value: null
          aliases: []
          tags: []
        - name: arg_validated
          description: A validated integer argument
          kind: Integer
          attributes:
            optional: false
            multiple: false
            is_default_arg: false
            interactive: false
            sensitive: false
          validation_rules: ["min:10", "max:100"]
          hint: Validated integer hint
          default_value: null
          aliases: []
          tags: []
        - name: arg_default
          description: An argument with a default value
          kind: String
          attributes:
            optional: true
            multiple: false
            is_default_arg: true
            interactive: false
            sensitive: false
          validation_rules: []
          hint: Default value hint
          default_value: "default_string"
          aliases: []
          tags: []
      namespace: .test
      hint: Complex command hint
      status: stable
      version: 1.0.0
      tags: [ "test", "complex" ]
      aliases: [ "comp_cmd" ]
      permissions: [ "public" ]
      idempotent: false
      deprecation_message: ""
      examples: []
      http_method_hint: ""
  "#;

  let registry = CommandRegistry::builder().load_from_yaml_str( yaml_str ).unwrap().build();

  assert!( registry.commands.contains_key( ".test.complex_command" ) );
  let command = registry.commands.get( ".test.complex_command" ).unwrap();
  assert_eq!( command.arguments.len(), 5 );
  assert_eq!( command.arguments[ 0 ].kind, Kind::JsonString );
  assert_eq!( command.arguments[ 1 ].kind, Kind::Object );
  assert!( command.arguments[ 2 ].attributes.multiple );
  assert_eq!(
    command.arguments[ 3 ].validation_rules,
    vec![ "min:10".to_string(), "max:100".to_string() ]
  );
  assert_eq!( command.arguments[ 4 ].attributes.is_default_arg, true );
  assert_eq!( command.arguments[ 4 ].default_value, Some( "default_string".to_string() ) );

  assert_eq!( command.namespace, ".test".to_string() );
  assert_eq!( command.hint, "Complex command hint" );
  assert_eq!( command.status, "stable" );
  assert_eq!( command.version, "1.0.0".to_string() );
  assert_eq!( command.tags, vec![ "test".to_string(), "complex".to_string() ] );
  assert_eq!( command.aliases, vec![ "comp_cmd".to_string() ] );
  assert_eq!( command.permissions, vec![ "public".to_string() ] );
  assert_eq!( command.idempotent, false );

  assert_eq!( command.arguments[ 0 ].hint, "Json string hint" );
  assert_eq!( command.arguments[ 0 ].attributes.is_default_arg, false );
  assert_eq!( command.arguments[ 0 ].default_value, None );
  assert_eq!( command.arguments[ 0 ].aliases, Vec::< String >::new() );
  assert_eq!( command.arguments[ 0 ].tags, Vec::< String >::new() );
  assert_eq!( command.arguments[ 0 ].attributes.interactive, false );
  assert_eq!( command.arguments[ 0 ].attributes.sensitive, false );
}

#[ test ]
fn test_load_from_yaml_str_multiple_commands()
{
  // Test Matrix Row: T1.6
  let yaml_str = r#"
    - name: command1
      description: First command
      arguments: []
      namespace: .group1
      hint: Command 1 hint
      status: stable
      version: 1.0.0
      tags: []
      aliases: []
      permissions: []
      idempotent: false
      deprecation_message: ""
      examples: []
      http_method_hint: ""
    - name: command2
      description: Second command
      arguments: []
      namespace: .group1
      hint: Command 2 hint
      status: stable
      version: 1.0.0
      tags: []
      aliases: []
      permissions: []
      idempotent: false
      deprecation_message: ""
      examples: []
      http_method_hint: ""
  "#;

  let registry = CommandRegistry::builder().load_from_yaml_str( yaml_str ).unwrap().build();

  assert!( registry.commands.contains_key( ".group1.command1" ) );
  assert!( registry.commands.contains_key( ".group1.command2" ) );
  assert_eq!(
    registry.commands.get( ".group1.command1" ).unwrap().namespace,
    ".group1".to_string()
  );
  assert_eq!(
    registry.commands.get( ".group1.command2" ).unwrap().namespace,
    ".group1".to_string()
  );
}

#[ test ]
fn test_load_from_json_str_simple_command()
{
  // Test Matrix Row: T2.1
  let json_str = r#"
    [
      {
        "name": "hello_json",
        "description": "Says hello from JSON",
        "arguments": [],
        "routine_link": "dummy_hello_json_routine",
        "namespace": ".system",
        "hint": "Says hello from JSON",
        "status": "stable",
        "version": "1.0.0",
        "tags": [ "greeting" ],
        "aliases": [ "hi_json" ],
        "permissions": [ "public" ],
        "idempotent": true,
        "deprecation_message": "",
        "examples": [],
        "http_method_hint": ""
      }
    ]
  "#;

  let registry = CommandRegistry::builder().load_from_json_str( json_str ).unwrap().build();

  assert!( registry.commands.contains_key( ".system.hello_json" ) );
  let command = registry.commands.get( ".system.hello_json" ).unwrap();
  assert_eq!( command.name, "hello_json" );
  assert_eq!( command.description, "Says hello from JSON" );
  assert!( command.arguments.is_empty() );
  assert_eq!( command.routine_link, Some( "dummy_hello_json_routine".to_string() ) );
  assert_eq!( command.namespace, ".system".to_string() );
  assert_eq!( command.hint, "Says hello from JSON" );
  assert_eq!( command.status, "stable" );
  assert_eq!( command.version, "1.0.0".to_string() );
  assert_eq!( command.tags, vec![ "greeting".to_string() ] );
  assert_eq!( command.aliases, vec![ "hi_json".to_string() ] );
  assert_eq!( command.permissions, vec![ "public".to_string() ] );
  assert_eq!( command.idempotent, true );
  assert!( registry.get_routine( ".system.hello_json" ).is_some() );
}

#[ test ]
fn test_load_from_json_str_all_scalar_types()
{
  // Test Matrix Row: T2.2
  let json_str = r#"
    [
      {
        "name": "scalar_command_json",
        "description": "Command with scalar arguments from JSON",
        "arguments": [
          { "name": "arg_string", "description": "A string argument", "kind": "String", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "String hint", "default_value": null, "aliases": [], "tags": [] },
          { "name": "arg_integer", "description": "An integer argument", "kind": "Integer", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "Integer hint", "default_value": null, "aliases": [], "tags": [] },
          { "name": "arg_float", "description": "A float argument", "kind": "Float", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "Float hint", "default_value": null, "aliases": [], "tags": [] },
          { "name": "arg_boolean", "description": "A boolean argument", "kind": "Boolean", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "Boolean hint", "default_value": null, "aliases": [], "tags": [] },
          { "name": "arg_path", "description": "A path argument", "kind": "Path", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "Path hint", "default_value": null, "aliases": [], "tags": [] },
          { "name": "arg_file", "description": "A file argument", "kind": "File", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "File hint", "default_value": null, "aliases": [], "tags": [] },
          { "name": "arg_directory", "description": "A directory argument", "kind": "Directory", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "Directory hint", "default_value": null, "aliases": [], "tags": [] },
          { "name": "arg_enum", "description": "An enum argument", "kind": "Enum(one,two,three)", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "Enum hint", "default_value": null, "aliases": [], "tags": [] },
          { "name": "arg_url", "description": "A URL argument", "kind": "Url", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "Url hint", "default_value": null, "aliases": [], "tags": [] },
          { "name": "arg_datetime", "description": "A DateTime argument", "kind": "DateTime", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "DateTime hint", "default_value": null, "aliases": [], "tags": [] },
          { "name": "arg_pattern", "description": "A Pattern argument", "kind": "Pattern", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "Pattern hint", "default_value": null, "aliases": [], "tags": [] }
        ],
        "namespace": ".test",
        "hint": "Scalar command hint",
        "status": "experimental",
        "version": "0.1.0",
        "tags": [ "test", "scalar" ],
        "aliases": [ "s_cmd_json" ],
        "permissions": [ "dev" ],
        "idempotent": false,
        "deprecation_message": "",
        "examples": [],
        "http_method_hint": ""
      }
    ]
  "#;

  let registry = CommandRegistry::builder().load_from_json_str( json_str ).unwrap().build();

  assert!( registry.commands.contains_key( ".test.scalar_command_json" ) );
  let command = registry.commands.get( ".test.scalar_command_json" ).unwrap();
  assert_eq!( command.arguments.len(), 11 );
  assert_eq!( command.arguments[ 0 ].kind, Kind::String );
  assert_eq!( command.arguments[ 1 ].kind, Kind::Integer );
  assert_eq!( command.arguments[ 2 ].kind, Kind::Float );
  assert_eq!( command.arguments[ 3 ].kind, Kind::Boolean );
  assert_eq!( command.arguments[ 4 ].kind, Kind::Path );
  assert_eq!( command.arguments[ 5 ].kind, Kind::File );
  assert_eq!( command.arguments[ 6 ].kind, Kind::Directory );
  assert_eq!(
    command.arguments[ 7 ].kind,
    Kind::Enum( vec![ "one".to_string(), "two".to_string(), "three".to_string() ])
  );
  assert_eq!( command.arguments[ 8 ].kind, Kind::Url );
  assert_eq!( command.arguments[ 9 ].kind, Kind::DateTime );
  assert_eq!( command.arguments[ 10 ].kind, Kind::Pattern );

  assert_eq!( command.namespace, ".test".to_string() );
  assert_eq!( command.hint, "Scalar command hint" );
  assert_eq!( command.status, "experimental" );
  assert_eq!( command.version, "0.1.0".to_string() );
  assert_eq!( command.tags, vec![ "test".to_string(), "scalar".to_string() ] );
  assert_eq!( command.aliases, vec![ "s_cmd_json".to_string() ] );
  assert_eq!( command.permissions, vec![ "dev".to_string() ] );
  assert_eq!( command.idempotent, false );

  assert_eq!( command.arguments[ 0 ].hint, "String hint" );
  assert_eq!( command.arguments[ 0 ].attributes.is_default_arg, false );
  assert_eq!( command.arguments[ 0 ].default_value, None );
  assert_eq!( command.arguments[ 0 ].aliases, Vec::< String >::new() );
  assert_eq!( command.arguments[ 0 ].tags, Vec::< String >::new() );
  assert_eq!( command.arguments[ 0 ].attributes.interactive, false );
  assert_eq!( command.arguments[ 0 ].attributes.sensitive, false );
}

#[ test ]
fn test_load_from_json_str_collection_types()
{
  // Test Matrix Row: T2.3
  let json_str = r#"
    [
      {
        "name": "collection_command_json",
        "description": "Command with collection arguments from JSON",
        "arguments": [
          { "name": "arg_list_string", "description": "A list of strings", "kind": "List(String)", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "List string hint", "default_value": null, "aliases": [], "tags": [] },
          { "name": "arg_list_integer_custom_delimiter", "description": "A list of integers with custom delimiter", "kind": "List(Integer,;)", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "List integer hint", "default_value": null, "aliases": [], "tags": [] },
          { "name": "arg_map_string_integer", "description": "A map of string to integer", "kind": "Map(String,Integer)", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "Map string integer hint", "default_value": null, "aliases": [], "tags": [] },
          { "name": "arg_map_string_string_custom_delimiters", "description": "A map of string to string with custom delimiters", "kind": "Map(String,String,;,=)", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "Map string string hint", "default_value": null, "aliases": [], "tags": [] }
        ],
        "namespace": ".test",
        "hint": "Collection command hint",
        "status": "stable",
        "version": "1.0.0",
        "tags": [ "test", "collection" ],
        "aliases": [ "c_cmd_json" ],
        "permissions": [ "public" ],
        "idempotent": true,
        "deprecation_message": "",
        "examples": [],
        "http_method_hint": ""
      }
    ]
  "#;

  let registry = CommandRegistry::builder().load_from_json_str( json_str ).unwrap().build();

  assert!( registry.commands.contains_key( ".test.collection_command_json" ) );
  let command = registry.commands.get( ".test.collection_command_json" ).unwrap();
  assert_eq!( command.arguments.len(), 4 );
  assert_eq!( command.arguments[ 0 ].kind, Kind::List( Box::new( Kind::String ), None ) );
  assert_eq!( command.arguments[ 1 ].kind, Kind::List( Box::new( Kind::Integer ), Some( ';' ) ) );
  assert_eq!(
    command.arguments[ 2 ].kind,
    Kind::Map( Box::new( Kind::String ), Box::new( Kind::Integer ), None, None )
  );
  assert_eq!(
    command.arguments[ 3 ].kind,
    Kind::Map( Box::new( Kind::String ), Box::new( Kind::String ), Some( ';' ), Some( '=' ) )
  );

  assert_eq!( command.namespace, ".test".to_string() );
  assert_eq!( command.hint, "Collection command hint" );
  assert_eq!( command.status, "stable" );
  assert_eq!( command.version, "1.0.0".to_string() );
  assert_eq!( command.tags, vec![ "test".to_string(), "collection".to_string() ] );
  assert_eq!( command.aliases, vec![ "c_cmd_json".to_string() ] );
  assert_eq!( command.permissions, vec![ "public".to_string() ] );
  assert_eq!( command.idempotent, true );

  assert_eq!( command.arguments[ 0 ].hint, "List string hint" );
  assert_eq!( command.arguments[ 0 ].attributes.is_default_arg, false );
  assert_eq!( command.arguments[ 0 ].default_value, None );
  assert_eq!( command.arguments[ 0 ].aliases, Vec::< String >::new() );
  assert_eq!( command.arguments[ 0 ].tags, Vec::< String >::new() );
  assert_eq!( command.arguments[ 0 ].attributes.interactive, false );
  assert_eq!( command.arguments[ 0 ].attributes.sensitive, false );
}

#[ test ]
fn test_load_from_json_str_complex_types_and_attributes()
{
  // Test Matrix Row: T2.4, T2.5
  let json_str = r#"
    [
      {
        "name": "complex_command_json",
        "description": "Command with complex types and attributes from JSON",
        "arguments": [
          { "name": "arg_json_string", "description": "A JSON string argument", "kind": "JsonString", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "Json string hint", "default_value": null, "aliases": [], "tags": [] },
          { "name": "arg_object", "description": "An object argument", "kind": "Object", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "Object hint", "default_value": null, "aliases": [], "tags": [] },
          { "name": "arg_multiple", "description": "A multiple string argument", "kind": "String", "attributes": { "optional": false, "multiple": true, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "Multiple string hint", "default_value": null, "aliases": [], "tags": [] },
          { "name": "arg_validated", "description": "A validated integer argument", "kind": "Integer", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": ["min:10", "max:100"], "hint": "Validated integer hint", "default_value": null, "aliases": [], "tags": [] },
          { "name": "arg_default", "description": "An argument with a default value", "kind": "String", "attributes": { "optional": true, "multiple": false, "is_default_arg": true, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "Default value hint", "is_default_arg": true, "default_value": "default_string", "aliases": [], "tags": [] }
        ],
        "namespace": ".test",
        "hint": "Complex command hint",
        "status": "stable",
        "version": "1.0.0",
        "tags": [ "test", "complex" ],
        "aliases": [ "comp_cmd_json" ],
        "permissions": [ "public" ],
        "idempotent": false,
        "deprecation_message": "",
        "examples": [],
        "http_method_hint": ""
      }
    ]
  "#;

  let registry = CommandRegistry::builder().load_from_json_str( json_str ).unwrap().build();

  assert!( registry.commands.contains_key( ".test.complex_command_json" ) );
  let command = registry.commands.get( ".test.complex_command_json" ).unwrap();
  assert_eq!( command.arguments.len(), 5 );
  assert_eq!( command.arguments[ 0 ].kind, Kind::JsonString );
  assert_eq!( command.arguments[ 1 ].kind, Kind::Object );
  assert!( command.arguments[ 2 ].attributes.multiple );
  assert_eq!(
    command.arguments[ 3 ].validation_rules,
    vec![ "min:10".to_string(), "max:100".to_string() ]
  );
  assert_eq!( command.arguments[ 4 ].attributes.is_default_arg, true );
  assert_eq!( command.arguments[ 4 ].default_value, Some( "default_string".to_string() ) );

  assert_eq!( command.namespace, ".test".to_string() );
  assert_eq!( command.hint, "Complex command hint" );
  assert_eq!( command.status, "stable" );
  assert_eq!( command.version, "1.0.0".to_string() );
  assert_eq!( command.tags, vec![ "test".to_string(), "complex".to_string() ] );
  assert_eq!( command.aliases, vec![ "comp_cmd_json".to_string() ] );
  assert_eq!( command.permissions, vec![ "public".to_string() ] );
  assert_eq!( command.idempotent, false );

  assert_eq!( command.arguments[ 0 ].hint, "Json string hint" );
  assert_eq!( command.arguments[ 0 ].attributes.is_default_arg, false );
  assert_eq!( command.arguments[ 0 ].default_value, None );
  assert_eq!( command.arguments[ 0 ].aliases, Vec::< String >::new() );
  assert_eq!( command.arguments[ 0 ].tags, Vec::< String >::new() );
  assert_eq!( command.arguments[ 0 ].attributes.interactive, false );
  assert_eq!( command.arguments[ 0 ].attributes.sensitive, false );
}

#[ test ]
fn test_load_from_json_str_multiple_commands()
{
  // Test Matrix Row: T2.6
  let json_str = r#"
    [
      {
        "name": "command1_json",
        "description": "First command from JSON",
        "arguments": [],
        "namespace": ".group1",
        "hint": "Command 1 hint",
        "status": "stable",
        "version": "1.0.0",
        "tags": [],
        "aliases": [],
        "permissions": [],
        "idempotent": false,
        "deprecation_message": "",
        "examples": [],
        "http_method_hint": ""
      },
      {
        "name": "command2_json",
        "description": "Second command from JSON",
        "arguments": [],
        "namespace": ".group1",
        "hint": "Command 2 hint",
        "status": "stable",
        "version": "1.0.0",
        "tags": [],
        "aliases": [],
        "permissions": [],
        "idempotent": false,
        "deprecation_message": "",
        "examples": [],
        "http_method_hint": ""
      }
    ]
  "#;

  let registry = CommandRegistry::builder().load_from_json_str( json_str ).unwrap().build();

  assert!( registry.commands.contains_key( ".group1.command1_json" ) );
  assert!( registry.commands.contains_key( ".group1.command2_json" ) );
  assert_eq!(
    registry.commands.get( ".group1.command1_json" ).unwrap().namespace,
    ".group1".to_string()
  );
  assert_eq!(
    registry.commands.get( ".group1.command2_json" ).unwrap().namespace,
    ".group1".to_string()
  );
}

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
          optional: false
          multiple: false
          validation_rules: []
          hint: ""
          is_default_arg: false
          default_value: null
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
          { "name": "arg1", "kind": "String", "attributes": { "optional": false, "multiple": false, "is_default_arg": false, "interactive": false, "sensitive": false }, "validation_rules": [], "hint": "", "default_value": null, "aliases": [], "tags": [] }
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
        "http_method_hint": ""
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
          optional: false
          multiple: false
          validation_rules: []
          hint: ""
          is_default_arg: false
          default_value: null
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
          { "name": "arg1", "kind": "NonExistentKind", "optional": false, "multiple": false, "validation_rules": [], "hint": "", "is_default_arg": false, "default_value": null, "aliases": [], "tags": [], "interactive": false, "sensitive": false, "deprecation_message": "", "examples": [], "http_method_hint": "" }
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
        "http_method_hint": ""
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
          optional: false
          multiple: false
          validation_rules: []
          hint: ""
          is_default_arg: false
          default_value: null
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
          optional: false
          multiple: false
          validation_rules: []
          hint: ""
          is_default_arg: false
          default_value: null
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
          optional: false
          multiple: false
          validation_rules: []
          hint: ""
          is_default_arg: false
          default_value: null
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
  "#;

  let result = CommandRegistry::builder().load_from_yaml_str( yaml_str );

  assert!( result.is_err() );
  // qqq: Check for specific error type/message if possible
}
