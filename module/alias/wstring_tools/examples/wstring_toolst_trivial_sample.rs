//! Demonstrates basic string splitting functionality using `wstring_tools`.
//!
//! This example shows how to use the `split()` builder to tokenize strings with
//! custom delimiters, both when delimiters are present and absent.
#[ allow( unused_imports ) ]
use wstring_tools::*;

fn main()
{
  #[ cfg( all( feature = "split", not( feature = "no_std" ) ) ) ]
  {
  /* delimeter exists */
  let src = "abc def";
  let iter = string::split()
  .src( src )
  .delimeter( " " )
  .stripping( false )
  .perform();
  let iterated = iter.map( String::from ).collect::< Vec< _ > >();
  assert_eq!( iterated, vec![ "abc", " ", "def" ] );

  /* delimeter not exists */
  let src = "abc def";
  let iter = string::split()
  .src( src )
  .delimeter( "g" )
  .perform();
  let iterated = iter.map( String::from ).collect::< Vec< _ > >();
  assert_eq!( iterated, vec![ "abc def" ] );
 }
}
