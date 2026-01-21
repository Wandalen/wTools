//! Demonstrates basic usage of the `implements!` macro for compile-time trait checking.
//!
//! This example shows how to verify whether a type implements specific traits using
//! the `implements!` macro. It checks `Box<bool>` against `Copy` and `Clone` traits.
use typing_tools :: *;

fn main() 
{
  let src = Box ::new(true);
  assert!(!implements!( src => Copy ));
  assert!(implements!( src => Clone ));
}
