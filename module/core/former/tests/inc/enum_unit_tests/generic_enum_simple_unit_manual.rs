#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Purpose: Provides a manual implementation of a constructor for a unit variant
//! within a generic enum with bounds, to serve as a reference for verifying
//! the `#[ derive( Former ) ]` macro's behavior.
//!
//! Coverage:
//! - Rule 3a (Unit + Default): Manual implementation of static method `EnumOuter::other_variant()`.
//! - Rule 1a (Unit + `#[ scalar ]`): Manual implementation of static method (as default for unit is scalar).
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines a generic enum `EnumOuter` with a unit variant `OtherVariant`.
//! - Manually implements a static method `EnumOuter::other_variant()` that mirrors the expected generated code for a scalar unit variant.
//! - This file is used as a reference for comparison in tests that include `generics_in_tuple_variant_only_test.rs` (though that file does not currently test unit variants).
// File: module/core/former/tests/inc/enum_unit_tests/generic_enum_simple_unit_manual.rs
use super::*; // Imports testing infrastructure and potentially other common items
use core::fmt::Debug; // Import Debug trait for bounds
                      // use std::marker::PhantomData; // No longer needed for this simple case

// --- Enum Definition with Bounds ---
#[ derive( Debug, PartialEq ) ]
pub enum EnumOuter<X: Copy + Debug + PartialEq> {
  // --- Unit Variant ---
  OtherVariant,
  #[ allow( dead_code ) ] // Re-added to use generic X
  _Phantom(core::marker::PhantomData<X>),
}

// --- Manual constructor for OtherVariant ---
impl<X: Copy + Debug + PartialEq> EnumOuter<X> {
  #[ allow( dead_code ) ]
  pub fn other_variant() -> Self {
    EnumOuter::OtherVariant
  }
}

include!("generic_enum_simple_unit_only_test.rs");
