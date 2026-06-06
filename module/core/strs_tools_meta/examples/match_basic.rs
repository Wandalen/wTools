//! Basic usage of `optimize_match!` compile-time match macro.
//!
//! Run with: `cargo run --example match_basic --features optimize_match`

use strs_tools_meta::optimize_match;

fn main()
{
  // Single pattern — direct find optimization path
  let input = "prefix_value";
  let result = optimize_match!( input, "prefix_" );
  println!( "Single pattern: {result:?}" );
  assert_eq!( result, Some( 0 ) );

  // Multiple patterns — trie-based optimization path
  let url = "https://example.com";
  let result = optimize_match!( url, [ "http://", "https://", "ftp://" ] );
  println!( "URL protocol match: {result:?}" );
  assert!( result.is_some() );

  // No match
  let text = "hello world";
  let result = optimize_match!( text, "xyz" );
  println!( "No match: {result:?}" );
  assert_eq!( result, None );

  println!( "All assertions passed." );
}
