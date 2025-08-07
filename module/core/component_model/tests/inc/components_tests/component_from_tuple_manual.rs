use super::*;

#[derive(Debug, Default, PartialEq)]
struct TupleStruct(i32, String);

// Manual implementation for the first field (i32)
impl From<&TupleStruct> for i32 {
  #[inline(always)]
  fn from(src: &TupleStruct) -> Self {
    src.0 // Access field by index
  }
}

// Manual implementation for the second field (String)
impl From<&TupleStruct> for String {
  #[inline(always)]
  fn from(src: &TupleStruct) -> Self {
    src.1.clone() // Access field by index
  }
}

//

// Reuse the same test logic
include!("./only_test/component_from_tuple.rs");
