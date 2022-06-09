#[ allow( unused_imports ) ]
use wstring_tools::*; /* qqq : xxx : use rather prelude. discuss first, please */

fn main()
{
  #[ cfg( all( feature = "split", feature = "use_std" ) ) ]
  {
    /* delimeter exists */
    let src = "abc def";
    let iter = string::split()
    .src( src )
    .delimeter( " " )
    .stripping( false )
    .perform();
    let iterated = iter.map( | e | String::from( e ) ).collect::< Vec< _ > >();
    assert_eq!( iterated, vec![ "abc", " ", "def" ] );

    /* delimeter not exists */
    let src = "abc def";
    let iter = string::split()
    .src( src )
    .delimeter( "g" )
    .perform();
    let iterated = iter.map( | e | String::from( e ) ).collect::< Vec< _ > >();
    assert_eq!( iterated, vec![ "abc def" ] );
  }
}
