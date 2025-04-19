// File: module/core/former/tests/inc/former_enum_tests/generics_shared_struct_derive.rs
use super::*; // Imports testing infrastructure and potentially other common items

// --- Dummy Bounds ---
// Defined in _only_test.rs, but repeated here conceptually for clarity
// pub trait BoundA : core::fmt::Debug + Default + Clone + PartialEq {}
// pub trait BoundB : core::fmt::Debug + Default + Clone + PartialEq {}

// --- Inner Struct Definition with Bounds ---
// Needs to derive Former for the enum's derive to work correctly for subforming.
#[ derive( Debug, Clone, PartialEq, Default, former::Former ) ] // Added Default and Former
pub struct InnerG4< T : BoundB > // BoundB required by the inner struct
{
  pub inner_field : T,
}

// --- Enum Definition with Bounds ---
// Apply Former derive here. This is what we are testing.
#[ derive( Debug, PartialEq, Clone, former::Former ) ]
// #[ debug ] // Uncomment to see generated code later
pub enum EnumG4< T : BoundA + BoundB > // BoundA required by enum, BoundB required by InnerG4<T>
{
  V1 // Struct-like variant
  {
    // Field holding the generic inner struct
    inner : InnerG4< T >,
    // A non-generic field for testing
    flag : bool,
  },
}

// --- Include the Test Logic ---
// This file contains the actual #[test] functions.
include!( "generics_shared_struct_only_test.rs" );