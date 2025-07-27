//! Purpose: Tests the `#[derive(Former)]` macro's generation of constructors for unnamed (tuple)
//! variants with shared generic parameters and bounds, using the default subform behavior.
//! This file focuses on verifying the derive-based implementation.
//!
//! Coverage:
//! - Rule 3d (Tuple + Single-Field + Default -> Subform): Verifies `EnumG3::<T>::v1() -> InnerG3Former<T>`.
//! - Rule 4b (Option 2 Logic): Verifies the use of the subformer returned by the variant constructor.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines a generic enum `EnumG3<T: BoundA + BoundB>` with a single-field tuple variant `V1(InnerG3<T>)`.
//! - The inner struct `InnerG3<T: BoundB>` has its own generic `T` and bound `BoundB`, and is instantiated with the enum's generic `T` in the variant.
//! - The enum has `#[derive(Former)]`.
//! - Relies on the derived static method `EnumG3::<T>::v_1()` provided by this file (via `include!`).
//! - Asserts that this constructor returns the expected subformer (`InnerG3Former<T>`) and that using the subformer's setter (`.inner_field()`) and `.form()` results in the correct `EnumG3` enum instance.
//! - Verifies that the bounds (`BoundA`, `BoundB`) are correctly handled by using a type that satisfies both.
#[ allow( unused_imports ) ]
use former::Former;
use super::*; // Imports testing infrastructure and potentially other common items

// --- Dummy Bounds ---
// Defined in _only_test.rs, but repeated here conceptually for clarity
// pub trait BoundA : core::fmt::Debug + Default + Clone + PartialEq {}
// pub trait BoundB : core::fmt::Debug + Default + Clone + PartialEq {}

// --- Inner Struct Definition with Bounds ---
// Needs to derive Former for the enum's derive to work correctly for subforming.
#[ derive( Debug, Clone, Default, PartialEq, former_meta::Former ) ]
pub struct InnerG3< T : BoundB > // BoundB required by the inner struct
{
  pub inner_field : T,
}

// --- Enum Definition with Bounds ---
// Apply Former derive here. This is what we are testing.
#[ derive( Debug, PartialEq, Clone, former_meta::Former ) ]
// #[ derive( Debug, PartialEq, Clone ) ]
// #[ debug ] // Uncomment to see generated code later
pub enum EnumG3< T : BoundA + BoundB > // BoundA required by enum, BoundB required by InnerG3<T>
{
  V1( InnerG3< T > ), // Inner type uses T
}

// --- Include the Test Logic ---
// This file contains the actual #[ test ] functions.
include!( "generics_shared_tuple_only_test.rs" );
