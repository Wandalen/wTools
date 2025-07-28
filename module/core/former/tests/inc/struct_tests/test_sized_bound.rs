#![allow(dead_code)]
#[allow(unused_imports)]
use super::*;

// Test with just ?Sized
#[derive(Debug, PartialEq, the_module::Former)]
#[debug]
pub struct WithSized<T: ?Sized> {
  data: Box<T>,
}

// Test that manual version would look like:
// pub struct WithSizedFormerStorage<T: ?Sized> {
//   data: Option<Box<T>>,
// }