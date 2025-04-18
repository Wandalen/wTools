// File: module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_derive.rs
use super::*; // Imports testing infrastructure and potentially other common items

// --- Dummy Bounds ---
// Defined in _only_test.rs, but repeated here conceptually for clarity
// pub trait BoundA : core::fmt::Debug + Default + Clone + PartialEq {}
// pub trait BoundB : core::fmt::Debug + Default + Clone + PartialEq {}

// --- Inner Struct Definition with Bounds ---
// Needs to derive Former for the enum's derive to work correctly for subforming.
#[ derive( Debug, Clone, PartialEq, former::Former ) ]
pub struct InnerG3< T : BoundB > // BoundB required by the inner struct
{
  pub inner_field : T,
}

// --- Enum Definition with Bounds ---
// Apply Former derive here. This is what we are testing.
#[ derive( Debug, PartialEq, Clone, former::Former ) ]
// #[ debug ] // Uncomment to see generated code later
pub enum EnumG3< T : BoundA + BoundB > // BoundA required by enum, BoundB required by InnerG3<T>
{
  V1( InnerG3< T > ), // Inner type uses T
}

// --- Include the Test Logic ---
// This file contains the actual #[test] functions.
include!( "generics_shared_tuple_only_test.rs" );

// qqq : xxx : enable