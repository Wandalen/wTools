// File: module/core/former/tests/inc/former_enum_tests/unit_tests/generics_in_tuple_variant_unit_derive.rs
use super::*; // Imports testing infrastructure and potentially other common items
use std::fmt::Debug; // Import Debug trait for bounds
use std::marker::PhantomData;

// --- Enum Definition with Bounds ---
// Apply Former derive here. This is what we are testing.
#[derive(Debug, PartialEq, former::Former)]
#[debug]
pub enum EnumOuter< X : Copy > // Enum bound: Copy
{
  // --- Unit Variant ---
  OtherVariant,
}

// No include! directive needed as the original only_test file does not test the unit variant.