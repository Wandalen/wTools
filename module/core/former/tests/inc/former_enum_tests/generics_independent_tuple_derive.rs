//! Derive-based test for enum variants with independent generic parameters.
//!
//! Purpose:
//! - Define `EnumG5<T: BoundA>` and `InnerG5<U: BoundB>` with independent generics.
//! - Apply `#[derive(Former)]` to both the enum and the inner struct.
//! - Use the included `_only_test.rs` file to verify that the macro-generated code
//!   correctly handles the distinct generics `T` and `U` (instantiated as `TypeForU`
//!   in the variant) and their respective bounds.

// File: module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_derive.rs
use super::*; // Imports testing infrastructure and potentially other common items

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

// --- Enum Definition with Bounds ---
// Apply Former derive here. This is what we are testing.
#[ derive( Debug, PartialEq, Clone, former::Former ) ]
// #[ debug ] // Uncomment to see generated code later
pub enum EnumG5< T : BoundA > // BoundA required by the enum
{
  // Variant holds InnerG5 instantiated with the *concrete* TypeForU
  // The macro needs to handle this fixed inner type correctly while keeping T generic.
  V1( InnerG5< TypeForU > ),
  // REMOVED: Manual PhantomData variant workaround
  // _Phantom( core::marker::PhantomData< T > ),
}

// --- Include the Test Logic ---
// This file contains the actual #[ test ] functions.
include!( "generics_independent_tuple_only_test.rs" );
// xxx : qqq : uncomment and fix issues