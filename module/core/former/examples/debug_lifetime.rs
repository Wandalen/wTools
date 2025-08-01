//! Example demonstrating lifetime debugging with Former-derived structs.

#![allow(missing_docs)]

#[cfg(feature = "enabled")]
use former_meta::Former;

#[derive(Debug, PartialEq, Former)]
// #[debug] // Commented out - debug attribute only for temporary debugging
pub struct Test<'a> {
  data: &'a str,
}

fn main() {
  println!("This won't compile, but we can see the debug output");
}