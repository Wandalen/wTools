//! Custom Parser Configuration Example
//!
//! This example demonstrates:
//! - Configuring parser options for strict parsing
//! - Error handling for duplicate arguments
//! - Controlling positional vs named argument ordering

use unilang_parser::{ Parser, UnilangParserOptions };

fn main()
{
  println!( "=== Custom Parser Configuration ===" );

  // Default configuration (permissive)
  println!( "\n1. Default Configuration (Permissive):" );
  let default_parser = Parser::new( UnilangParserOptions::default() );

  // This should work with default settings
  match default_parser.parse_single_instruction( "cmd pos1 name::val1 pos2 name::val2" )
  {
    Ok( instruction ) =>
    {
      println!( "✓ Default parser accepted mixed argument order" );
      println!( "  Positional: {:?}", instruction.positional_arguments );
      println!( "  Named: {:?}", instruction.named_arguments );
    }
    Err( e ) => println!( "✗ Default parser error: {e}" ),
  }

  // Strict configuration
  println!( "\n2. Strict Configuration:" );
  let strict_options = UnilangParserOptions
  {
    main_delimiters : vec![ " ", "." ],
    operators : vec![ "::", "?", "!" ],
    whitespace_is_separator : true,
    error_on_positional_after_named : true,
    error_on_duplicate_named_arguments : true,
    quote_pairs : vec![ ( '"', '"' ), ( '\'', '\'' ) ],
    verbosity : 0,
  };
  let strict_parser = Parser::new( strict_options );

  // Test duplicate named arguments (should error in strict mode)
  println!( "\n2a. Testing Duplicate Named Arguments:" );
  match strict_parser.parse_single_instruction( "cmd arg1::val1 arg1::val2" )
  {
    Ok( _ ) => println!( "✗ Strict parser unexpectedly accepted duplicates" ),
    Err( e ) =>
    {
      println!( "✓ Strict parser correctly rejected duplicate arguments" );
      println!( "   Error: {e}" );
    }
  }

  // Test positional after named (should error in strict mode)
  println!( "\n2b. Testing Positional After Named:" );
  match strict_parser.parse_single_instruction( "cmd named::value positional_arg" )
  {
    Ok( _ ) => println!( "✗ Strict parser unexpectedly accepted positional after named" ),
    Err( e ) =>
    {
      println!( "✓ Strict parser correctly rejected positional after named" );
      println!( "   Error: {e}" );
    }
  }

  // Show what strict parser accepts
  println!( "\n2c. What Strict Parser Accepts:" );
  match strict_parser.parse_single_instruction( "cmd pos1 pos2 named1::val1 named2::val2" )
  {
    Ok( instruction ) =>
    {
      println!( "✓ Strict parser accepted well-ordered arguments" );
      println!( "  Positional: {:?}", instruction.positional_arguments );
      println!( "  Named: {:?}", instruction.named_arguments );
    }
    Err( e ) => println!( "✗ Strict parser error: {e}" ),
  }

  // Compare configurations side by side
  println!( "\n=== Configuration Comparison ===" );
  let test_cases = vec!
  [
    ( "Mixed order", "cmd pos1 name::val pos2" ),
    ( "Duplicates", "cmd name::val1 name::val2" ),
    ( "Valid order", "cmd pos1 pos2 name::val" ),
  ];

  for ( description, test_input ) in test_cases
  {
    println!( "\nTest: {description} - '{test_input}'" );

    match default_parser.parse_single_instruction( test_input )
    {
      Ok( _ ) => println!( "  Default: ✓ Accepted" ),
      Err( _ ) => println!( "  Default: ✗ Rejected" ),
    }

    match strict_parser.parse_single_instruction( test_input )
    {
      Ok( _ ) => println!( "  Strict:  ✓ Accepted" ),
      Err( _ ) => println!( "  Strict:  ✗ Rejected" ),
    }
  }

  // Demonstrate configuration flexibility
  println!( "\n=== Custom Configuration Options ===" );

  // Only error on duplicates, allow mixed order
  let partial_strict = UnilangParserOptions
  {
    main_delimiters : vec![ " ", "." ],
    operators : vec![ "::", "?", "!" ],
    whitespace_is_separator : true,
    error_on_duplicate_named_arguments : true,
    error_on_positional_after_named : false, // Allow mixed order
    quote_pairs : vec![ ( '"', '"' ), ( '\'', '\'' ) ],
    verbosity : 0,
  };
  let partial_parser = Parser::new( partial_strict );

  println!( "Partial strict (no duplicates, mixed order OK):" );
  match partial_parser.parse_single_instruction( "cmd pos1 name::val pos2" )
  {
    Ok( _ ) => println!( "  ✓ Accepted mixed order" ),
    Err( _ ) => println!( "  ✗ Rejected mixed order" ),
  }

  match partial_parser.parse_single_instruction( "cmd name::val1 name::val1" )
  {
    Ok( _ ) => println!( "  ✗ Unexpectedly accepted duplicates" ),
    Err( _ ) => println!( "  ✓ Correctly rejected duplicates" ),
  }

  println!( "\n✓ Custom parser configuration demonstration complete!" );
}