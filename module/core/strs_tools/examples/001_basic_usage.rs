//! Basic usage examples for `strs_tools` crate.
//!
//! This example demonstrates the core functionality of `strs_tools`,
//! showing how to perform advanced string operations that go beyond
//! Rust's standard library capabilities.

#[ allow( unused_imports ) ]
use strs_tools :: *;

fn main()
{
  println!( "=== strs_tools Basic Examples ===" );
  
  basic_string_splitting();
  delimiter_preservation();
}

/// Demonstrates basic string splitting functionality.
/// 
/// Unlike standard `str.split()`, `strs_tools` provides more control
/// over how delimiters are handled and what gets returned.
fn basic_string_splitting()
{
  println!( "\n--- Basic String Splitting ---" );
  
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  {
  // Split a simple string on spaces
  let src = "abc def ghi";
  let iter = string ::split()
  .src( src )                    // Set source string
  .delimeter( " " )              // Set delimiter to space
  .perform();                    // Execute the split operation
  
  let result: Vec< String > = iter
  .map( String ::from )           // Convert each segment to owned String
  .collect();
  
  println!( "Input: '{src}' -> {result:?}" );
  // Note: With stripping(false), delimiters are preserved in output
  assert_eq!( result, vec![ "abc", " ", "def", " ", "ghi" ] );
  
  // Example with delimiter that doesn't exist
  let iter = string ::split()
  .src( src )
  .delimeter( "x" )              // Delimiter not found in string
  .perform();
  
  let result: Vec< String > = iter.map( String ::from ).collect();
  println!( "No delimiter found: '{src}' -> {result:?}" );
  assert_eq!( result, vec![ "abc def ghi" ] );  // Returns original string
 }
}

/// Demonstrates delimiter preservation feature.
///
/// This shows how `strs_tools` can preserve delimiters in the output,
/// which is useful for reconstructing the original string or for
/// maintaining formatting context.
fn delimiter_preservation()
{
  println!( "\n--- Delimiter Preservation ---" );
  
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  {
  let src = "word1 word2 word3";
  
  // Split while preserving delimiters (spaces)
  let iter = string ::split()
  .src( src )
  .delimeter( " " )
  .stripping( false )            // Keep delimiters in output
  .perform();
  
  let result: Vec< String > = iter.map( String ::from ).collect();
  
  println!( "With delimiters preserved: " );
  println!( "  Input: '{src}' -> {result:?}" );
  assert_eq!( result, vec![ "word1", " ", "word2", " ", "word3" ] );
  
  // Verify we can reconstruct the original string
  let reconstructed = result.join( "" );
  assert_eq!( reconstructed, src );
  println!( "  Reconstructed: '{reconstructed}'" );
 }
}
