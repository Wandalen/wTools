#![allow(dead_code)]
#[allow(unused_imports)]
use super::*;

// Test with just ?Sized
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[derive(Debug, PartialEq, the_module::Former)]
#[derive(Debug, PartialEq)]
#[debug]
pub struct WithSized<T: ?Sized> {
  data: Box<T>,
}

// Test that manual version would look like:
// pub struct WithSizedFormerStorage<T: ?Sized> {
//   data: Option<Box<T>>,
// }