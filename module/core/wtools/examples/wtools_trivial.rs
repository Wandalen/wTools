//! qqq : write proper description
// #[ cfg( feature = "typing" ) ]
// use wtools::*;
#[ cfg( any( feature = "typing_implements", feature = "typing") ) ]
use wtools::implements;

fn main()
{
  #[ cfg( feature = "typing" ) ]
  {
    println!( "implements!( 13_i32 => Copy ) : {}", implements!( 13_i32 => Copy ) );
    println!( "implements!( Box::new( 13_i32 ) => Copy ) : {}", implements!( Box::new( 13_i32 ) => Copy ) );
  }
}
