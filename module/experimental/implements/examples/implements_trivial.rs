//! Demonstrates basic trait checking with the implements! macro.
//!
//! This example shows how to check trait implementation at runtime using the
//! `implements!` macro. It compares a primitive type (i32) which implements Copy
//! with a heap-allocated type (Box<i32>) which does not implement Copy.

pub use implements :: *;

fn main() 
{
  dbg!(implements!( 13_i32 => Copy ));
  // < implements!( 13_i32 = > Copy ) : true
  dbg!(implements!( Box ::new( 13_i32 ) => Copy ));
  // < implements!( 13_i32 = > Copy ) : false
}
