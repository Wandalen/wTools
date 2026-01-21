//! Smoke testing of the package.
//!
//! Smoke tests verify basic compilation, imports, and trivial functionality.
//! These tests ensure the alias crate properly re-exports underlying functionality.

#[ cfg( feature = "indentation" ) ]
#[ test ]
fn local_smoke_test()
{
  // Verify wstring_tools re-exports strs_tools indentation functionality
  use wstring_tools::string;

  // Test indentation with prefix, src, and postfix (3 required arguments)
  let result = string::indentation( ">>", "line1\nline2", ";" );
  assert!( result.contains( ">>" ), "Smoke test failed: indentation didn't add prefix" );
  assert!( result.contains( ';' ), "Smoke test failed: indentation didn't add postfix" );
}

#[ cfg( feature = "parse_number" ) ]
#[ test ]
fn published_smoke_test()
{
  // Verify parse_number functionality is accessible via wstring_tools
  use wstring_tools::string;

  // Test basic number parsing from lexical re-export
  let result = string::number::parse::< i32, _ >( "42" );
  assert_eq!( result, Ok( 42 ), "Smoke test failed: number::parse returned unexpected result" );
}
