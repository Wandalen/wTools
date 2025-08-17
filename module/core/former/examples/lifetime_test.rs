
//! This example tests Former with lifetime parameters.

#![allow(missing_docs)]

#[ cfg( feature = "enabled" ) ]
use former_meta::Former;

#[ derive( Debug, PartialEq, Former ) ]
pub struct Simple<'a> {
  name: &'a str,
}

fn main() {
  let s = "hello";
  let instance = Simple::former().name(s).form();
  println!("{instance:?}");
}