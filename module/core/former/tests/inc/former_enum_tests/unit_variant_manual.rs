use super::*;

/// Enum with only unit variants for testing.
#[derive(Debug, PartialEq)]
enum Status
{
  Pending,
  Complete,
}

// Manual implementation of static constructors
impl Status
{
  #[inline(always)]
  pub fn pending() -> Self
  {
    Self::Pending
  }

  #[inline(always)]
  pub fn complete() -> Self
  {
    Self::Complete
  }
}

// Include the test logic
include!("unit_variant_only_test.rs");