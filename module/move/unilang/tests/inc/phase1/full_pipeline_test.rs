//!
//! Integration tests for the full Phase 1 pipeline.
//!

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, ErrorData, Kind, OutputData };
use unilang::help::HelpGenerator; // Added for help_generator_tests
use unilang::interpreter::{ ExecutionContext, Interpreter };
use unilang::registry::CommandRegistry;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::types::Value;
use unilang_parser::{ GenericInstruction, Parser, UnilangParserOptions };

///
/// Tests for the `SemanticAnalyzer`.
///
/// This test covers the following combinations from the Test Matrix:
/// - T3.1: A valid command with correct arguments.
/// - T3.2: An unknown command.
/// - T3.3: A command with a missing required argument.
/// - T3.4: A command with an argument of the wrong type.
/// - T3.5: A command with too many arguments.
///
#[ test ]
fn semantic_analyzer_tests()
{
  let mut registry = CommandRegistry::new();
  registry.register( CommandDefinition
  {
    name : "test_cmd".to_string(),
    description : "A test command".to_string(),
    arguments : vec!
    [
      ArgumentDefinition
      {
        name : "arg1".to_string(),
        description : "A string argument".to_string(),
        kind : Kind::String,
        attributes : ArgumentAttributes {
          optional: false,
          multiple: false,
          interactive: false,
          sensitive: false,
          ..Default::default()
        },
        validation_rules : vec![],
        hint : "".to_string(),
        aliases : vec![],
        tags : vec![],
      },
      ArgumentDefinition
      {
        name : "arg2".to_string(),
        description : "An integer argument".to_string(),
        kind : Kind::Integer,
        attributes : ArgumentAttributes {
          optional: true,
          multiple: false,
          interactive: false,
          sensitive: false,
          ..Default::default()
        },
        validation_rules : vec![],
        hint : "".to_string(),
        aliases : vec![],
        tags : vec![],
      },
    ],
    routine_link : None,
    namespace : "".to_string(),
    hint : "".to_string(),
    status : "".to_string(),
    version : "".to_string(),
    tags : vec![],
    aliases : vec![],
    permissions : vec![],
    idempotent : false,
    deprecation_message : "".to_string(),
    examples : vec![],
    http_method_hint : "".to_string(),
  });

  let parser = Parser::new( UnilangParserOptions::default() );

  // T3.1
  let input = "test_cmd hello 123";
  let instruction = parser.parse_single_instruction( input ).unwrap();
  let instructions = &[ instruction ][ .. ];
  let analyzer = SemanticAnalyzer::new( instructions, &registry );
  let verified = analyzer.analyze().unwrap();
  assert_eq!( verified.len(), 1 );
  assert_eq!( verified[ 0 ].definition.name, "test_cmd" );
  assert_eq!(
    verified[ 0 ].arguments.get( "arg1" ).unwrap(),
    &Value::String( "hello".to_string() )
  );
  assert_eq!( verified[ 0 ].arguments.get( "arg2" ).unwrap(), &Value::Integer( 123 ) );

  // T3.2
  let input = "unknown_cmd";
  let instruction = parser.parse_single_instruction( input ).unwrap();
  let instructions = &[ instruction ][ .. ];
  let analyzer = SemanticAnalyzer::new( instructions, &registry );
  let error = analyzer.analyze().unwrap_err();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "COMMAND_NOT_FOUND" ) );

  // T3.3
  let input = "test_cmd";
  let instruction = parser.parse_single_instruction( input ).unwrap();
  let instructions = &[ instruction ][ .. ];
  let analyzer = SemanticAnalyzer::new( instructions, &registry );
  let error = analyzer.analyze().unwrap_err();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "MISSING_ARGUMENT" ) );

  // T3.4 - Updated to test a clear type mismatch for the second argument
  let input = "test_cmd hello not-an-integer";
  let instruction = parser.parse_single_instruction( input ).unwrap();
  let instructions = &[ instruction ][ .. ];
  let analyzer = SemanticAnalyzer::new( instructions, &registry );
  let error = analyzer.analyze().unwrap_err();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "INVALID_ARGUMENT_TYPE" ) );

  // T3.5
  let input = "test_cmd \"hello\" 123 456";
  let instruction = parser.parse_single_instruction( input ).unwrap();
  let instructions = &[ instruction ][ .. ];
  let analyzer = SemanticAnalyzer::new( instructions, &registry );
  let error = analyzer.analyze().unwrap_err();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "TOO_MANY_ARGUMENTS" ) );
}

