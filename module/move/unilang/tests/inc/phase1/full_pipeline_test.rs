//!
//! Integration tests for the full Phase 1 pipeline.
//!

use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind, OutputData, ErrorData }; // Corrected import for ErrorData
use unilang::parsing::{ Lexer, Parser, Token };
use unilang::registry::CommandRegistry;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::interpreter::{ Interpreter, ExecutionContext };
use unilang::types::Value;

///
/// Tests for the `Lexer`.
///
/// This test covers the following combinations from the Test Matrix:
/// - T1.1: A command with various argument types.
/// - T1.2: Multiple commands separated by `;;`.
/// - T1.3: Whitespace handling.
/// - T1.4: Empty string literals.
///
#[test]
fn lexer_tests()
{
  // T1.1
  let input = "command \"arg1\" 123 1.23 true";
  let mut lexer = Lexer::new( input );
  assert_eq!( lexer.next_token(), Token::Identifier( "command".to_string() ) );
  assert_eq!( lexer.next_token(), Token::String( "arg1".to_string() ) );
  assert_eq!( lexer.next_token(), Token::Integer( 123 ) );
  assert_eq!( lexer.next_token(), Token::Float( 1.23 ) );
  assert_eq!( lexer.next_token(), Token::Boolean( true ) );
  assert_eq!( lexer.next_token(), Token::Eof );

  // T1.2
  let input = "cmd1 ;; cmd2";
  let mut lexer = Lexer::new( input );
  assert_eq!( lexer.next_token(), Token::Identifier( "cmd1".to_string() ) );
  assert_eq!( lexer.next_token(), Token::CommandSeparator );
  assert_eq!( lexer.next_token(), Token::Identifier( "cmd2".to_string() ) );
  assert_eq!( lexer.next_token(), Token::Eof );

  // T1.3
  let input = "   ";
  let mut lexer = Lexer::new( input );
  assert_eq!( lexer.next_token(), Token::Eof );

  // T1.4
  let input = "\"\"";
  let mut lexer = Lexer::new( input );
  assert_eq!( lexer.next_token(), Token::String( "".to_string() ) );
  assert_eq!( lexer.next_token(), Token::Eof );
}

