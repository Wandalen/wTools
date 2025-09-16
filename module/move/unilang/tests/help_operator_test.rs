//! Tests for the help operator (?) functionality
//!
//! This module tests that the ? operator shows help instead of
//! generating missing argument errors.

#[test]
fn test_help_operator_shows_help_not_error()
{
  use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind };
  use unilang::registry::CommandRegistry;
  use unilang::semantic::SemanticAnalyzer;
  use unilang_parser::{ Parser, UnilangParserOptions };

  // Create a command with required arguments
  let mut registry = CommandRegistry::new();
  registry.register( CommandDefinition
  {
    name: "run_file".to_string(),
    namespace: String::new(),
    description: "Run prompts from a file".to_string(),
    hint: "Load and execute prompts".to_string(),
    arguments: vec![
      ArgumentDefinition
      {
        name: "file".to_string(),
        description: "Path to the file containing prompts".to_string(),
        kind: Kind::File,
        attributes: ArgumentAttributes
        {
          optional: false, // Required argument
          ..Default::default()
        },
        validation_rules: vec![],
        hint: "File path".to_string(),
        aliases: vec![],
        tags: vec![],
      }
    ],
    routine_link: None,
    auto_help_enabled: false,
    status: "stable".to_string(),
    version: "1.0.0".to_string(),
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
    deprecation_message: String::new(),
    http_method_hint: String::new(),
    examples: vec![],
  });

  // Parse command with help operator
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( ".run_file ?" ).unwrap();
  
  // Verify help was requested
  assert!( instruction.help_requested, "Help operator should be detected" );
  
  // Run semantic analysis
  let instructions = vec![instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();
  
  // Should return a HELP_REQUESTED error, not MISSING_ARGUMENT
  assert!( result.is_err(), "Should return an error for help" );
  
  let error = result.unwrap_err();
  match error
  {
    unilang::error::Error::Execution( error_data ) =>
    {
      assert_eq!( error_data.code, "HELP_REQUESTED", "Should return HELP_REQUESTED error code" );
      assert!( error_data.message.contains( "run_file" ), "Help should mention the command name" );
      assert!( error_data.message.contains( "file" ), "Help should mention the argument" );
      assert!( !error_data.message.contains( "missing" ), "Should not complain about missing arguments" );
    },
    _ => panic!( "Expected execution error with HELP_REQUESTED" ),
  }
}

#[test]
fn test_help_operator_with_multiple_required_args()
{
  use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind };
  use unilang::registry::CommandRegistry;
  use unilang::semantic::SemanticAnalyzer;
  use unilang_parser::{ Parser, UnilangParserOptions };

  // Create a command with multiple required arguments
  let mut registry = CommandRegistry::new();
  registry.register( CommandDefinition
  {
    name: "copy".to_string(),
    namespace: ".files".to_string(),
    description: "Copy a file".to_string(),
    hint: "Copy files".to_string(),
    arguments: vec![
      ArgumentDefinition
      {
        name: "source".to_string(),
        description: "Source file path".to_string(),
        kind: Kind::File,
        attributes: ArgumentAttributes
        {
          optional: false,
          ..Default::default()
        },
        validation_rules: vec![],
        hint: "Source".to_string(),
        aliases: vec!["src".to_string()],
        tags: vec![],
      },
      ArgumentDefinition
      {
        name: "destination".to_string(),
        description: "Destination file path".to_string(),
        kind: Kind::Path,
        attributes: ArgumentAttributes
        {
          optional: false,
          ..Default::default()
        },
        validation_rules: vec![],
        hint: "Destination".to_string(),
        aliases: vec!["dst".to_string()],
        tags: vec![],
      }
    ],
    routine_link: None,
    auto_help_enabled: false,
    status: "stable".to_string(),
    version: "1.0.0".to_string(),
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
    deprecation_message: String::new(),
    http_method_hint: String::new(),
    examples: vec![],
  });

  // Parse command with help operator
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( ".files.copy ?" ).unwrap();
  
  // Run semantic analysis
  let instructions = vec![instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();
  
  // Should return help, not complain about missing arguments
  assert!( result.is_err() );
  
  let error = result.unwrap_err();
  match error
  {
    unilang::error::Error::Execution( error_data ) =>
    {
      assert_eq!( error_data.code, "HELP_REQUESTED" );
      assert!( error_data.message.contains( "source" ) );
      assert!( error_data.message.contains( "destination" ) );
    },
    _ => panic!( "Expected execution error with HELP_REQUESTED" ),
  }
}

