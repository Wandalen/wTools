use super::*;

/// Enum with only unit variants for testing.
#[derive(Debug, PartialEq)]
pub enum Status // Made enum public
{
  Pending, // Variants are public by default if enum is public
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

// Manual implementation of standalone constructors (moved before include!)
#[inline(always)]
pub fn pending() -> Status
{
  Status::Pending
}

#[inline(always)]
pub fn complete() -> Status
{
  Status::Complete
}

// Include the test logic (now defined after standalone constructors)
include!("unit_variant_only_test.rs");