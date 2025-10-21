//! Tests for unknown/unrecognized parameter detection
//!
//! This test file verifies that unilang properly rejects commands with unknown parameters
//! instead of silently ignoring them.
//!
//! Bug reproduction: Currently unilang silently ignores named parameters that don't match
//! any defined argument in the command definition.

#![ allow( deprecated ) ]

use unilang::
{
  data::{ ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes },
  registry::CommandRegistry,
  semantic::SemanticAnalyzer,
};
use unilang_parser::{ Parser, UnilangParserOptions };

/// Helper to create a simple test command with one parameter
fn create_test_command() -> CommandDefinition
{
  CommandDefinition::former()
    .name( ".test" )
    .description( "Test command" )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "dry" )
        .kind( Kind::Boolean )
        .attributes( ArgumentAttributes
        {
          optional: true,
          default: Some( "false".to_string() ),
          ..Default::default()
        })
        .description( "Dry run flag" )
        .end(),
    ])
    .end()
}

/// TEST: Unknown named parameter should cause error with suggestion
///
/// Tests that unilang detects typos and provides helpful "Did you mean..." suggestions
#[test]
fn test_unknown_named_parameter_with_suggestion()
{
  let mut registry = CommandRegistry::new();
  registry.register( create_test_command() );

  let parser = Parser::new( UnilangParserOptions::default() );

  // User types: .test drry::1
  // Note the typo: "drry" instead of "dry"
  let instruction_text = ".test drry::1";
  let instruction = parser.parse_single_instruction( instruction_text )
    .expect( "Parser should succeed" );

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();

  assert!( result.is_err(), "Unknown parameter should cause error" );

  let error = result.unwrap_err();
  let error_msg = format!( "{error:?}" );

  // Should mention the unknown parameter
  assert!(
    error_msg.contains( "drry" ),
    "Error should mention unknown parameter 'drry', got: {error_msg}"
  );

  // Should provide suggestion
  assert!(
    error_msg.contains( "Did you mean 'dry'" ),
    "Error should suggest 'dry', got: {error_msg}"
  );
}

/// TEST: Multiple unknown parameters should cause error listing all of them
#[test]
fn test_multiple_unknown_parameters()
{
  let mut registry = CommandRegistry::new();
  registry.register( create_test_command() );

  let parser = Parser::new( UnilangParserOptions::default() );

  // User provides multiple unknown parameters
  let instruction_text = ".test drry::1 verbose::1 foo::bar";
  let instruction = parser.parse_single_instruction( instruction_text )
    .expect( "Parser should succeed" );

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();

  assert!( result.is_err(), "Multiple unknown parameters should cause error" );

  let error = result.unwrap_err();
  let error_msg = format!( "{error:?}" );

  // Should mention unknown parameters (plural)
  assert!(
    error_msg.contains( "Unknown parameter" ),
    "Error should mention unknown parameters, got: {error_msg}"
  );

  // Should list the parameters
  assert!(
    error_msg.contains( "drry" ) && error_msg.contains( "verbose" ) && error_msg.contains( "foo" ),
    "Error should list all unknown parameters, got: {error_msg}"
  );
}

/// TEST: Mix of valid and unknown parameters should fail
#[test]
fn test_mix_valid_and_unknown()
{
  let mut registry = CommandRegistry::new();
  registry.register( create_test_command() );

  let parser = Parser::new( UnilangParserOptions::default() );

  // Mix valid parameter "dry" with unknown "drry"
  let instruction_text = ".test dry::1 drry::1";
  let instruction = parser.parse_single_instruction( instruction_text )
    .expect( "Parser should succeed" );

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();

  assert!( result.is_err(), "Mix of valid and unknown parameters should cause error" );

  let error = result.unwrap_err();
  let error_msg = format!( "{error:?}" );

  // Should detect the unknown parameter even when valid one is present
  assert!(
    error_msg.contains( "drry" ),
    "Error should mention unknown parameter 'drry', got: {error_msg}"
  );
}

/// TEST: Command with no parameters should reject ANY named parameter
#[test]
fn test_no_params_command_rejects_any_named()
{
  let cmd = CommandDefinition::former()
    .name( ".simple" )
    .description( "Command with no parameters" )
    .arguments( vec![] )
    .end();

  let mut registry = CommandRegistry::new();
  registry.register( cmd );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction_text = ".simple foo::bar";
  let instruction = parser.parse_single_instruction( instruction_text )
    .expect( "Parser should succeed" );

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();

  assert!( result.is_err(), "Command with no parameters should reject any named parameter" );

  let error = result.unwrap_err();
  let error_msg = format!( "{error:?}" );

  assert!(
    error_msg.contains( "foo" ),
    "Error should mention unknown parameter 'foo', got: {error_msg}"
  );
}

/// CONTROL TEST: Valid parameter should succeed
/// This test should PASS even before the fix
#[test]
fn test_valid_parameter_succeeds()
{
  let mut registry = CommandRegistry::new();
  registry.register( create_test_command() );

  let parser = Parser::new( UnilangParserOptions::default() );

  // Correct parameter name "dry"
  let instruction_text = ".test dry::1";
  let instruction = parser.parse_single_instruction( instruction_text )
    .expect( "Parser should succeed" );

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_ok(), "Valid parameter should succeed" );
}
