use unilang::
{
  data::
  {
    Kind,
  },
  registry::CommandRegistry,
};


// Test Matrix for Command Loader
// This matrix covers successful loading of command definitions from valid YAML/JSON strings,
// error handling for invalid YAML/JSON, and basic testing of `routine_link` resolution.

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

#[ test ]
fn test_load_from_yaml_str_simple_command()
{
  // Test Matrix Row: T1.1
  let yaml_str = r#"
    - name: hello
      description: Says hello
      arguments: []
      routine_link: dummy_hello_routine
  "#;

  let registry = CommandRegistry::builder()
  .load_from_yaml_str( yaml_str )
  .unwrap()
  .build();

  assert!( registry.commands.contains_key( "hello" ) );
  let command = registry.commands.get( "hello" ).unwrap();
  assert_eq!( command.name, "hello" );
  assert_eq!( command.description, "Says hello" );
  assert!( command.arguments.is_empty() );
  assert_eq!( command.routine_link, Some( "dummy_hello_routine".to_string() ) );
  assert!( registry.get_routine( "hello" ).is_some() );
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
          optional: false
          multiple: false
          validation_rules: []
        - name: arg_integer
          description: An integer argument
          kind: Integer
          optional: false
          multiple: false
          validation_rules: []
        - name: arg_float
          description: A float argument
          kind: Float
          optional: false
          multiple: false
          validation_rules: []
        - name: arg_boolean
          description: A boolean argument
          kind: Boolean
          optional: false
          multiple: false
          validation_rules: []
        - name: arg_path
          description: A path argument
          kind: Path
          optional: false
          multiple: false
          validation_rules: []
        - name: arg_file
          description: A file argument
          kind: File
          optional: false
          multiple: false
          validation_rules: []
        - name: arg_directory
          description: A directory argument
          kind: Directory
          optional: false
          multiple: false
          validation_rules: []
        - name: arg_enum
          description: An enum argument
          kind: Enum(one,two,three)
          optional: false
          multiple: false
          validation_rules: []
        - name: arg_url
          description: A URL argument
          kind: Url
          optional: false
          multiple: false
          validation_rules: []
        - name: arg_datetime
          description: A DateTime argument
          kind: DateTime
          optional: false
          multiple: false
          validation_rules: []
        - name: arg_pattern
          description: A Pattern argument
          kind: Pattern
          optional: false
          multiple: false
          validation_rules: []
  "#;

  let registry = CommandRegistry::builder()
  .load_from_yaml_str( yaml_str )
  .unwrap()
  .build();

  assert!( registry.commands.contains_key( "scalar_command" ) );
  let command = registry.commands.get( "scalar_command" ).unwrap();
  assert_eq!( command.arguments.len(), 11 );
  assert_eq!( command.arguments[ 0 ].kind, Kind::String );
  assert_eq!( command.arguments[ 1 ].kind, Kind::Integer );
  assert_eq!( command.arguments[ 2 ].kind, Kind::Float );
  assert_eq!( command.arguments[ 3 ].kind, Kind::Boolean );
  assert_eq!( command.arguments[ 4 ].kind, Kind::Path );
  assert_eq!( command.arguments[ 5 ].kind, Kind::File );
  assert_eq!( command.arguments[ 6 ].kind, Kind::Directory );
  assert_eq!( command.arguments[ 7 ].kind, Kind::Enum( vec![ "one".to_string(), "two".to_string(), "three".to_string() ] ) );
  assert_eq!( command.arguments[ 8 ].kind, Kind::Url );
  assert_eq!( command.arguments[ 9 ].kind, Kind::DateTime );
  assert_eq!( command.arguments[ 10 ].kind, Kind::Pattern );
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
          optional: false
          multiple: false
          validation_rules: []
        - name: arg_list_integer_custom_delimiter
          description: A list of integers with custom delimiter
          kind: List(Integer,;)
          optional: false
          multiple: false
          validation_rules: []
        - name: arg_map_string_integer
          description: A map of string to integer
          kind: Map(String,Integer)
          optional: false
          multiple: false
          validation_rules: []
        - name: arg_map_string_string_custom_delimiters
          description: A map of string to string with custom delimiters
          kind: Map(String,String,;,=)
          optional: false
          multiple: false
          validation_rules: []
  "#;

  let registry = CommandRegistry::builder()
  .load_from_yaml_str( yaml_str )
  .unwrap()
  .build();

  assert!( registry.commands.contains_key( "collection_command" ) );
  let command = registry.commands.get( "collection_command" ).unwrap();
  assert_eq!( command.arguments.len(), 4 );
  assert_eq!( command.arguments[ 0 ].kind, Kind::List( Box::new( Kind::String ), None ) );
  assert_eq!( command.arguments[ 1 ].kind, Kind::List( Box::new( Kind::Integer ), Some( ';' ) ) );
  assert_eq!( command.arguments[ 2 ].kind, Kind::Map( Box::new( Kind::String ), Box::new( Kind::Integer ), None, None ) );
  assert_eq!( command.arguments[ 3 ].kind, Kind::Map( Box::new( Kind::String ), Box::new( Kind::String ), Some( ';' ), Some( '=' ) ) );
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
          optional: false
          multiple: false
          validation_rules: []
        - name: arg_object
          description: An object argument
          kind: Object
          optional: false
          multiple: false
          validation_rules: []
        - name: arg_multiple
          description: A multiple string argument
          kind: String
          optional: false
          multiple: true
          validation_rules: []
        - name: arg_validated
          description: A validated integer argument
          kind: Integer
          optional: false
          multiple: false
          validation_rules: ["min:10", "max:100"]
  "#;

  let registry = CommandRegistry::builder()
  .load_from_yaml_str( yaml_str )
  .unwrap()
  .build();

  assert!( registry.commands.contains_key( "complex_command" ) );
  let command = registry.commands.get( "complex_command" ).unwrap();
  assert_eq!( command.arguments.len(), 4 );
  assert_eq!( command.arguments[ 0 ].kind, Kind::JsonString );
  assert_eq!( command.arguments[ 1 ].kind, Kind::Object );
  assert!( command.arguments[ 2 ].multiple );
  assert_eq!( command.arguments[ 3 ].validation_rules, vec![ "min:10".to_string(), "max:100".to_string() ] );
}

