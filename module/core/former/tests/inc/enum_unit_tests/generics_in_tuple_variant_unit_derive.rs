//! Purpose: Tests the `#[derive(Former)]` macro's generation of constructors for unit variants
//! within an enum that has generic parameters and bounds. This file focuses on verifying
//! the derive-based implementation.
//!
//! Coverage:
//! - Rule 3a (Unit + Default): Verifies `EnumOuter::<T>::other_variant() -> EnumOuter<T>` for a generic enum.
//! - Rule 1a (Unit + `#[scalar]`): Verifies `EnumOuter::<T>::other_variant() -> EnumOuter<T>` (as default for unit is scalar) for a generic enum.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines a generic enum `EnumOuter<X: Copy>` with a unit variant `OtherVariant`, and the `#[derive(Former)]` and `#[ debug ]` attributes.
//! - Relies on the derived static method `EnumOuter::<MyType>::other_variant()`.
//! - Asserts that the `got` instance is equal to an `expected` instance, which is manually
//!   constructed as `EnumOuter::<MyType>::OtherVariant`. This confirms the constructor produces the correct variant instance for a generic enum.
// File: module/core/former/tests/inc/former_enum_tests/unit_tests/generics_in_tuple_variant_unit_derive.rs
use super::*; // Imports testing infrastructure and potentially other common items
use std::fmt::Debug; // Import Debug trait for bounds
use std::marker::PhantomData; // Import PhantomData

// --- Enum Definition with Bounds ---
// Apply Former derive here. This is what we are testing.
#[derive(Debug, PartialEq, former::Former)]
// #[ debug ]
pub enum EnumOuter< X : Copy > // Enum bound: Copy
{
  // --- Unit Variant ---
  OtherVariant,
}

// No include! directive needed as the original only_test file does not test the unit variant.