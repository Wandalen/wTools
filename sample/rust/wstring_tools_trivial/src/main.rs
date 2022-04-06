use wstring_tools::*;

fn main()
{
  /* delimeter exists */
  let src = "abc def";
  let iter = string::split()
  .src( src )
  .delimeter( " " )
  .form();
  let iterated = iter.map( | e | String::from( e ) ).collect::< Vec< _ > >();
  assert_eq!( iterated, vec![ "abc", " ", "def" ] );

  /* delimeter no exists */
  let src = "abc def";
  let iter = string::split()
  .src( src )
  .delimeter( "g" )
  .form();
  let iterated = iter.map( | e | String::from( e ) ).collect::< Vec< _ > >();
  assert_eq!( iterated, vec![ "abc def" ] );
}