#[ test ]
fn test_load_from_yaml_str_multiple_commands()
{
  // Test Matrix Row: T1.6
  let yaml_str = r#"
    - name: command1
      description: First command
      arguments: []
    - name: command2
      description: Second command
      arguments: []
  "#;

  let registry = CommandRegistry::builder()
  .load_from_yaml_str( yaml_str )
  .unwrap()
  .build();

  assert!( registry.commands.contains_key( "command1" ) );
  assert!( registry.commands.contains_key( "command2" ) );
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
        "routine_link": "dummy_hello_json_routine"
      }
    ]
  "#;

  let registry = CommandRegistry::builder()
  .load_from_json_str( json_str )
  .unwrap()
  .build();

  assert!( registry.commands.contains_key( "hello_json" ) );
  let command = registry.commands.get( "hello_json" ).unwrap();
  assert_eq!( command.name, "hello_json" );
  assert_eq!( command.description, "Says hello from JSON" );
  assert!( command.arguments.is_empty() );
  assert_eq!( command.routine_link, Some( "dummy_hello_json_routine".to_string() ) );
  assert!( registry.get_routine( "hello_json" ).is_some() );
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
          { "name": "arg_string", "description": "A string argument", "kind": "String", "optional": false, "multiple": false, "validation_rules": [] },
          { "name": "arg_integer", "description": "An integer argument", "kind": "Integer", "optional": false, "multiple": false, "validation_rules": [] },
          { "name": "arg_float", "description": "A float argument", "kind": "Float", "optional": false, "multiple": false, "validation_rules": [] },
          { "name": "arg_boolean", "description": "A boolean argument", "kind": "Boolean", "optional": false, "multiple": false, "validation_rules": [] },
          { "name": "arg_path", "description": "A path argument", "kind": "Path", "optional": false, "multiple": false, "validation_rules": [] },
          { "name": "arg_file", "description": "A file argument", "kind": "File", "optional": false, "multiple": false, "validation_rules": [] },
          { "name": "arg_directory", "description": "A directory argument", "kind": "Directory", "optional": false, "multiple": false, "validation_rules": [] },
          { "name": "arg_enum", "description": "An enum argument", "kind": "Enum(one,two,three)", "optional": false, "multiple": false, "validation_rules": [] },
          { "name": "arg_url", "description": "A URL argument", "kind": "Url", "optional": false, "multiple": false, "validation_rules": [] },
          { "name": "arg_datetime", "description": "A DateTime argument", "kind": "DateTime", "optional": false, "multiple": false, "validation_rules": [] },
          { "name": "arg_pattern", "description": "A Pattern argument", "kind": "Pattern", "optional": false, "multiple": false, "validation_rules": [] }
        ]
      }
    ]
  "#;

  let registry = CommandRegistry::builder()
  .load_from_json_str( json_str )
  .unwrap()
  .build();

  assert!( registry.commands.contains_key( "scalar_command_json" ) );
  let command = registry.commands.get( "scalar_command_json" ).unwrap();
  assert_eq!( command.arguments.len(), 11 );
  assert_eq!( command.arguments[ 0 ].kind, Kind::String );
  assert_eq!( command.arguments[ 1 ].kind, Kind::Integer );
  assert_eq!( command.arguments[ 2 ].kind, Kind::Float );
  assert_eq!( command.arguments[ 3 ].kind, Kind::Boolean );
  assert_eq!( command.arguments[ 4 ].kind, Kind::Path );
  assert_eq!( command.arguments[ 5 ].kind, Kind::File );
  assert_eq!( command.arguments[ 6 ].kind, Kind::Directory );
  assert_eq!( command.arguments[ 7 ].kind, Kind::Enum( vec![ "one".to_string(), "two".to_string(), "three".to_string() ] ) );
  assert_eq!( command.arguments[ 8 ].kind, Kind::Url );
  assert_eq!( command.arguments[ 9 ].kind, Kind::DateTime );
  assert_eq!( command.arguments[ 10 ].kind, Kind::Pattern );
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
          { "name": "arg_list_string", "description": "A list of strings", "kind": "List(String)", "optional": false, "multiple": false, "validation_rules": [] },
          { "name": "arg_list_integer_custom_delimiter", "description": "A list of integers with custom delimiter", "kind": "List(Integer,;)", "optional": false, "multiple": false, "validation_rules": [] },
          { "name": "arg_map_string_integer", "description": "A map of string to integer", "kind": "Map(String,Integer)", "optional": false, "multiple": false, "validation_rules": [] },
          { "name": "arg_map_string_string_custom_delimiters", "description": "A map of string to string with custom delimiters", "kind": "Map(String,String,;,=)", "optional": false, "multiple": false, "validation_rules": [] }
        ]
      }
    ]
  "#;

  let registry = CommandRegistry::builder()
  .load_from_json_str( json_str )
  .unwrap()
  .build();

  assert!( registry.commands.contains_key( "collection_command_json" ) );
  let command = registry.commands.get( "collection_command_json" ).unwrap();
  assert_eq!( command.arguments.len(), 4 );
  assert_eq!( command.arguments[ 0 ].kind, Kind::List( Box::new( Kind::String ), None ) );
  assert_eq!( command.arguments[ 1 ].kind, Kind::List( Box::new( Kind::Integer ), Some( ';' ) ) );
  assert_eq!( command.arguments[ 2 ].kind, Kind::Map( Box::new( Kind::String ), Box::new( Kind::Integer ), None, None ) );
  assert_eq!( command.arguments[ 3 ].kind, Kind::Map( Box::new( Kind::String ), Box::new( Kind::String ), Some( ';' ), Some( '=' ) ) );
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
          { "name": "arg_json_string", "description": "A JSON string argument", "kind": "JsonString", "optional": false, "multiple": false, "validation_rules": [] },
          { "name": "arg_object", "description": "An object argument", "kind": "Object", "optional": false, "multiple": false, "validation_rules": [] },
          { "name": "arg_multiple", "description": "A multiple string argument", "kind": "String", "optional": false, "multiple": true, "validation_rules": [] },
          { "name": "arg_validated", "description": "A validated integer argument", "kind": "Integer", "optional": false, "multiple": false, "validation_rules": ["min:10", "max:100"] }
        ]
      }
    ]
  "#;

  let registry = CommandRegistry::builder()
  .load_from_json_str( json_str )
  .unwrap()
  .build();

  assert!( registry.commands.contains_key( "complex_command_json" ) );
  let command = registry.commands.get( "complex_command_json" ).unwrap();
  assert_eq!( command.arguments.len(), 4 );
  assert_eq!( command.arguments[ 0 ].kind, Kind::JsonString );
  assert_eq!( command.arguments[ 1 ].kind, Kind::Object );
  assert!( command.arguments[ 2 ].multiple );
  assert_eq!( command.arguments[ 3 ].validation_rules, vec![ "min:10".to_string(), "max:100".to_string() ] );
}

