//! Manual implementation for testing unit variants in generic enums.

use super::*;

/// Generic enum with a unit variant.
#[derive(Debug, PartialEq)]
pub enum GenericOption<T>
{
  Value(T),
  UnitNone,
}

impl<T> GenericOption<T>
{
  #[inline(always)]
  pub fn unit_none() -> Self
  {
    Self::UnitNone
  }
}

// Standalone constructor
#[inline(always)]
pub fn unit_none<T>() -> GenericOption<T>
{
  GenericOption::<T>::UnitNone
}

include!("generic_unit_variant_only_test.rs");