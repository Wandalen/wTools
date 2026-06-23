#[ allow( unused_imports ) ]
use super::*;
use the_module::string::parser::*;

#[ test ]
fn test_parse_and_split_integers()
{
  let input = "1,2,3,4,5";
  let result: Result< Vec< i32 >, _ > = input
    .split_and_parse( &[ "," ], |token| {
      token.parse().map_err( |_| ParseError::InvalidToken {
        token: token.to_string(),
        position: 0,
        expected: "integer".to_string(),
      } )
    } )
    .collect();

  assert!( result.is_ok() );
  let numbers = result.unwrap();
  assert_eq!( numbers, vec![ 1, 2, 3, 4, 5 ] );
}

#[ test ]
fn test_command_line_parsing()
{
  let input = "myapp --verbose input.txt output.txt";
  let result: Result< Vec< _ >, _ > = input.parse_command_line().collect();

  assert!( result.is_ok() );
  let tokens = result.unwrap();

  assert_eq!( tokens.len(), 4 );
  assert!( matches!( tokens[ 0 ], ParsedToken::Command( "myapp" ) ) );
  assert!( matches!( tokens[ 1 ], ParsedToken::Flag( "verbose" ) ) );
  assert!( matches!( tokens[ 2 ], ParsedToken::Positional( "input.txt" ) ) );
  assert!( matches!( tokens[ 3 ], ParsedToken::Positional( "output.txt" ) ) );
}

#[ test ]
fn test_key_value_parsing()
{
  let input = "config timeout: 30 retries: 5";
  let result: Result< Vec< _ >, _ > = input.parse_command_line().collect();

  if result.is_err() {
    println!( "DEBUG: Error = {:?}", result );
  }
  assert!( result.is_ok() );
  let tokens = result.unwrap();

  println!( "DEBUG: tokens = {:?}", tokens );
  assert_eq!( tokens.len(), 3 );
  assert!( matches!( tokens[ 0 ], ParsedToken::Command( "config" ) ) );

  if let ParsedToken::KeyValue { key, value } = &tokens[ 1 ]
  {
    assert_eq!( *key, "timeout" );
    assert_eq!( *value, "30" );
  }
  else
  {
    panic!( "Expected KeyValue token" );
  }

  if let ParsedToken::KeyValue { key, value } = &tokens[ 2 ]
  {
    assert_eq!( *key, "retries" );
    assert_eq!( *value, "5" );
  }
  else
  {
    panic!( "Expected KeyValue token" );
  }
}

#[ test ]
fn test_validation_during_split()
{
  let input = "apple,123,banana,456,cherry";

  // Count only alphabetic tokens
  let alpha_count = input.count_valid_tokens( &[ "," ], |token| {
    token.chars().all( char::is_alphabetic )
  } );

  assert_eq!( alpha_count, 3 ); // apple, banana, cherry
}

#[ test ]
fn test_empty_and_invalid_tokens()
{
  let input = "valid,123,banana";
  let results: Vec< _ > = input
    .split_with_validation( &[ "," ], |token| token.chars().all( char::is_alphabetic ) )
    .collect();

  // Should have validation errors for "123" token (not alphabetic)
  assert!( results.iter().any( std::result::Result::is_err ) );

  // Should have successful results for "valid" and "banana"
  assert!( results.iter().any( std::result::Result::is_ok ) );
}