#[ test ]
fn test_load_from_json_str_multiple_commands()
{
  // Test Matrix Row: T2.6
  let json_str = r#"
    [
      { "name": "command1_json", "description": "First command from JSON", "arguments": [] },
      { "name": "command2_json", "description": "Second command from JSON", "arguments": [] }
    ]
  "#;

  let registry = CommandRegistry::builder()
  .load_from_json_str( json_str )
  .unwrap()
  .build();

  assert!( registry.commands.contains_key( "command1_json" ) );
  assert!( registry.commands.contains_key( "command2_json" ) );
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
    - This line is malformed
  "#;

  let result = CommandRegistry::builder()
  .load_from_yaml_str( yaml_str );

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
          { "name": "arg1", "kind": "String" }
        ]
      },
      { This is malformed json }
    ]
  "#;

  let result = CommandRegistry::builder()
  .load_from_json_str( json_str );

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
  "#;

  let result = CommandRegistry::builder()
  .load_from_yaml_str( yaml_str );

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
          { "name": "arg1", "kind": "NonExistentKind", "optional": false, "multiple": false, "validation_rules": [] }
        ]
      }
    ]
  "#;

  let result = CommandRegistry::builder()
  .load_from_json_str( json_str );

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
  "#;

  let result = CommandRegistry::builder()
  .load_from_yaml_str( yaml_str );

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
  "#;

  let result = CommandRegistry::builder()
  .load_from_yaml_str( yaml_str );

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
  "#;

  let result = CommandRegistry::builder()
  .load_from_yaml_str( yaml_str );

  assert!( result.is_err() );
  // qqq: Check for specific error type/message if possible
}