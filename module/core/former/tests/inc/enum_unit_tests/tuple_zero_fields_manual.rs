//! Manual implementation for testing zero-field tuple variants.
use super::*; // To find the _only_test.rs file via include
use former::Former; // Only for derive on ZeroTupleScalar if we test manual derive here. Not needed for pure manual.

// For Test Matrix Row: T8.1 (Default behavior)
#[derive(Debug, PartialEq)]
pub enum ZeroTuple { Variant() }

impl ZeroTuple
{
  #[inline(always)]
  pub fn variant() -> Self
  {
    Self::Variant()
  }
}

#[inline(always)]
pub fn zero_tuple_variant() -> ZeroTuple // Renamed
{
  ZeroTuple::Variant()
}

// For Test Matrix Row: T8.2 (#[scalar] attribute)
// Manual equivalent of #[derive(Former)] #[former(scalar)]
#[derive(Debug, PartialEq)]
pub enum ZeroTupleScalar { Variant() }

impl ZeroTupleScalar
{
  #[inline(always)]
  pub fn variant() -> Self // Scalar generates method with same name as variant
  {
    Self::Variant()
  }
}

// Standalone for ZeroTupleScalar
// The derive macro with #[former(scalar, standalone_constructors)] would generate this.
// We name it zero_tuple_scalar_variant to match the _only_test.rs expectation for the scalar case.
#[inline(always)]
pub fn zero_tuple_scalar_variant() -> ZeroTupleScalar // Renamed
{
  ZeroTupleScalar::Variant()
}

include!("tuple_zero_fields_only_test.rs");