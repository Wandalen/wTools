//! qqq : write proper description
pub use implements::*;

fn main()
{
  dbg!( implements!( 13_i32 => Copy ) );
  // < implements!( 13_i32 => Copy ) : true
  dbg!( implements!( Box::new( 13_i32 ) => Copy ) );
  // < implements!( 13_i32 => Copy ) : false
}
