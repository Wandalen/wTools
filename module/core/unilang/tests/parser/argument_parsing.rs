//! Argument Parsing Unit Tests
//!
//! ## Scope
//! Tests the parser's ability to extract and process command arguments from input strings.
//! This covers the fundamental parsing logic that converts text input into structured argument data.
//!
//! ## Coverage
//! - Basic argument syntax parsing (`name::value`)
//! - Positional vs named argument handling
//! - Complex argument structures
//! - Error conditions and malformed input
//! - Performance characteristics
//!
//! ## Related
//! - `unit/parser/quoted_values.rs` - Quoted string handling
//! - `unit/semantic/argument_binding.rs` - Argument to definition binding

use unilang_parser::{ Parser, UnilangParserOptions };

#[test]
fn test_basic_named_argument_parsing()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = r#".test name::"value""#;

  let instruction = parser.parse_single_instruction( input ).expect( "Should parse successfully" );

  assert_eq!( instruction.command_path_slices, vec![ "test" ] );
  assert_eq!( instruction.named_arguments.len(), 1 );

  let name_args = instruction.named_arguments.get( "name" ).expect( "name argument should exist" );
  assert_eq!( name_args.len(), 1 );
  assert_eq!( name_args[0].value, "value" );
}

#[test]
fn test_multiple_named_arguments()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = r#".test first::"value1" second::"value2" third::"value3""#;

  let instruction = parser.parse_single_instruction( input ).expect( "Should parse successfully" );

  assert_eq!( instruction.named_arguments.len(), 3 );

  let first_args = instruction.named_arguments.get( "first" ).unwrap();
  assert_eq!( first_args[0].value, "value1" );

  let second_args = instruction.named_arguments.get( "second" ).unwrap();
  assert_eq!( second_args[0].value, "value2" );

  let third_args = instruction.named_arguments.get( "third" ).unwrap();
  assert_eq!( third_args[0].value, "value3" );
}

#[test]
fn test_positional_argument_parsing()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = r#".test "arg1" "arg2" "arg3""#;

  let instruction = parser.parse_single_instruction( input ).expect( "Should parse successfully" );

  assert_eq!( instruction.positional_arguments.len(), 3 );
  assert_eq!( instruction.positional_arguments[0].value, "arg1" );
  assert_eq!( instruction.positional_arguments[1].value, "arg2" );
  assert_eq!( instruction.positional_arguments[2].value, "arg3" );
}

#[test]
fn test_mixed_positional_and_named_arguments()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = r#".test "pos1" name::"named_value" "pos2""#;

  let instruction = parser.parse_single_instruction( input ).expect( "Should parse successfully" );

  assert_eq!( instruction.positional_arguments.len(), 2 );
  assert_eq!( instruction.positional_arguments[0].value, "pos1" );
  assert_eq!( instruction.positional_arguments[1].value, "pos2" );

  assert_eq!( instruction.named_arguments.len(), 1 );
  let name_args = instruction.named_arguments.get( "name" ).unwrap();
  assert_eq!( name_args[0].value, "named_value" );
}

#[test]
fn test_same_name_multiple_arguments()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = r#".test param::"value1" param::"value2" param::"value3""#;

  let instruction = parser.parse_single_instruction( input ).expect( "Should parse successfully" );

  assert_eq!( instruction.named_arguments.len(), 1 );
  let param_args = instruction.named_arguments.get( "param" ).expect( "param should exist" );
  assert_eq!( param_args.len(), 3 );
  assert_eq!( param_args[0].value, "value1" );
  assert_eq!( param_args[1].value, "value2" );
  assert_eq!( param_args[2].value, "value3" );
}

#[test]
fn test_command_path_parsing()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Test simple command
  let instruction1 = parser.parse_single_instruction( ".test" ).expect( "Should parse" );
  assert_eq!( instruction1.command_path_slices, vec![ "test" ] );

  // Test namespaced command
  let instruction2 = parser.parse_single_instruction( ".video.search" ).expect( "Should parse" );
  assert_eq!( instruction2.command_path_slices, vec![ "video", "search" ] );

  // Test deeply nested command
  let instruction3 = parser.parse_single_instruction( ".level1.level2.level3.command" ).expect( "Should parse" );
  assert_eq!( instruction3.command_path_slices, vec![ "level1", "level2", "level3", "command" ] );
}

#[test]
fn test_help_operator_parsing()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = ".test ?";

  let instruction = parser.parse_single_instruction( input ).expect( "Should parse successfully" );

  assert_eq!( instruction.command_path_slices, vec![ "test" ] );
  assert!( instruction.help_requested, "Help should be requested" );
}

#[test]
fn test_empty_command_parsing()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = ".";

  let instruction = parser.parse_single_instruction( input ).expect( "Should parse successfully" );

  assert!( instruction.command_path_slices.is_empty(), "Command path should be empty for dot command" );
}

#[test]
fn test_argument_without_quotes()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = ".test param::value";

  let instruction = parser.parse_single_instruction( input ).expect( "Should parse successfully" );

  let param_args = instruction.named_arguments.get( "param" ).expect( "param should exist" );
  assert_eq!( param_args[0].value, "value" );
}

#[test]
fn test_complex_values_with_special_characters()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = r#".test url::"https://example.com/path?param=value&other=data" regex::"[a-zA-Z0-9]+""#;

  let instruction = parser.parse_single_instruction( input ).expect( "Should parse successfully" );

  let url_args = instruction.named_arguments.get( "url" ).expect( "url should exist" );
  assert_eq!( url_args[0].value, "https://example.com/path?param=value&other=data" );

  let regex_args = instruction.named_arguments.get( "regex" ).expect( "regex should exist" );
  assert_eq!( regex_args[0].value, "[a-zA-Z0-9]+" );
}

#[test]
fn test_performance_with_large_input()
{
  use std::time::Instant;

  let parser = Parser::new( UnilangParserOptions::default() );

  // Create large input with many arguments
  let mut input_parts = vec![ ".test".to_string() ];
  for i in 1..=100 {
    input_parts.push( format!( r#"param{i}::"value{i}""# ) );
  }
  let input = input_parts.join( " " );

  let start = Instant::now();
  let instruction = parser.parse_single_instruction( &input ).expect( "Should parse successfully" );
  let duration = start.elapsed();

  // Performance check - 100 arguments should parse within reasonable time (debug build allowance)
  assert!( duration.as_millis() < 400, "Parsing 100 arguments took too long: {duration:?}" );

  // Correctness check
  assert_eq!( instruction.named_arguments.len(), 100 );
  let param1_args = instruction.named_arguments.get( "param1" ).expect( "param1 should exist" );
  assert_eq!( param1_args[0].value, "value1" );
}

#[test]
fn test_malformed_input_handling()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Test various malformed inputs that should fail gracefully
  let malformed_inputs = vec![
    "invalid_without_dot",
    ".test param:", // Missing value
    ".test :", // Missing parameter name and value
    ".test param:::", // Too many colons
  ];

  for input in malformed_inputs {
    let result = parser.parse_single_instruction( input );
    // The exact behavior depends on parser implementation
    // We just ensure it doesn't panic and handles errors gracefully
    match result {
      Ok( _instruction ) => {
        // Some malformed inputs might still parse with lenient parsing
      },
      Err( _error ) => {
        // Expected for truly malformed input
      }
    }
  }
}