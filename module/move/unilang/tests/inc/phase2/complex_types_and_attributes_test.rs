use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind };
use unilang_instruction_parser::{ Parser, UnilangParserOptions }; // Updated import
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang::types::Value;
// use std::collections::HashMap; // Removed unused import
use serde_json::json;

use unilang_instruction_parser::SourceLocation::StrSpan;
fn setup_test_environment( command: CommandDefinition ) -> CommandRegistry
{
  let mut registry = CommandRegistry::new();
  registry.commands.insert( command.name.clone(), command );
  registry
}

fn analyze_program( command_name: &str, positional_args: Vec<unilang_instruction_parser::Argument>, named_args: std::collections::HashMap<String, unilang_instruction_parser::Argument>, registry: &CommandRegistry ) -> Result< Vec< unilang::semantic::VerifiedCommand >, unilang::error::Error >
{
  let instructions = vec!
  [
    unilang_instruction_parser::GenericInstruction
    {
      command_path_slices : command_name.split( '.' ).map( |s| s.to_string() ).collect(),
      named_arguments : named_args,
      positional_arguments : positional_args,
      help_requested : false,
      overall_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 }, // Placeholder
    }
  ];
  let analyzer = SemanticAnalyzer::new( &instructions, registry );
  analyzer.analyze()
}

#[test]
fn test_json_string_argument_type()
{
  // Test Matrix Row: T3.1
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "json_arg".to_string(),
      description: "A JSON string argument".to_string(),
      kind: Kind::JsonString,
      optional: false,
      multiple: false,
      validation_rules: vec![],
    }],
    routine_link : None,
  };
  let registry = setup_test_environment( command );
  let json_str = r#"{"key": "value"}"#; // Input string for parsing
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_instruction_parser::Argument
      {
        name : None,
        value : json_str.to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "json_arg" ).unwrap();
  assert_eq!( *arg, Value::JsonString( json_str.to_string() ) );

  // Test Matrix Row: T3.2
  let json_str_invalid = r#"{"key": "value""#; // Input string for parsing
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_instruction_parser::Argument
      {
        name : None,
        value : json_str_invalid.to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_err() );
  let error = result.err().unwrap();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "INVALID_ARGUMENT_TYPE" ) );
}

#[test]
fn test_object_argument_type()
{
  // Test Matrix Row: T3.3
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "object_arg".to_string(),
      description: "An object argument".to_string(),
      kind: Kind::Object,
      optional: false,
      multiple: false,
      validation_rules: vec![],
    }],
    routine_link : None,
  };
  let registry = setup_test_environment( command );
  let json_str = r#"{"num": 123}"#; // Input string for parsing
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_instruction_parser::Argument
      {
        name : None,
        value : json_str.to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "object_arg" ).unwrap();
  assert_eq!( *arg, Value::Object( json!({ "num": 123 }) ) );

  // Test Matrix Row: T3.4
  let json_str_invalid = r#"invalid"#; // Input string for parsing
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_instruction_parser::Argument
      {
        name : None,
        value : json_str_invalid.to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_err() );
  let error = result.err().unwrap();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "INVALID_ARGUMENT_TYPE" ) );
}

#[test]
fn test_multiple_attribute()
{
  // Test Matrix Row: T3.5
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "multi_arg".to_string(),
      description: "A multiple string argument".to_string(),
      kind: Kind::String,
      optional: false,
      multiple: true,
      validation_rules: vec![],
    }],
    routine_link : None,
  };
  let registry = setup_test_environment( command );
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_instruction_parser::Argument
      {
        name : None,
        value : "val1".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      },
      unilang_instruction_parser::Argument
      {
        name : None,
        value : "val2".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "multi_arg" ).unwrap();
  assert_eq!( *arg, Value::List( vec![ Value::String( "val1".to_string() ), Value::String( "val2".to_string() ) ] ) );

  // Test Matrix Row: T3.6
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "multi_arg".to_string(),
      description: "A multiple integer argument".to_string(),
      kind: Kind::Integer,
      optional: false,
      multiple: true,
      validation_rules: vec![],
    }],
    routine_link : None,
  };
  let registry = setup_test_environment( command );
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_instruction_parser::Argument
      {
        name : None,
        value : "1".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      },
      unilang_instruction_parser::Argument
      {
        name : None,
        value : "2".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "multi_arg" ).unwrap();
  assert_eq!( *arg, Value::List( vec![ Value::Integer( 1 ), Value::Integer( 2 ) ] ) );

  // Test Matrix Row: T3.13
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "multi_list_arg".to_string(),
      description: "A multiple list of strings argument".to_string(),
      kind: Kind::List( Box::new( Kind::String ), None ),
      optional: false,
      multiple: true,
      validation_rules: vec![],
    }],
    routine_link : None,
  };
  let registry = setup_test_environment( command );
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_instruction_parser::Argument
      {
        name : None,
        value : "a,b".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      },
      unilang_instruction_parser::Argument
      {
        name : None,
        value : "c,d".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "multi_list_arg" ).unwrap();
  assert_eq!( *arg, Value::List( vec![ Value::List( vec![ Value::String( "a".to_string() ), Value::String( "b".to_string() ) ] ), Value::List( vec![ Value::String( "c".to_string() ), Value::String( "d".to_string() ) ] ) ] ) );
}

#[test]
fn test_validation_rules()
{
  // Test Matrix Row: T3.8
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "num_arg".to_string(),
      description: "A number argument with range validation".to_string(),
      kind: Kind::Integer,
      optional: false,
      multiple: false,
      validation_rules: vec!["min:10".to_string(), "max:20".to_string()],
    }],
    routine_link : None,
  };
  let registry = setup_test_environment( command );
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_instruction_parser::Argument
      {
        name : None,
        value : "15".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "num_arg" ).unwrap();
  assert_eq!( *arg, Value::Integer( 15 ) );

  // Test Matrix Row: T3.9
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_instruction_parser::Argument
      {
        name : None,
        value : "5".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_err() );
  let error = result.err().unwrap();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "VALIDATION_RULE_FAILED" ) );

  // Test Matrix Row: T3.10
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_instruction_parser::Argument
      {
        name : None,
        value : "25".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_err() );
  let error = result.err().unwrap();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "VALIDATION_RULE_FAILED" ) );

  // Test Matrix Row: T3.11
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "str_arg".to_string(),
      description: "A string argument with regex validation".to_string(),
      kind: Kind::String,
      optional: false,
      multiple: false,
      validation_rules: vec!["regex:^[a-zA-Z]+$".to_string()],
    }],
    routine_link : None,
  };
  let registry = setup_test_environment( command );
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_instruction_parser::Argument
      {
        name : None,
        value : "abc".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "str_arg" ).unwrap();
  assert_eq!( *arg, Value::String( "abc".to_string() ) );

  // Test Matrix Row: T3.12
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_instruction_parser::Argument
      {
        name : None,
        value : "abc1".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_err() );
  let error = result.err().unwrap();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "VALIDATION_RULE_FAILED" ) );

  // Test Matrix Row: T3.7 - min_length validation for multiple arguments
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "multi_str_arg".to_string(),
      description: "A multiple string argument with validation".to_string(),
      kind: Kind::String,
      optional: false,
      multiple: true,
      validation_rules: vec!["min_length:3".to_string()],
    }],
    routine_link : None,
  };
  let registry = setup_test_environment( command );
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_instruction_parser::Argument
      {
        name : None,
        value : "ab".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      },
      unilang_instruction_parser::Argument
      {
        name : None,
        value : "cde".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_err() );
  let error = result.err().unwrap();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "VALIDATION_RULE_FAILED" ) );
}