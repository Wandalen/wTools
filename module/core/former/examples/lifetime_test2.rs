// This example demonstrates a current limitation of the Former derive macro.
// The FormerBegin trait is hardcoded to use lifetime 'a, so structs that use
// different lifetime names (like 'x in this example) will fail to compile.
//
// This is a known issue that requires redesigning the FormerBegin trait to be
// more flexible with lifetime parameters.

use former::Former;

// #[derive(Debug, PartialEq, Former)]
#[derive(Debug, PartialEq)]
pub struct Other<'x> {
  data: &'x str,
}

fn main() {
  // This won't compile due to the lifetime mismatch
  // let s = "hello";
  // let instance = Other::former().data(s).form();
  // println!("{:?}", instance);
  
  println!("This example demonstrates a limitation with non-'a lifetimes");
}