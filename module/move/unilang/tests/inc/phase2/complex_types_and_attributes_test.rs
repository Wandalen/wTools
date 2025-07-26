use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes };
use unilang_parser::{ SourceLocation };
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang::types::Value;


fn setup_test_environment( command: CommandDefinition ) -> CommandRegistry
{
  let mut registry = CommandRegistry::new();
  registry.commands.insert( command.name.clone(), command );
  registry
}

fn analyze_program( command_name: &str, positional_args: Vec<unilang_parser::Argument>, named_args: std::collections::HashMap<String, unilang_parser::Argument>, registry: &CommandRegistry ) -> Result< Vec< unilang::semantic::VerifiedCommand >, unilang::error::Error >
{
  // eprintln!( "--- analyze_program debug ---" );
  // eprintln!( "Command Name: '{}'", command_name );
  // eprintln!( "Positional Args: {:?}", positional_args );
  // eprintln!( "Named Args: {:?}", named_args );

  let instructions = vec!
  [
    unilang_parser::GenericInstruction
    {
      command_path_slices : command_name.split( '.' ).map( |s| s.to_string() ).collect(),
      named_arguments : named_args,
      positional_arguments : positional_args,
      help_requested : false,
      overall_location : SourceLocation::StrSpan { start : 0, end : 0 }, // Placeholder
    }
  ];
  // eprintln!( "Manually Constructed Instructions: {:?}", instructions );
  let analyzer = SemanticAnalyzer::new( &instructions, registry );
  let result = analyzer.analyze();
  // eprintln!( "Analyzer Result: {:?}", result );
  // eprintln!( "--- analyze_program end ---" );
  result
}

#[test]
fn test_json_string_argument_type()
{
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "json_arg".to_string(),
      description: "A JSON string argument".to_string(),
      kind: Kind::JsonString,
      attributes: ArgumentAttributes::former()
        .optional( false )
        .multiple( false )
        .is_default_arg( false )
        .interactive( false )
        .sensitive( false )
        .form(),
      validation_rules: vec![],
      hint: "".to_string(),
      default_value: None,
      aliases: vec![],
      tags: vec![],
    }],
    routine_link : None,
    namespace: "".to_string(),
    hint: "".to_string(),
    status: "".to_string(),
    version: None,
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
  };
  let registry = setup_test_environment( command );

  // Test Matrix Row: T1.1
  let json_str = r#"{ "key": "value", "num": 123 }"#;
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_parser::Argument
      {
        name : None,
        value : json_str.to_string(),
        name_location : None,
        value_location : SourceLocation::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "json_arg" ).unwrap();
  assert_eq!( *arg, Value::JsonString( json_str.to_string() ) );

  // Test Matrix Row: T1.2
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_parser::Argument
      {
        name : None,
        value : "not a json".to_string(),
        name_location : None,
        value_location : SourceLocation::StrSpan { start : 0, end : 0 },
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
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "object_arg".to_string(),
      description: "An object argument".to_string(),
      kind: Kind::Object,
      attributes: ArgumentAttributes::former()
        .optional( false )
        .multiple( false )
        .is_default_arg( false )
        .interactive( false )
        .sensitive( false )
        .form(),
      validation_rules: vec![],
      hint: "".to_string(),
      default_value: None,
      aliases: vec![],
      tags: vec![],
    }],
    routine_link : None,
    namespace: "".to_string(),
    hint: "".to_string(),
    status: "".to_string(),
    version: None,
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
  };
  let registry = setup_test_environment( command );

  // Test Matrix Row: T1.3
  let object_str = r#"{ "key": "value", "num": 123 }"#;
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_parser::Argument
      {
        name : None,
        value : object_str.to_string(),
        name_location : None,
        value_location : SourceLocation::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "object_arg" ).unwrap();
  assert_eq!( *arg, Value::Object( serde_json::from_str( object_str ).unwrap() ) );

  // Test Matrix Row: T1.4
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_parser::Argument
      {
        name : None,
        value : "not an object".to_string(),
        name_location : None,
        value_location : SourceLocation::StrSpan { start : 0, end : 0 },
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
fn test_multiple_argument()
{
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "multiple_arg".to_string(),
      description: "A multiple string argument".to_string(),
      kind: Kind::String,
      attributes: ArgumentAttributes::former()
        .optional( false )
        .multiple( true )
        .is_default_arg( false )
        .interactive( false )
        .sensitive( false )
        .form(),
      validation_rules: vec![],
      hint: "".to_string(),
      default_value: None,
      aliases: vec![],
      tags: vec![],
    }],
    routine_link : None,
    namespace: "".to_string(),
    hint: "".to_string(),
    status: "".to_string(),
    version: None,
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
  };
  let registry = setup_test_environment( command );

  // Test Matrix Row: T1.5
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_parser::Argument
      {
        name : None,
        value : "val1".to_string(),
        name_location : None,
        value_location : SourceLocation::StrSpan { start : 0, end : 0 },
      },
      unilang_parser::Argument
      {
        name : None,
        value : "val2".to_string(),
        name_location : None,
        value_location : SourceLocation::StrSpan { start : 0, end : 0 },
      },
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "multiple_arg" ).unwrap();
  assert_eq!( *arg, Value::List( vec![ Value::String( "val1".to_string() ), Value::String( "val2".to_string() ) ] ) );
}

