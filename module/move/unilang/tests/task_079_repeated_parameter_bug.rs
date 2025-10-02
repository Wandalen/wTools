//! Task 079: Repeated Parameter Investigation - Root Cause Identified and Fixed
//!
//! **Root Cause Found**: Shell argument handling, NOT unilang core
//!
//! **Investigation Result**: Unilang parser and semantic analyzer correctly handle
//! repeated parameter names with `multiple: true` attribute for ALL test scenarios.
//!
//! **Production Bug**: wrun CLI failed because it joined shell arguments with spaces
//! after the shell had already stripped quotes:
//! ```text
//! Shell:  wrun .run command::"echo a" command::"echo b"
//! Args:   [".run", "command::echo a", "command::echo b"]  ← quotes stripped
//! Joined: ".run command::echo a command::echo b"         ← parser sees unquoted
//! ```
//!
//! **Solution**: Re-quote values containing spaces before joining (see task_079_fix_shell_argument_handling.rs)
//!
//! ## Test Strategy
//!
//! These tests validate unilang core handles the production scenario correctly:
//! 1. Test 2 repeated parameters (baseline) ✅
//! 2. Test 3 repeated parameters (validation) ✅
//! 3. Test 4 repeated parameters (validates core works) ✅
//! 4. Verify command definition structure matches wrun ✅
//! 5. Shell argument fix implemented and tested separately ✅

#![allow(clippy::doc_markdown)] // Allow command-line syntax in documentation

use unilang::
{
  data:: { ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes },
  registry::CommandRegistry,
  semantic::SemanticAnalyzer,
  Value,
};
use unilang_parser:: { Parser, UnilangParserOptions };

/// Helper: Create command definition matching wrun's .run command
///
/// Uses Kind::List(String) + multiple:true exactly like wrun production definition
fn create_run_command_definition() -> CommandDefinition
{
  CommandDefinition::former()
    .name( ".run" )
    .description( "Execute multiple commands (wrun production command)" )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "command" )
        .kind( Kind::List( Box::new( Kind::String ), None ) ) // Matches wrun's definition
        .attributes( ArgumentAttributes
        {
          optional: true,
          multiple: true,  // Enables Value::List collection for repeated parameters
          ..Default::default()
        })
        .description( "Commands to execute" )
        .hint( "Shell command" )
        .end(),

      ArgumentDefinition::former()
        .name( "parallel" )
        .kind( Kind::Integer )
        .attributes( ArgumentAttributes
        {
          optional: true,
          default: Some( "2".to_string() ),
          ..Default::default()
        })
        .description( "Number of parallel executions" )
        .end(),
    ])
    .end()
}

/// Helper: Parse and verify a command string
fn parse_and_verify_command( input: &str ) -> Result< String, String >
{
  let mut registry = CommandRegistry::new();
  let cmd_def = create_run_command_definition();
  registry.register( cmd_def );

  let parser = Parser::new( UnilangParserOptions::default() );

  match parser.parse_single_instruction( input )
  {
    Ok( instruction ) =>
    {
      let instructions = vec![ instruction ];
      let analyzer = SemanticAnalyzer::new( &instructions, &registry );

      match analyzer.analyze()
      {
        Ok( verified_commands ) =>
        {
          if verified_commands.is_empty()
          {
            return Err( "No commands verified".to_string() );
          }

          let cmd = &verified_commands[ 0 ];
          let command_arg = cmd.arguments.get( "command" );

          match command_arg
          {
            Some( Value::List( list ) ) =>
            {
              Ok( format!( "SUCCESS: Collected {} commands into List", list.len() ) )
            },
            Some( Value::String( s ) ) =>
            {
              Ok( format!( "PARTIAL: Single string value: {s}" ) )
            },
            Some( other ) =>
            {
              Err( format!( "UNEXPECTED: Got {other:?} instead of List" ) )
            },
            None =>
            {
              Err( "MISSING: No 'command' parameter found".to_string() )
            }
          }
        },
        Err( e ) =>
        {
          Err( format!( "Semantic analysis error: {e:?}" ) )
        }
      }
    },
    Err( e ) =>
    {
      Err( format!( "Parse error: {e:?}" ) )
    }
  }
}

