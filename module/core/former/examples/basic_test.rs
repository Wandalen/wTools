//! This example tests Former with a basic struct.

#![allow(missing_docs)]

#[ cfg( feature = "enabled" ) ]
use former_meta::Former;

/// A basic structure to test Former derive macro
#[ derive( Debug, PartialEq, Former ) ]
pub struct Basic {
  data: i32,
}

fn main() {
  let instance = Basic::former().data(42).form();
  println!("{instance:?}");
}