///
/// Tests for the `Interpreter`.
///
/// This test covers the following combinations from the Test Matrix:
/// - T4.1: A single valid command.
/// - T4.2: Multiple valid commands.
///
#[ test ]
fn interpreter_tests()
{
  let mut registry = CommandRegistry::new();

  // Dummy routine for cmd1
  let cmd1_routine = Box::new(
    | _cmd : VerifiedCommand, _ctx : ExecutionContext | -> Result< OutputData, ErrorData >
    {
      Ok( OutputData
      {
        content : "cmd1 executed".to_string(),
        format : "text".to_string(),
      })
    },
  );
  registry
  .command_add_runtime
  (
    &CommandDefinition
    {
      name : "cmd1".to_string(),
      description : "".to_string(),
      arguments : vec![],
      routine_link : Some( "cmd1_routine_link".to_string() ),
      namespace : "".to_string(),
      hint : "".to_string(),
      status : "".to_string(),
      version : "".to_string(),
      tags : vec![],
      aliases : vec![],
      permissions : vec![],
      idempotent : false,
      deprecation_message : "".to_string(),
      examples : vec![],
      http_method_hint : "".to_string(),
    },
    cmd1_routine,
  )
  .unwrap();

  // Dummy routine for cmd2
  let cmd2_routine = Box::new(
    | _cmd : VerifiedCommand, _ctx : ExecutionContext | -> Result< OutputData, ErrorData >
    {
      Ok( OutputData
      {
        content : "cmd2 executed".to_string(),
        format : "text".to_string(),
      })
    },
  );
  registry
  .command_add_runtime
  (
    &CommandDefinition
    {
      name : "cmd2".to_string(),
      description : "".to_string(),
      arguments : vec![],
      routine_link : Some( "cmd2_routine_link".to_string() ),
      namespace : "".to_string(),
      hint : "".to_string(),
      status : "".to_string(),
      version : "".to_string(),
      tags : vec![],
      aliases : vec![],
      permissions : vec![],
      idempotent : false,
      deprecation_message : "".to_string(),
      examples : vec![],
      http_method_hint : "".to_string(),
    },
    cmd2_routine,
  )
  .unwrap();

  let parser = Parser::new( UnilangParserOptions::default() );

  // T4.1
  let input = "cmd1";
  let instruction = parser.parse_single_instruction( input ).unwrap();
  let instructions = &[ instruction ][ .. ];
  let analyzer = SemanticAnalyzer::new( instructions, &registry );
  let verified = analyzer.analyze().unwrap();
  let interpreter = Interpreter::new( &verified, &registry ); // Added registry
  let mut context = ExecutionContext::default();
  let result = interpreter.run( &mut context ).unwrap();
  assert_eq!( result.len(), 1 );
  assert_eq!( result[ 0 ].content, "cmd1 executed" );

  // T4.2
  let input_commands = vec![ "cmd1", "cmd2" ];
  let mut instructions_vec : Vec< GenericInstruction > = Vec::new();
  for cmd_str in input_commands
  {
    instructions_vec.push( parser.parse_single_instruction( cmd_str ).unwrap() );
  }
  let analyzer = SemanticAnalyzer::new( &instructions_vec, &registry );
  let verified = analyzer.analyze().unwrap();
  let interpreter = Interpreter::new( &verified, &registry ); // Added registry
  let mut context = ExecutionContext::default();
  let result = interpreter.run( &mut context ).unwrap();
  assert_eq!( result.len(), 2 );
  assert_eq!( result[ 0 ].content, "cmd1 executed" );
  assert_eq!( result[ 1 ].content, "cmd2 executed" );
}

///
/// Tests for the `HelpGenerator`.
///
/// This test covers the following combinations from the Test Matrix:
/// - T5.1: A command with arguments.
/// - T5.2: A command without arguments.
///
#[ test ]
fn help_generator_tests()
{
  let mut registry = CommandRegistry::new();
  let cmd_with_args_def = CommandDefinition
  {
    name : "test_cmd".to_string(),
    description : "A test command".to_string(),
    arguments : vec!
    [
      ArgumentDefinition
      {
        name : "arg1".to_string(),
        description : "A string argument".to_string(),
        kind : Kind::String,
        attributes : ArgumentAttributes {
          optional: false,
          multiple: false,
          interactive: false,
          sensitive: false,
          ..Default::default()
        },
        validation_rules : vec![],
        hint : "".to_string(),
        aliases : vec![],
        tags : vec![],
      }
    ],
    routine_link : None,
    namespace : "".to_string(),
    hint : "".to_string(),
    status : "".to_string(),
    version : "".to_string(),
    tags : vec![],
    aliases : vec![],
    permissions : vec![],
    idempotent : false,
    deprecation_message : "".to_string(),
    examples : vec![],
    http_method_hint : "".to_string(),
  };
  registry.register( cmd_with_args_def.clone() );

  let cmd_without_args_def = CommandDefinition
  {
    name : "simple_cmd".to_string(),
    description : "A simple command".to_string(),
    arguments : vec![],
    routine_link : None,
    namespace : "".to_string(),
    hint : "".to_string(),
    status : "".to_string(),
    version : "".to_string(),
    tags : vec![],
    aliases : vec![],
    permissions : vec![],
    idempotent : false,
    deprecation_message : "".to_string(),
    examples : vec![],
    http_method_hint : "".to_string(),
  };
  registry.register( cmd_without_args_def.clone() );

  let help_gen = HelpGenerator::new( &registry );

  // T5.1
  let help_text = help_gen.command( &cmd_with_args_def.name ).unwrap();
  assert!( help_text.contains( "Usage: test_cmd" ) );
  assert!( help_text.contains( "A test command" ) );
  assert!( help_text.contains( "Arguments:" ) );
  assert!( help_text.contains( "arg1" ) );

  // T5.2
  let help_text = help_gen.command( &cmd_without_args_def.name ).unwrap();
  assert!( help_text.contains( "Usage: simple_cmd" ) );
  assert!( help_text.contains( "A simple command" ) );
  assert!( !help_text.contains( "Arguments:" ) );
}