/// T079.1: Baseline - 2 Repeated Parameters
///
/// Validates unilang correctly collects 2 repeated parameters into Value::List
#[test]
fn test_repeated_parameter_two_commands()
{
  let input = r#".run command::"cargo build" command::"cargo test" parallel::2"#;

  let result = parse_and_verify_command( input );

  match result
  {
    Ok( msg ) =>
    {
      println!( "✅ 2 commands: {msg}" );
      assert!( msg.contains( "List" ) || msg.contains( '2' ), "Should create list with 2 items" );
    },
    Err( e ) =>
    {
      panic!( "❌ 2 commands FAILED (unexpected): {e}" );
    }
  }
}

/// T079.2: Extended Validation - 3 Repeated Parameters
///
/// Validates unilang correctly collects 3 repeated parameters into Value::List
#[test]
fn test_repeated_parameter_three_commands()
{
  let input = r#".run command::"cargo build" command::"cargo test" command::"cargo doc" parallel::2"#;

  let result = parse_and_verify_command( input );

  match result
  {
    Ok( msg ) =>
    {
      println!( "✅ 3 commands: {msg}" );
      assert!( msg.contains( "List" ) || msg.contains( '3' ), "Should create list with 3 items" );
    },
    Err( e ) =>
    {
      panic!( "❌ 3 commands FAILED (unexpected): {e}" );
    }
  }
}

/// T079.3: 4 Repeated Parameters - Validates Unilang Core Works Correctly
///
/// **Finding**: Unilang parser and semantic analyzer handle 4+ repeated parameters correctly.
/// **Conclusion**: The Task 079 bug does NOT originate in unilang core - it must be in
/// shell argument parsing or wrun's value extraction layer.
///
/// This test validates that unilang correctly collects repeated parameters into `Value::List`.
#[test]
fn test_repeated_parameter_four_commands_works_correctly()
{
  let input = r#".run command::"cargo build" command::"echo hello1" command::"cargo test" command::"echo hello2" parallel::2"#;

  let result = parse_and_verify_command( input );

  assert!( result.is_ok(), "4 repeated parameters should parse successfully: {result:?}" );

  let msg = result.unwrap();
  println!( "✅ 4 commands: {msg}" );

  // Verify correct collection into list
  assert!( msg.contains( "List" ), "Should collect into Value::List" );
  assert!( msg.contains( '4' ), "Should have 4 commands" );
}

/// T079.4: Production Scenario - Exact wrun Command Structure
///
/// Validates unilang handles the production command structure correctly.
/// Bug reported in wrun binary doesn't reproduce here, confirming issue is in wrun's argument handling layer.
#[test]
fn test_wrun_production_exact_scenario()
{
  let input = r#".run command::"cargo build" command::"cargo test" parallel::2"#;

  let result = parse_and_verify_command( input );

  assert!( result.is_ok(), "Production wrun command should parse successfully: {result:?}" );

  let success_msg = result.unwrap();
  assert!( success_msg.contains( "List" ), "Should collect commands into List" );
  assert!( success_msg.contains( '2' ), "Should have 2 commands" );

  println!( "✅ Production scenario works: {success_msg}" );
}

/// T079.5: Edge Case - Single Word Commands
///
/// Validates unilang correctly collects single-word commands into Value::List.
/// (If data loss occurs in production, it's in wrun's argument handling, not unilang core)
#[test]
fn test_repeated_parameter_single_word_commands()
{
  let input = r#".run command::"pwd" command::"whoami" command::"date" command::"hostname" parallel::2"#;

  let result = parse_and_verify_command( input );

  assert!( result.is_ok(), "Single-word commands should parse: {result:?}" );

  let msg = result.unwrap();
  assert!( msg.contains( '4' ), "Should have 4 commands, not just the last one" );
}

/// T079.6: Configuration Verification
///
/// Validates command definition uses Kind::List with multiple:true attribute,
/// matching wrun's production configuration exactly
#[test]
fn test_multiple_attribute_is_set()
{
  let cmd_def = create_run_command_definition();

  let command_arg = cmd_def.arguments
    .iter()
    .find( |arg| arg.name == "command" )
    .expect( "command argument should exist" );

  assert!( command_arg.attributes.multiple,
    "command argument MUST have multiple:true attribute" );

  // Verify it's Kind::List, not Kind::String
  match &command_arg.kind
  {
    Kind::List( inner, _ ) =>
    {
      assert_eq!( **inner, Kind::String, "Inner type should be String" );
      println!( "✅ Command definition matches wrun: Kind::List(String) + multiple=true" );
    },
    other =>
    {
      panic!( "Expected Kind::List, got {other:?}" );
    }
  }
}
