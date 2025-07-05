//! Derive implementation for testing unit variants in enums with mixed variant kinds.

use super::*;
// use former_types::EntityToFormer; // Not strictly needed if Complex data is i32

/// Enum with a unit variant and a struct-like variant, using Former.
#[derive(Debug, PartialEq, former::Former)]
#[former(standalone_constructors)] // Attribute present, added debug
pub enum MixedEnum
{
  SimpleUnit,
  #[allow(dead_code)] // This variant is not constructed by these specific unit tests
  Complex { data: i32 }, // Complex variant present
}

include!("mixed_enum_unit_only_test.rs");