#[test]
fn test_validated_argument()
{
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "validated_arg".to_string(),
      description: "A validated integer argument".to_string(),
      kind: Kind::Integer,
      attributes: ArgumentAttributes::former()
        .optional( false )
        .multiple( false )
        .is_default_arg( false )
        .interactive( false )
        .sensitive( false )
        .form(),
      validation_rules: vec!["min:10".to_string(), "max:100".to_string()],
      hint: "".to_string(),
      default_value: None,
      aliases: vec![],
      tags: vec![],
    }],
    routine_link : None,
    namespace: "".to_string(),
    hint: "".to_string(),
    status: "".to_string(),
    version: None,
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
  };
  let registry = setup_test_environment( command );

  // Test Matrix Row: T1.6 (valid)
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_parser::Argument
      {
        name : None,
        value : "50".to_string(),
        name_location : None,
        value_location : SourceLocation::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );

  // Test Matrix Row: T1.7 (min violation)
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_parser::Argument
      {
        name : None,
        value : "5".to_string(),
        name_location : None,
        value_location : SourceLocation::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_err() );
  let error = result.err().unwrap();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "VALIDATION_RULE_FAILED" ) );

  // Test Matrix Row: T1.8 (max violation)
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_parser::Argument
      {
        name : None,
        value : "150".to_string(),
        name_location : None,
        value_location : SourceLocation::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_err() );
  let error = result.err().unwrap();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "VALIDATION_RULE_FAILED" ) );
}

#[test]
fn test_default_argument()
{
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "default_arg".to_string(),
      description: "An argument with a default value".to_string(),
      kind: Kind::String,
      attributes: ArgumentAttributes::former()
        .optional( true )
        .multiple( false )
        .is_default_arg( true )
        .interactive( false )
        .sensitive( false )
        .form(),
      validation_rules: vec![],
      hint: "".to_string(),
      default_value: Some( "default_value_string".to_string() ),
      aliases: vec![],
      tags: vec![],
    }],
    routine_link : None,
    namespace: "".to_string(),
    hint: "".to_string(),
    status: "".to_string(),
    version: None,
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
  };
  let registry = setup_test_environment( command );

  // Test Matrix Row: T1.9 (no value provided, use default)
  let result = analyze_program
  (
    ".test.command",
    vec![],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "default_arg" ).unwrap();
  assert_eq!( *arg, Value::String( "default_value_string".to_string() ) );

  // Test Matrix Row: T1.10 (value provided, override default)
  let result = analyze_program
  (
    ".test.command",
    vec!
    [
      unilang_parser::Argument
      {
        name : None,
        value : "provided_value".to_string(),
        name_location : None,
        value_location : SourceLocation::StrSpan { start : 0, end : 0 },
      }
    ],
    std::collections::HashMap::new(),
    &registry
  );
  assert!( result.is_ok() );
  let verified_command = result.unwrap().remove( 0 );
  let arg = verified_command.arguments.get( "default_arg" ).unwrap();
  assert_eq!( *arg, Value::String( "provided_value".to_string() ) );
}