///
/// Tests for the `Parser`.
///
/// This test covers the following combinations from the Test Matrix:
/// - T2.1: A single command with one argument.
/// - T2.2: Multiple commands with arguments.
/// - T2.3: Empty input.
///
#[test]
fn parser_tests()
{
  // T2.1
  let input = "command \"arg1\"";
  let mut parser = Parser::new( input );
  let program = parser.parse();
  assert_eq!( program.statements.len(), 1 );
  assert_eq!( program.statements[ 0 ].command, "command" );
  assert_eq!( program.statements[ 0 ].args, vec![ Token::String( "arg1".to_string() ) ] );

  // T2.2
  let input = "cmd1 1 ;; cmd2 2";
  let mut parser = Parser::new( input );
  let program = parser.parse();
  assert_eq!( program.statements.len(), 2 );
  assert_eq!( program.statements[ 0 ].command, "cmd1" );
  assert_eq!( program.statements[ 0 ].args, vec![ Token::Integer( 1 ) ] );
  assert_eq!( program.statements[ 1 ].command, "cmd2" );
  assert_eq!( program.statements[ 1 ].args, vec![ Token::Integer( 2 ) ] );

  // T2.3
  let input = "";
  let mut parser = Parser::new( input );
  let program = parser.parse();
  assert_eq!( program.statements.len(), 0 );
}

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
#[test]
fn semantic_analyzer_tests()
{
  let mut registry = CommandRegistry::new();
  registry.register( CommandDefinition {
    name : "test_cmd".to_string(),
    description : "A test command".to_string(),
    arguments : vec![
      ArgumentDefinition {
        name : "arg1".to_string(),
        description : "A string argument".to_string(),
        kind : Kind::String,
        optional : false,
        multiple : false, // Added
        validation_rules : vec![], // Added
      },
      ArgumentDefinition {
        name : "arg2".to_string(),
        description : "An integer argument".to_string(),
        kind : Kind::Integer,
        optional : true,
        multiple : false, // Added
        validation_rules : vec![], // Added
      },
    ],
  } );

  // T3.1
  let input = "test_cmd hello 123";
  let program = Parser::new( input ).parse();
  let analyzer = SemanticAnalyzer::new( &program, &registry );
  let verified = analyzer.analyze().unwrap();
  assert_eq!( verified.len(), 1 );
  assert_eq!( verified[ 0 ].definition.name, "test_cmd" );
  assert_eq!( verified[ 0 ].arguments.get( "arg1" ).unwrap(), &Value::String( "hello".to_string() ) );
  assert_eq!( verified[ 0 ].arguments.get( "arg2" ).unwrap(), &Value::Integer( 123 ) );

  // T3.2
  let input = "unknown_cmd";
  let program = Parser::new( input ).parse();
  let analyzer = SemanticAnalyzer::new( &program, &registry );
  let error = analyzer.analyze().unwrap_err();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "COMMAND_NOT_FOUND" ) );

  // T3.3
  let input = "test_cmd";
  let program = Parser::new( input ).parse();
  let analyzer = SemanticAnalyzer::new( &program, &registry );
  let error = analyzer.analyze().unwrap_err();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "MISSING_ARGUMENT" ) );

  // T3.4 - Updated to test a clear type mismatch for the second argument
  let input = "test_cmd hello not-an-integer";
  let program = Parser::new( input ).parse();
  let analyzer = SemanticAnalyzer::new( &program, &registry );
  let error = analyzer.analyze().unwrap_err();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "INVALID_ARGUMENT_TYPE" ) );

  // T3.5
  let input = "test_cmd \"hello\" 123 456";
  let program = Parser::new( input ).parse();
  let analyzer = SemanticAnalyzer::new( &program, &registry );
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
#[test]
fn interpreter_tests()
{
  let mut registry = CommandRegistry::new();
  
  // Dummy routine for cmd1
  let cmd1_routine = Box::new( | _cmd: VerifiedCommand, _ctx: ExecutionContext | -> Result<OutputData, ErrorData> {
    Ok( OutputData { content: "cmd1 executed".to_string(), format: "text".to_string() } )
  });
  registry.command_add_runtime( CommandDefinition {
    name : "cmd1".to_string(),
    description : "".to_string(),
    arguments : vec![],
  }, cmd1_routine ).unwrap();

  // Dummy routine for cmd2
  let cmd2_routine = Box::new( | _cmd: VerifiedCommand, _ctx: ExecutionContext | -> Result<OutputData, ErrorData> {
    Ok( OutputData { content: "cmd2 executed".to_string(), format: "text".to_string() } )
  });
  registry.command_add_runtime( CommandDefinition {
    name : "cmd2".to_string(),
    description : "".to_string(),
    arguments : vec![],
  }, cmd2_routine ).unwrap();

  // T4.1
  let input = "cmd1";
  let program = Parser::new( input ).parse();
  let analyzer = SemanticAnalyzer::new( &program, &registry );
  let verified = analyzer.analyze().unwrap();
  let interpreter = Interpreter::new( &verified, &registry ); // Added registry
  let mut context = ExecutionContext::default();
  let result = interpreter.run( &mut context ).unwrap();
  assert_eq!( result.len(), 1 );
  assert_eq!( result[0].content, "cmd1 executed" );

  // T4.2
  let input = "cmd1 ;; cmd2";
  let program = Parser::new( input ).parse();
  let analyzer = SemanticAnalyzer::new( &program, &registry );
  let verified = analyzer.analyze().unwrap();
  let interpreter = Interpreter::new( &verified, &registry ); // Added registry
  let mut context = ExecutionContext::default();
  let result = interpreter.run( &mut context ).unwrap();
  assert_eq!( result.len(), 2 );
  assert_eq!( result[0].content, "cmd1 executed" );
  assert_eq!( result[1].content, "cmd2 executed" );
}

///
/// Tests for the `HelpGenerator`.
///
/// This test covers the following combinations from the Test Matrix:
/// - T5.1: A command with arguments.
/// - T5.2: A command without arguments.
///
#[test]
fn help_generator_tests()
{
  let help_gen = unilang::help::HelpGenerator::new();

  // T5.1
  let cmd_with_args = CommandDefinition {
    name : "test_cmd".to_string(),
    description : "A test command".to_string(),
    arguments : vec![ ArgumentDefinition {
      name : "arg1".to_string(),
      description : "A string argument".to_string(),
      kind : Kind::String,
      optional : false,
      multiple : false, // Added
      validation_rules : vec![], // Added
    } ],
  };
  let help_text = help_gen.command( &cmd_with_args );
  assert!( help_text.contains( "Usage: test_cmd" ) );
  assert!( help_text.contains( "A test command" ) );
  assert!( help_text.contains( "Arguments:" ) );
  assert!( help_text.contains( "arg1" ) );

  // T5.2
  let cmd_without_args = CommandDefinition {
    name : "simple_cmd".to_string(),
    description : "A simple command".to_string(),
    arguments : vec![],
  };
  let help_text = help_gen.command( &cmd_without_args );
  assert!( help_text.contains( "Usage: simple_cmd" ) );
  assert!( help_text.contains( "A simple command" ) );
  assert!( !help_text.contains( "Arguments:" ) );
}