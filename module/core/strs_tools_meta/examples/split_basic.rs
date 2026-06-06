//! Basic usage of `optimize_split!` compile-time split macro.
//!
//! Run with: `cargo run --example split_basic --features optimize_split`

use strs_tools_meta::optimize_split;

fn main()
{
  // Single character delimiter — highest optimization path
  let csv = "field1,field2,field3";
  let result = optimize_split!( csv, "," );
  println!( "CSV split: {result:?}" );
  assert_eq!( result, vec![ "field1", "field2", "field3" ] );

  // Multiple delimiters — multi-delimiter optimization path
  let mixed = "a,b;c";
  let result = optimize_split!( mixed, [ ",", ";" ] );
  println!( "Multi-delimiter split: {result:?}" );
  assert_eq!( result, vec![ "a", "b", "c" ] );

  // Preserve delimiters in output
  let data = "x,y,z";
  let result = optimize_split!( data, ",", preserve_delimiters = true );
  println!( "Preserve delimiters: {result:?}" );
  assert!( result.contains( &"," ) );

  println!( "All assertions passed." );
}
