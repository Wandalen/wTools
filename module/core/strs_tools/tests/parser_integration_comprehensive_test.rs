//! Comprehensive test suite for parser integration functionality
//!
//! Tests all parser integration features including single-pass parsing,
//! command-line parsing, validation, and error handling scenarios.

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
use strs_tools::string::parser::*;

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ test ]
fn test_single_pass_integer_parsing()
{
  // Test parsing integers while splitting
  let input = "10,20,30,40,50";
  let results: Result< Vec< i32 >, _ > = input
    .split_and_parse( &[ "," ], |token| {
      token.parse().map_err( |_| ParseError::InvalidToken {
        token: token.to_string(),
        position: 0,
        expected: "integer".to_string(),
      } )
    } )
    .collect();

  assert!( results.is_ok() );
  let numbers = results.unwrap();
  assert_eq!( numbers, vec![ 10, 20, 30, 40, 50 ] );
}

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ test ]
fn test_single_pass_parsing_with_errors()
{
  // Test parsing with some invalid tokens
  let input = "10,invalid,30,bad,50";
  let results: Vec< _ > = input
    .split_and_parse( &[ "," ], |token| {
      token.parse::< i32 >().map_err( |_| ParseError::InvalidToken {
        token: token.to_string(),
        position: 0,
        expected: "integer".to_string(),
      } )
    } )
    .collect();

  // Should have 5 results total
  assert_eq!( results.len(), 5 );
  
  // First, third, and fifth should be successful
  assert!( results[ 0 ].is_ok() );
  assert!( results[ 2 ].is_ok() );
  assert!( results[ 4 ].is_ok() );
  
  // Second and fourth should be errors
  assert!( results[ 1 ].is_err() );
  assert!( results[ 3 ].is_err() );
  
  // Verify successful values
  assert_eq!( results[ 0 ].as_ref().unwrap(), &10 );
  assert_eq!( results[ 2 ].as_ref().unwrap(), &30 );
  assert_eq!( results[ 4 ].as_ref().unwrap(), &50 );
}

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ test ]
fn test_command_line_parsing_comprehensive()
{
  let input = "myapp --verbose --output:result.txt input1.txt input2.txt --debug";
  let results: Result< Vec< _ >, _ > = input.parse_command_line().collect();
  
  assert!( results.is_ok() );
  let tokens = results.unwrap();
  
  assert_eq!( tokens.len(), 6 );
  
  // Verify each token type
  assert!( matches!( tokens[ 0 ], ParsedToken::Command( "myapp" ) ) );
  assert!( matches!( tokens[ 1 ], ParsedToken::Flag( "verbose" ) ) );
  assert!( matches!( tokens[ 2 ], ParsedToken::KeyValue { key: "output", value: "result.txt" } ) );
  assert!( matches!( tokens[ 3 ], ParsedToken::Positional( "input1.txt" ) ) );
  assert!( matches!( tokens[ 4 ], ParsedToken::Positional( "input2.txt" ) ) );
  assert!( matches!( tokens[ 5 ], ParsedToken::Flag( "debug" ) ) );
}

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ test ]
fn test_command_line_parsing_with_spaces_and_tabs()
{
  let input = "cmd\t--flag1\t\targ1   --key:value  \t arg2";
  let results: Result< Vec< _ >, _ > = input.parse_command_line().collect();
  
  assert!( results.is_ok() );
  let tokens = results.unwrap();
  
  // Should handle multiple spaces and tabs correctly
  assert_eq!( tokens.len(), 5 );
  assert!( matches!( tokens[ 0 ], ParsedToken::Command( "cmd" ) ) );
  assert!( matches!( tokens[ 1 ], ParsedToken::Flag( "flag1" ) ) );
  assert!( matches!( tokens[ 2 ], ParsedToken::Positional( "arg1" ) ) );
  assert!( matches!( tokens[ 3 ], ParsedToken::KeyValue { key: "key", value: "value" } ) );
  assert!( matches!( tokens[ 4 ], ParsedToken::Positional( "arg2" ) ) );
}

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ test ]
fn test_validation_during_splitting()
{
  let input = "apple,123,banana,456,cherry,789,grape";
  
  // Test validation that only allows alphabetic tokens
  let results: Vec< _ > = input
    .split_with_validation( &[ "," ], |token| {
      token.chars().all( char::is_alphabetic )
    } )
    .collect();
  
  assert_eq!( results.len(), 7 );
  
  // Alphabetic tokens should succeed
  assert!( results[ 0 ].is_ok() && results[ 0 ].as_ref().unwrap() == &"apple" );
  assert!( results[ 2 ].is_ok() && results[ 2 ].as_ref().unwrap() == &"banana" );
  assert!( results[ 4 ].is_ok() && results[ 4 ].as_ref().unwrap() == &"cherry" );
  assert!( results[ 6 ].is_ok() && results[ 6 ].as_ref().unwrap() == &"grape" );
  
  // Numeric tokens should fail validation
  assert!( results[ 1 ].is_err() );
  assert!( results[ 3 ].is_err() );
  assert!( results[ 5 ].is_err() );
}

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ test ]
fn test_count_valid_tokens()
{
  let input = "apple,123,banana,456,cherry,789,grape";
  
  // Count only alphabetic tokens
  let alphabetic_count = input.count_valid_tokens( &[ "," ], |token| {
    token.chars().all( char::is_alphabetic )
  } );
  
  // Count only numeric tokens  
  let numeric_count = input.count_valid_tokens( &[ "," ], |token| {
    token.chars().all( char::is_numeric )
  } );
  
  assert_eq!( alphabetic_count, 4 ); // apple, banana, cherry, grape
  assert_eq!( numeric_count, 3 );    // 123, 456, 789
}

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ test ]
fn test_multiple_delimiters()
{
  let input = "a,b;c:d|e f\tg";
  let delimiters = &[ ",", ";", ":", "|", " ", "\t" ];
  
  let results: Vec< _ > = input
    .split_with_validation( delimiters, |_| true )
    .collect();
  
  // Should split into 7 tokens
  assert_eq!( results.len(), 7 );
  
  // Verify all tokens
  let expected = [ "a", "b", "c", "d", "e", "f", "g" ];
  for (i, result) in results.iter().enumerate() {
    assert!( result.is_ok() );
    assert_eq!( result.as_ref().unwrap(), &expected[ i ] );
  }
}

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ test ]
fn test_empty_input_handling()
{
  let input = "";
  
  // Empty input should produce no tokens
  let results: Vec< _ > = input
    .split_with_validation( &[ "," ], |_| true )
    .collect();
  
  assert_eq!( results.len(), 0 );
  
  // Command line parsing of empty string
  let cmd_results: Result< Vec< _ >, _ > = input.parse_command_line().collect();
  assert!( cmd_results.is_ok() );
  assert_eq!( cmd_results.unwrap().len(), 0 );
}

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ test ]
fn test_single_token_input()
{
  let input = "single";
  
  // Single token should work correctly
  let results: Vec< _ > = input
    .split_with_validation( &[ "," ], |_| true )
    .collect();
  
  assert_eq!( results.len(), 1 );
  assert!( results[ 0 ].is_ok() );
  assert_eq!( results[ 0 ].as_ref().unwrap(), &"single" );
}

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ test ]
fn test_consecutive_delimiters()
{
  let input = "a,,b,,,c";
  
  // Consecutive delimiters should be handled (empty tokens skipped)
  let results: Vec< _ > = input
    .split_with_validation( &[ "," ], |_| true )
    .collect();
  
  // Should only get non-empty tokens
  assert_eq!( results.len(), 3 );
  assert_eq!( results[ 0 ].as_ref().unwrap(), &"a" );
  assert_eq!( results[ 1 ].as_ref().unwrap(), &"b" );
  assert_eq!( results[ 2 ].as_ref().unwrap(), &"c" );
}

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ test ]
fn test_complex_parsing_scenario()
{
  // Complex real-world scenario: parsing configuration-like input
  let input = "server --port:8080 --host:localhost --ssl --config:app.conf debug.log error.log";
  
  let results: Result< Vec< _ >, _ > = input.parse_command_line().collect();
  assert!( results.is_ok() );
  
  let tokens = results.unwrap();
  assert_eq!( tokens.len(), 7 );
  
  // Verify structure
  assert!( matches!( tokens[ 0 ], ParsedToken::Command( "server" ) ) );
  assert!( matches!( tokens[ 1 ], ParsedToken::KeyValue { key: "port", value: "8080" } ) );
  assert!( matches!( tokens[ 2 ], ParsedToken::KeyValue { key: "host", value: "localhost" } ) );
  assert!( matches!( tokens[ 3 ], ParsedToken::Flag( "ssl" ) ) );
  assert!( matches!( tokens[ 4 ], ParsedToken::KeyValue { key: "config", value: "app.conf" } ) );
  assert!( matches!( tokens[ 5 ], ParsedToken::Positional( "debug.log" ) ) );
  assert!( matches!( tokens[ 6 ], ParsedToken::Positional( "error.log" ) ) );
}

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ test ]
fn test_error_position_information()
{
  let input = "10,invalid,30";
  let results: Vec< _ > = input
    .split_and_parse( &[ "," ], |token| {
      token.parse::< i32 >().map_err( |_| ParseError::InvalidToken {
        token: token.to_string(),
        position: 0, // Position would be calculated in real implementation
        expected: "integer".to_string(),
      } )
    } )
    .collect();
  
  // Verify error contains token information
  assert!( results[ 1 ].is_err() );
  if let Err( ParseError::InvalidToken { token, expected, .. } ) = &results[ 1 ] {
    assert_eq!( token, "invalid" );
    assert_eq!( expected, "integer" );
  } else {
    panic!( "Expected InvalidToken error" );
  }
}

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ test ]
fn test_string_vs_str_compatibility()
{
  let owned_string = String::from( "a,b,c,d" );
  let str_slice = "a,b,c,d";
  
  // Both String and &str should work with the same interface
  let string_results: Vec< _ > = owned_string
    .split_with_validation( &[ "," ], |_| true )
    .collect();
  
  let str_results: Vec< _ > = str_slice
    .split_with_validation( &[ "," ], |_| true )
    .collect();
  
  assert_eq!( string_results.len(), str_results.len() );
  assert_eq!( string_results.len(), 4 );
  
  // Results should be equivalent
  for (string_result, str_result) in string_results.iter().zip( str_results.iter() ) {
    assert_eq!( string_result.as_ref().unwrap(), str_result.as_ref().unwrap() );
  }
}

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ test ]
fn test_performance_characteristics()
{
  // Test with smaller input to verify basic performance characteristics
  let input: String = (0..10)
    .map( |i| i.to_string() )
    .collect::< Vec< _ > >()
    .join( "," );
  
  // Single-pass parsing should handle inputs efficiently
  let results: Result< Vec< i32 >, _ > = input
    .split_and_parse( &[ "," ], |token| {
      token.parse().map_err( |_| ParseError::InvalidToken {
        token: token.to_string(),
        position: 0,
        expected: "integer".to_string(),
      } )
    } )
    .collect();
  
  assert!( results.is_ok() );
  let numbers = results.unwrap();
  assert_eq!( numbers.len(), 10 );
  
  // Verify first and last elements
  assert_eq!( numbers[ 0 ], 0 );
  assert_eq!( numbers[ 9 ], 9 );
}