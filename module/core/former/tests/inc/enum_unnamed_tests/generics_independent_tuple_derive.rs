//! Purpose: Tests the `#[derive(Former)]` macro's generation of constructors for unnamed (tuple)
//! variants with independent generic parameters and bounds, specifically when the variant
//! is marked with `#[scalar]`. This file focuses on verifying the derive-based implementation.
//!
//! Coverage:
//! - Rule 1d (Tuple + Single-Field + `#[scalar]` -> Scalar): Verifies `EnumG5::<T>::v1() -> EnumG5<T>`.
//! - Rule 4a (#[standalone_constructors]): Verifies generation of top-level constructor functions (though not explicitly tested in `_only_test.rs`).
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines a generic enum `EnumG5<T: BoundA>` with a single-field tuple variant `V1(InnerG5<TypeForU>, PhantomData<T>)`.
//! - The inner struct `InnerG5<U: BoundB>` has its own generic `U` and bound `BoundB`, and is instantiated with a concrete `TypeForU` in the variant.
//! - The variant `V1` is annotated with `#[scalar]`. The enum has `#[derive(Former)]`.
//! - Relies on the derived static method `EnumG5::<TypeForT>::v_1()` defined in `generics_independent_tuple_only_test.rs`.
//! - Asserts that this constructor produces the correct `EnumG5` enum instance by comparing with a manually constructed variant, confirming correct handling of independent generics and the `#[scalar]` attribute.
use super::*; // Imports testing infrastructure and potentially other common items
use std::marker::PhantomData;

// --- Dummy Bounds ---
// Defined in _only_test.rs
// pub trait BoundA : core::fmt::Debug + Default + Clone + PartialEq {}
// pub trait BoundB : core::fmt::Debug + Default + Clone + PartialEq {}

// --- Concrete Types ---
// Defined in _only_test.rs
// pub struct TypeForT( String ); impl BoundA for TypeForT {}
// pub struct TypeForU( i32 );    impl BoundB for TypeForU {}

// --- Inner Struct Definition with Bounds ---
// Needs to derive Former for the enum's derive to work correctly for subforming.
#[ derive( Debug, Clone, PartialEq, Default, former::Former ) ] // Added Default and Former
pub struct InnerG5< U : BoundB > // BoundB required by the inner struct
{
  pub inner_field : U,
}

// Implement Into manually for testing the constructor signature
impl< U : BoundB > From< U > for InnerG5< U >
{
  fn from( data : U ) -> Self { Self { inner_field : data } }
}

// --- Enum Definition with Bounds ---
// Apply Former derive here. This is what we are testing.
#[ derive( Debug, PartialEq, Clone, former::Former ) ]
// #[ debug ] // Uncomment to see generated code later
pub enum EnumG5< T : BoundA > // BoundA required by the enum
{
  // Variant holds InnerG5 instantiated with the *concrete* TypeForU
  // The macro needs to handle this fixed inner type correctly while keeping T generic.
  #[ scalar ]
  V1( InnerG5< TypeForU >, core::marker::PhantomData< T > ),
}

// --- Include the Test Logic ---
// This file contains the actual #[ test ] functions.
include!( "generics_independent_tuple_only_test.rs" );
// xxx : qqq : uncomment and fix issues