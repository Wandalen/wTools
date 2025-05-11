//! Manual implementation for testing unit variants in generic enums.

use super::*;

/// Generic enum with a unit variant.
#[derive(Debug, PartialEq)]
pub enum GenericOption<T>
{
  #[allow(dead_code)] // This variant is not constructed by these specific unit tests
  Value(T),
  NoValue, // Renamed from UnitNone
}

impl<T> GenericOption<T>
{
  #[inline(always)]
  pub fn no_value() -> Self // Renamed from unit_none
  {
    Self::NoValue // Renamed from UnitNone
  }
}

// Standalone constructor
#[inline(always)]
pub fn no_value<T>() -> GenericOption<T> // Renamed from unit_none
{
  GenericOption::<T>::NoValue // Renamed from UnitNone
}

include!("generic_unit_variant_only_test.rs");