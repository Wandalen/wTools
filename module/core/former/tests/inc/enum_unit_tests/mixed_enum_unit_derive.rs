//! Derive implementation for testing unit variants in enums with mixed variant kinds.

use super::*;
// use former_types::EntityToFormer; // Not strictly needed if Complex data is i32

/// Enum with a unit variant and a struct-like variant, using Former.
#[derive(Debug, PartialEq, former::Former)]
#[former(standalone_constructors)] // Restore attribute
pub enum MixedEnum
{
  SimpleUnit,
  Complex { data: i32 }, // Restore Complex variant with i32
}

include!("mixed_enum_unit_only_test.rs");