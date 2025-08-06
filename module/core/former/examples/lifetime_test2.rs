//! Example demonstrating Former working with custom lifetime names and substitution.

#![allow(missing_docs)]

// This example demonstrates Former working with different lifetime names.
// The FormerBegin trait expects lifetime 'a, but the struct uses 'x.
// The derive macro now properly handles this by substituting lifetimes.

#[cfg(feature = "enabled")]
use former_meta::Former;

#[derive(Debug, PartialEq, Former)]
pub struct Other<'x> {
  data: &'x str,
}

fn main() {
  let s = "hello";
  let instance = Other::former().data(s).form();
  println!("{instance:?}");
}