#[test]
fn test_help_operator_takes_precedence_over_validation()
{
  use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, ValidationRule };
  use unilang::registry::CommandRegistry;
  use unilang::semantic::SemanticAnalyzer;
  use unilang_parser::{ Parser, UnilangParserOptions };

  // Create a command with validation rules
  let mut registry = CommandRegistry::new();
  registry.register( CommandDefinition
  {
    name: "set_port".to_string(),
    namespace: String::new(),
    description: "Set server port".to_string(),
    hint: "Configure port".to_string(),
    arguments: vec![
      ArgumentDefinition
      {
        name: "port".to_string(),
        description: "Port number".to_string(),
        kind: Kind::Integer,
        attributes: ArgumentAttributes
        {
          optional: false,
          ..Default::default()
        },
        validation_rules: vec![
          ValidationRule::Min(1.0),
          ValidationRule::Max(65535.0),
        ],
        hint: "1-65535".to_string(),
        aliases: vec![],
        tags: vec![],
      }
    ],
    routine_link: None,
    auto_help_enabled: false,
    status: "stable".to_string(),
    version: "1.0.0".to_string(),
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: true,
    deprecation_message: String::new(),
    http_method_hint: String::new(),
    examples: vec![],
  });

  // Parse command with help - no arguments provided
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( "set_port ?" ).unwrap();
  
  let instructions = vec![instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();
  
  // Should show help, not validation errors
  assert!( result.is_err() );
  
  let error = result.unwrap_err();
  match error
  {
    unilang::error::Error::Execution( error_data ) =>
    {
      assert_eq!( error_data.code, "HELP_REQUESTED" );
      assert!( error_data.message.contains( "1-65535" ), "Should show validation hint in help" );
    },
    _ => panic!( "Expected HELP_REQUESTED error" ),
  }
}

#[test]
fn test_normal_command_without_help_operator_still_validates()
{
  use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind };
  use unilang::registry::CommandRegistry;
  use unilang::semantic::SemanticAnalyzer;
  use unilang_parser::{ Parser, UnilangParserOptions };

  // Same command as first test
  let mut registry = CommandRegistry::new();
  registry.register( CommandDefinition
  {
    name: "run_file".to_string(),
    namespace: String::new(),
    description: "Run prompts from a file".to_string(),
    hint: "Load and execute prompts".to_string(),
    arguments: vec![
      ArgumentDefinition
      {
        name: "file".to_string(),
        description: "Path to the file containing prompts".to_string(),
        kind: Kind::File,
        attributes: ArgumentAttributes
        {
          optional: false,
          ..Default::default()
        },
        validation_rules: vec![],
        hint: "File path".to_string(),
        aliases: vec![],
        tags: vec![],
      }
    ],
    routine_link: None,
    auto_help_enabled: false,
    status: "stable".to_string(),
    version: "1.0.0".to_string(),
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
    deprecation_message: String::new(),
    http_method_hint: String::new(),
    examples: vec![],
  });

  // Parse command WITHOUT help operator
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( ".run_file" ).unwrap();
  
  assert!( !instruction.help_requested, "Help should not be requested" );
  
  // Run semantic analysis
  let instructions = vec![instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();
  
  // Should fail with missing argument error
  assert!( result.is_err() );
  
  let error = result.unwrap_err();
  match error
  {
    unilang::error::Error::Execution( error_data ) =>
    {
      assert_eq!( error_data.code, "UNILANG_ARGUMENT_MISSING", "Should return missing argument error" );
      assert!( error_data.message.contains( "file" ), "Should mention the missing argument" );
    },
    _ => panic!( "Expected missing argument error" ),
  }
}