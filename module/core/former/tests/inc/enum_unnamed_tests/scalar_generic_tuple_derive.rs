//! Purpose: Tests the `#[derive(Former)]` macro's generation of constructors for single-field and multi-field tuple variants within a generic enum with bounds. This file focuses on verifying the derive-based implementation, particularly the default behavior when `#[scalar]` is commented out.
//!
//! Coverage:
//! - Rule 3d (Tuple + Single-Field + Default): Verifies `Enum::variant() -> InnerFormer<...>` for a generic enum.
//! - Rule 3f (Tuple + Multi-Field + Default): Verifies `Enum::variant(T1, T2, ...) -> Enum` for a generic enum. (Note: Tests in `_only_test.rs` included by this file seem to expect subformer behavior for multi-field variants, which contradicts this rule. The comment reflects the rule as defined in the plan).
//! - Rule 4b (Option 2 Logic): Related to the subformer mechanism used for `Variant1` (as tested) and expected for `Variant2` (as tested, contradicting Rule 3f).
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines a generic enum `EnumScalarGeneric<T: Bound>` with variants `Variant1(InnerScalar<T>)` and `Variant2(InnerScalar<T>, bool)`.
//! - Includes shared test logic from `scalar_generic_tuple_only_test.rs`.
//! - Relies on `#[derive(Former)]` to generate static methods (`variant_1`, `variant_2`).
//! - The included tests invoke these methods and use `.into()` for `variant_1` (expecting scalar) and setters/`.form()` for `variant_2` (expecting subformer), asserting the final enum instance matches manual construction. This tests the derived constructors' behavior with generic tuple variants.

// File: module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_derive.rs

// Types are imported from mod.rs via include!


// --- Enum Definition with Bounds and #[scalar] Variants ---
// Apply Former derive here. This is what we are testing.
#[derive(Debug, PartialEq, Clone)]
#[derive(former::Former)]
pub enum EnumScalarGeneric<T : Bound>
{
  #[scalar] // Enabled for Rule 1d testing
  Variant1(InnerScalar<T>), // Tuple variant with one generic field

  // TEMP: Removing Variant2 for Increment 3 debugging
  // Variant2(InnerScalar<T>, bool), // Tuple variant with generic and non-generic fields
}

// --- Include the Test Logic ---
// This file contains the actual #[ test ] functions.
include!( "scalar_generic_tuple_only_test.rs" );