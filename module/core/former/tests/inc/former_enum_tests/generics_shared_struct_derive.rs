// File: module/core/former/tests/inc/former_enum_tests/generics_shared_struct_derive.rs

//! # Derive Test: Shared Generics in Struct Variants
//!
//! This test file focuses on verifying the `#[derive(Former)]` macro's ability to handle
//! enums with struct-like variants where the generic parameter is shared between the enum
//! and a field within the variant.
//! Specifically, it tests an enum `EnumG4<T>` where a variant `V1` contains a field
//! whose type uses the *same* generic parameter `T` (`InnerG4<T>`).
//!
//! ## Purpose:
//!
//! - To ensure the derive macro correctly generates the implicit former infrastructure
//!   (storage, definitions, former struct, end struct) for the struct variant `V1`.
//! - To verify that the generated code correctly handles the shared generic parameter `T`
//!   and its bounds (`BoundA`, `BoundB`) throughout the generated types and implementations.
//! - To confirm that the generated setters within the implicit former work for fields
//!   containing generic types like `InnerG4<T>`.
//! - It uses the shared test logic from `generics_shared_struct_only_test.rs`.

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
    inner : InnerG4< T >,
    flag : bool,
  },
}

// --- Include the Test Logic ---
// This file contains the actual #[ test ] functions.
include!( "generics_shared_struct_only_test.rs" );
// qqq : xxx : uncomment please