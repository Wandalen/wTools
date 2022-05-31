#[ allow( unused_imports ) ]
use wstring_tools::*; /* qqq : xxx : use rather prelude. discuss first, please */

fn main()
{
  #[ cfg( feature = "split" ) ]
  {
    /* delimeter exists */
    let src = "abc def";
    let iter = string::split()
    .src( src )
    .delimeter( " " )
    .perform();
    let iterated = iter.map( | e | String::from( e ) ).collect::< Vec< _ > >();
    assert_eq!( iterated, vec![ "abc", " ", "def" ] );

    /* delimeter no exists */
    let src = "abc def";
    let iter = string::split()
    .src( src )
    .delimeter( "g" )
    .perform();
    let iterated = iter.map( | e | String::from( e ) ).collect::< Vec< _ > >();
    assert_eq!( iterated, vec![ "abc def" ] );
  }
}
