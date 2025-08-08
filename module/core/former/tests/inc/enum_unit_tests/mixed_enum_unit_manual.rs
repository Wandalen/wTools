//! Manual implementation for testing unit variants in enums with mixed variant kinds.

use super::*;

/// Enum with a unit variant and a struct-like variant.
#[ derive( Debug, PartialEq ) ]
pub enum MixedEnum {
  SimpleUnit,
  #[ allow( dead_code ) ] // This variant is not constructed by these specific unit tests
  Complex {
    data: String,
  }, // data field for the complex variant
}

impl MixedEnum {
  #[ inline( always ) ]
  pub fn simple_unit() -> Self {
    Self::SimpleUnit
  }
}

// Standalone constructor for the unit variant
#[ inline( always ) ]
pub fn simple_unit() -> MixedEnum {
  MixedEnum::SimpleUnit
}

include!("mixed_enum_unit_only_test.rs");
