//!
//! Integration tests for the full Phase 1 pipeline.
//!

use unilang::data::{ ArgumentDefinition, CommandDefinition };
use unilang::parsing::{ Lexer, Parser, Token };
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang::interpreter::{ Interpreter, ExecutionContext };

///
/// Tests for the `Lexer`.
///
// Test Matrix Rows: T1.1, T1.2, T1.3, T1.4
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
// Test Matrix Rows: T2.1, T2.2, T2.3
#[test]
fn parser_tests()
{
  // T2.1
  let input = "command \"arg1\"";
  let lexer = Lexer::new( input );
  let mut parser = Parser::new( lexer );
  let program = parser.parse_program();
  assert_eq!( program.statements.len(), 1 );
  assert_eq!( program.statements[ 0 ].command, "command" );
  assert_eq!( program.statements[ 0 ].args, vec![ Token::String( "arg1".to_string() ) ] );

  // T2.2
  let input = "cmd1 1 ;; cmd2 2";
  let lexer = Lexer::new( input );
  let mut parser = Parser::new( lexer );
  let program = parser.parse_program();
  assert_eq!( program.statements.len(), 2 );
  assert_eq!( program.statements[ 0 ].command, "cmd1" );
  assert_eq!( program.statements[ 0 ].args, vec![ Token::Integer( 1 ) ] );
  assert_eq!( program.statements[ 1 ].command, "cmd2" );
  assert_eq!( program.statements[ 1 ].args, vec![ Token::Integer( 2 ) ] );

  // T2.3
  let input = "";
  let lexer = Lexer::new( input );
  let mut parser = Parser::new( lexer );
  let program = parser.parse_program();
  assert_eq!( program.statements.len(), 0 );
}

///
/// Tests for the `SemanticAnalyzer`.
///
// Test Matrix Rows: T3.1, T3.2, T3.3, T3.4, T3.5
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
        kind : "String".to_string(),
        optional : false,
      },
      ArgumentDefinition {
        name : "arg2".to_string(),
        description : "An integer argument".to_string(),
        kind : "Integer".to_string(),
        optional : true,
      },
    ],
  } );

  // T3.1
  let input = "test_cmd \"hello\" 123";
  let lexer = Lexer::new( input );
  let mut parser = Parser::new( lexer );
  let program = parser.parse_program();
  let analyzer = SemanticAnalyzer::new( &program, &registry );
  let verified = analyzer.analyze().unwrap();
  assert_eq!( verified.len(), 1 );
  assert_eq!( verified[ 0 ].definition.name, "test_cmd" );
  assert_eq!( verified[ 0 ].arguments.get( "arg1" ).unwrap(), &Token::String( "hello".to_string() ) );
  assert_eq!( verified[ 0 ].arguments.get( "arg2" ).unwrap(), &Token::Integer( 123 ) );

  // T3.2
  let input = "unknown_cmd";
  let lexer = Lexer::new( input );
  let mut parser = Parser::new( lexer );
  let program = parser.parse_program();
  let analyzer = SemanticAnalyzer::new( &program, &registry );
  let error = analyzer.analyze().unwrap_err();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "COMMAND_NOT_FOUND" ) );

  // T3.3
  let input = "test_cmd";
  let lexer = Lexer::new( input );
  let mut parser = Parser::new( lexer );
  let program = parser.parse_program();
  let analyzer = SemanticAnalyzer::new( &program, &registry );
  let error = analyzer.analyze().unwrap_err();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "MISSING_ARGUMENT" ) );

  // T3.4
  let input = "test_cmd 123";
  let lexer = Lexer::new( input );
  let mut parser = Parser::new( lexer );
  let program = parser.parse_program();
  let analyzer = SemanticAnalyzer::new( &program, &registry );
  let error = analyzer.analyze().unwrap_err();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "INVALID_ARGUMENT_TYPE" ) );

  // T3.5
  let input = "test_cmd \"hello\" 123 456";
  let lexer = Lexer::new( input );
  let mut parser = Parser::new( lexer );
  let program = parser.parse_program();
  let analyzer = SemanticAnalyzer::new( &program, &registry );
  let error = analyzer.analyze().unwrap_err();
  assert!( matches!( error, unilang::error::Error::Execution( data ) if data.code == "TOO_MANY_ARGUMENTS" ) );
}

///
/// Tests for the `Interpreter`.
///
// Test Matrix Rows: T4.1, T4.2
#[test]
fn interpreter_tests()
{
  let mut registry = CommandRegistry::new();
  registry.register( CommandDefinition {
    name : "cmd1".to_string(),
    description : "".to_string(),
    arguments : vec![],
  } );
  registry.register( CommandDefinition {
    name : "cmd2".to_string(),
    description : "".to_string(),
    arguments : vec![],
  } );

  // T4.1
  let input = "cmd1";
  let lexer = Lexer::new( input );
  let mut parser = Parser::new( lexer );
  let program = parser.parse_program();
  let analyzer = SemanticAnalyzer::new( &program, &registry );
  let verified = analyzer.analyze().unwrap();
  let interpreter = Interpreter::new( &verified );
  let mut context = ExecutionContext::default();
  let result = interpreter.run( &mut context ).unwrap();
  assert_eq!( result.len(), 1 );

  // T4.2
  let input = "cmd1 ;; cmd2";
  let lexer = Lexer::new( input );
  let mut parser = Parser::new( lexer );
  let program = parser.parse_program();
  let analyzer = SemanticAnalyzer::new( &program, &registry );
  let verified = analyzer.analyze().unwrap();
  let interpreter = Interpreter::new( &verified );
  let mut context = ExecutionContext::default();
  let result = interpreter.run( &mut context ).unwrap();
  assert_eq!( result.len(), 2 );
}

///
/// Tests for the `HelpGenerator`.
///
// Test Matrix Rows: T5.1, T5.2
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
      kind : "String".to_string(),
      optional : false,
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