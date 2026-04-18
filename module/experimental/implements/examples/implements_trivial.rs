//! Demonstrates basic trait checking with the implements! macro.
//!
//! This example shows how to check trait implementation at runtime using the
//! `implements!` macro. It compares a primitive type (i32) which implements Copy
//! with a heap-allocated type (Box<i32>) which does not implement Copy.

pub use implements :: *;

// Dual-main: `#![cfg(feature = "enabled")]` at crate level removes fn main() → E0601 for binary crates.
// Fallback empty main required so cargo can always produce a valid binary regardless of features.
#[ cfg( not( feature = "enabled" ) ) ]
fn main() {}

#[ cfg( feature = "enabled" ) ]
fn main()
{
  dbg!(implements!( 13_i32 => Copy ));
  // < implements!( 13_i32 = > Copy ) : true
  dbg!(implements!( Box ::new( 13_i32 ) => Copy ));
  // < implements!( 13_i32 = > Copy ) : false
}
