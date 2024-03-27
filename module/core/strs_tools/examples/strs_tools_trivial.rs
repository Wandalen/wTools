//! qqq : write proper description
#[ allow( unused_imports ) ]
use strs_tools::*;

fn main()
{
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
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