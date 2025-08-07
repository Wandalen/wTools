use super::*;

// Define a source tuple struct with several fields
#[derive(Debug, Default, PartialEq, Clone)] // Added Clone for manual impl
struct SourceTuple(i32, String, f32);

// Define a target tuple struct (no derive here)
#[derive(Debug, Default, PartialEq)]
struct TargetTuple(i32, String);

// Implement From<&SourceTuple> for each type it contains that TargetTuple needs
impl From<&SourceTuple> for i32 {
  #[inline(always)]
  fn from(src: &SourceTuple) -> Self {
    src.0.clone()
  }
}

impl From<&SourceTuple> for String {
  #[inline(always)]
  fn from(src: &SourceTuple) -> Self {
    src.1.clone()
  }
}

// Manual implementation of From<T> for TargetTuple
impl<T> From<T> for TargetTuple
where
  T: Into<i32>,
  T: Into<String>,
  T: Clone, // The generic T needs Clone for the assignments below
{
  #[inline(always)]
  fn from(src: T) -> Self {
    let field0 = Into::<i32>::into(src.clone());
    let field1 = Into::<String>::into(src.clone());
    Self(field0, field1) // Use tuple constructor syntax
  }
}

//

// Reuse the same test logic
include!("./only_test/from_components_tuple.rs");
