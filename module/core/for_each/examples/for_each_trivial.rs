//! qqq : write proper description
use for_each::for_each;

fn main()
{

  for_each!( dbg, "a", "b", "c" );

  // generates
  dbg!( "a" );
  dbg!( "b" );
  dbg!( "c" );

}