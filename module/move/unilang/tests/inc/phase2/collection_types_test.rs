use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind };
use unilang_instruction_parser::{ Parser, UnilangParserOptions }; // Updated import
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang::types::Value;
use std::collections::HashMap;
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
fn test_list_argument_type()
{
  // Test Matrix Row: T2.1
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "list_arg".to_string(),
      description: "A list argument".to_string(),
      kind: Kind::List( Box::new( Kind::String ), None ),
      optional: false,
      multiple: false,
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
        value : "val1,val2,val3".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "list_arg" ).unwrap();
  assert_eq!( *arg, Value::List( vec![ Value::String( "val1".to_string() ), Value::String( "val2".to_string() ), Value::String( "val3".to_string() ) ] ) );

  // Test Matrix Row: T2.2
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "list_arg".to_string(),
      description: "A list argument".to_string(),
      kind: Kind::List( Box::new( Kind::Integer ), None ),
      optional: false,
      multiple: false,
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
        value : "1,2,3".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "list_arg" ).unwrap();
  assert_eq!( *arg, Value::List( vec![ Value::Integer( 1 ), Value::Integer( 2 ), Value::Integer( 3 ) ] ) );

  // Test Matrix Row: T2.3
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "list_arg".to_string(),
      description: "A list argument".to_string(),
      kind: Kind::List( Box::new( Kind::String ), Some( ';' ) ),
      optional: false,
      multiple: false,
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
        value : "val1;val2;val3".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "list_arg" ).unwrap();
  assert_eq!( *arg, Value::List( vec![ Value::String( "val1".to_string() ), Value::String( "val2".to_string() ), Value::String( "val3".to_string() ) ] ) );

  // Test Matrix Row: T2.4
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "list_arg".to_string(),
      description: "A list argument".to_string(),
      kind: Kind::List( Box::new( Kind::String ), None ),
      optional: false,
      multiple: false,
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
        value : "".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "list_arg" ).unwrap();
  assert_eq!( *arg, Value::List( vec![] ) );

  // Test Matrix Row: T2.5
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "list_arg".to_string(),
      description: "A list argument".to_string(),
      kind: Kind::List( Box::new( Kind::Integer ), None ),
      optional: false,
      multiple: false,
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
        value : "1,invalid,3".to_string(),
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
fn test_map_argument_type()
{
  // Test Matrix Row: T2.6
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "map_arg".to_string(),
      description: "A map argument".to_string(),
      kind: Kind::Map( Box::new( Kind::String ), Box::new( Kind::String ), None, None ),
      optional: false,
      multiple: false,
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
        value : "key1=val1,key2=val2".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "map_arg" ).unwrap();
  let mut expected_map = HashMap::new();
  expected_map.insert( "key1".to_string(), Value::String( "val1".to_string() ) );
  expected_map.insert( "key2".to_string(), Value::String( "val2".to_string() ) );
  assert_eq!( *arg, Value::Map( expected_map ) );

  // Test Matrix Row: T2.7
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "map_arg".to_string(),
      description: "A map argument".to_string(),
      kind: Kind::Map( Box::new( Kind::String ), Box::new( Kind::Integer ), None, None ),
      optional: false,
      multiple: false,
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
        value : "num1=1,num2=2".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "map_arg" ).unwrap();
  let mut expected_map = HashMap::new();
  expected_map.insert( "num1".to_string(), Value::Integer( 1 ) );
  expected_map.insert( "num2".to_string(), Value::Integer( 2 ) );
  assert_eq!( *arg, Value::Map( expected_map ) );

  // Test Matrix Row: T2.8
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "map_arg".to_string(),
      description: "A map argument".to_string(),
      kind: Kind::Map( Box::new( Kind::String ), Box::new( Kind::String ), Some( ';' ), Some( ':' ) ),
      optional: false,
      multiple: false,
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
        value : "key1:val1;key2:val2".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "map_arg" ).unwrap();
  let mut expected_map = HashMap::new();
  expected_map.insert( "key1".to_string(), Value::String( "val1".to_string() ) );
  expected_map.insert( "key2".to_string(), Value::String( "val2".to_string() ) );
  assert_eq!( *arg, Value::Map( expected_map ) );

  // Test Matrix Row: T2.9
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "map_arg".to_string(),
      description: "A map argument".to_string(),
      kind: Kind::Map( Box::new( Kind::String ), Box::new( Kind::String ), None, None ),
      optional: false,
      multiple: false,
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
        value : "".to_string(),
        name_location : None,
        value_location : unilang_instruction_parser::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "map_arg" ).unwrap();
  assert_eq!( *arg, Value::Map( HashMap::new() ) );

  // Test Matrix Row: T2.10
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "map_arg".to_string(),
      description: "A map argument".to_string(),
      kind: Kind::Map( Box::new( Kind::String ), Box::new( Kind::String ), None, None ),
      optional: false,
      multiple: false,
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
        value : "key1=val1,key2".to_string(),
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

  // Test Matrix Row: T2.11
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "map_arg".to_string(),
      description: "A map argument".to_string(),
      kind: Kind::Map( Box::new( Kind::String ), Box::new( Kind::Integer ), None, None ),
      optional: false,
      multiple: false,
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
        value : "key1=val1,key2=invalid".to_string(),
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