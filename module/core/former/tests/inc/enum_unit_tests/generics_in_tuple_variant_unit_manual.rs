// File: module/core/former/tests/inc/former_enum_tests/unit_tests/generics_in_tuple_variant_unit_manual.rs
use super::*; // Imports testing infrastructure and potentially other common items
use std::fmt::Debug; // Import Debug trait for bounds
use std::marker::PhantomData; // Import PhantomData

// --- Enum Definition with Bounds ---
#[ derive( Debug, PartialEq ) ]
pub enum EnumOuter
{
  // --- Unit Variant ---
  OtherVariant,
}

// --- Manual constructor for OtherVariant ---
impl EnumOuter
{
  #[ allow( dead_code ) ]
  pub fn other_variant() -> Self
  {
    EnumOuter::OtherVariant
  }
}

// No include! directive needed as the original only_test file does not test the unit variant.