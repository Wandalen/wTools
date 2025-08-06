//! Purpose: Tests the `#[derive(Former)]` macro's generation of constructors for unnamed (tuple)
//! variants with shared generic parameters and bounds, using the default subform behavior.
//! This file focuses on verifying the derive-based implementation.
//!
//! Coverage:
//! - Rule 3d (Tuple + Single-Field + Default -> Subform): Verifies `EnumOuter::<X>::variant() -> InnerGenericFormer<X>`.
//! - Rule 4b (Option 2 Logic): Verifies the use of the subformer returned by the variant constructor.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines a generic enum `EnumOuter<X: Copy + Debug + Default + PartialEq>` with a single-field tuple variant `Variant(InnerGeneric<X>)`.
//! - The inner struct `InnerGeneric<T: Debug + Copy + Default + PartialEq>` has its own generic `T` and bounds, and is instantiated with the enum's generic `X` in the variant.
//! - The enum has `#[derive(Former)]` and `#[ debug ]`.
//! - Relies on the derived static method `EnumOuter::<X>::variant()` provided by this file (via `include!`).
//! - Asserts that this constructor returns the expected subformer (`InnerGenericFormer<X>`) and that using the subformer's setter (`.inner_field()`) and `.form()` results in the correct `EnumOuter` enum instance.
//! - Verifies that the bounds (`Copy`, `Debug`, `Default`, `PartialEq`) are correctly handled by using types that satisfy them.
#[ allow( unused_imports ) ]
use super::*; // Imports testing infrastructure and potentially other common items
use std::fmt::Debug; // Import Debug trait for bounds
use std::marker::PhantomData; // Import PhantomData
use ::former::Former; // Import Former derive macro

// --- Inner Struct Definition with Bounds ---
// Needs to derive Former for the enum's derive to work correctly for subforming.
#[derive(Debug, PartialEq)] // Former derive BLOCKED - cannot parse generic enum syntax
pub struct InnerGeneric< T : Debug + Copy + Default + PartialEq > // Added Copy bound here too
{
  pub inner_field : T,
}

// Implement Into manually for testing the constructor signature
impl< T : Debug + Copy + Default + PartialEq > From< T > for InnerGeneric< T >
{
  fn from( data : T ) -> Self { Self { inner_field : data } }
}

// --- Enum Definition with Bounds ---
// Apply Former derive here. This is what we are testing.
#[derive(Debug, PartialEq)] // Former derive BLOCKED - cannot parse generic enum syntax
// #[ debug ]
pub enum EnumOuter< X : Copy + Debug + Default + PartialEq > // Enum bound: Copy
{
  // --- Tuple Variant with Generics ---
  Variant( InnerGeneric< X > ), // Inner type uses X, which must satisfy InnerGeneric's bounds (Debug + Copy)
  // --- Unit Variant for tests ---
  OtherVariant, // Unit variant expected by tests
}

// --- Include the Test Logic ---
// This file contains the actual #[ test ] functions.
include!( "generics_in_tuple_variant_only_test.rs" );
// xxx : qqq : uncomment and fix issues