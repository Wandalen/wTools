//! Demonstrates basic function-style invocation of the `for_each!` macro.
//!
//! This example shows how to apply a callback macro (`dbg!`) to multiple elements
//! using the simplest syntax: `for_each!(callback, elem1, elem2, ...)`.
//!
//! The macro expands to individual callback invocations for each element.

use for_each::for_each;

fn main()
{
  // Apply dbg! macro to each string element
  for_each!( dbg, "a", "b", "c" );

  // The above macro invocation generates the equivalent of:
  // dbg!( "a" );
  // dbg!( "b" );
  // dbg!( "c" );
}