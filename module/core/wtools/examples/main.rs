//! example

// #[ cfg( feature = "typing" ) ]
// use wtools::*;

use wtools::implements;

fn main()
{
  #[ cfg( feature = "typing" ) ]
  {
    println!( "implements!( 13_i32 => Copy ) : {}", implements!( 13_i32 => Copy ) );
    println!( "implements!( Box::new( 13_i32 ) => Copy ) : {}", implements!( Box::new( 13_i32 ) => Copy ) );
  }
}
