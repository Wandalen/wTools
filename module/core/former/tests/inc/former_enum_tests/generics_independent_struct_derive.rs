// File: module/core/former/tests/inc/former_enum_tests/generics_independent_struct_derive.rs

//! # Derive Test: Independent Generics in Struct Variants
//!
//! This test file focuses on verifying the `#[derive(Former)]` macro's ability to handle
//! enums with struct-like variants where the generics involved are independent.
//! Specifically, it tests an enum `EnumG6<T>` where a variant `V1` contains a field
//! whose type uses a *concrete* type (`InnerG6<TypeForU>`) unrelated to the enum's `T`.
//!
//! ## Purpose:
//!
//! - To ensure the derive macro correctly generates the implicit former infrastructure
//!   (storage, definitions, former struct, end struct) for the struct variant `V1`.
//! - To verify that the generated code correctly handles the enum's generic parameter `T`
//!   and its bounds (`BoundA`) where necessary (e.g., in the `End` struct and its `impl`).
//! - To confirm that the generated setters within the implicit former work for fields
//!   containing concrete types like `InnerG6<TypeForU>`.
//! - It uses the shared test logic from `generics_independent_struct_only_test.rs`.

use super::*; // Imports testing infrastructure and potentially other common items
// FIX: Import PhantomData as it's now needed in the enum definition

// --- Dummy Bounds and Concrete Types ---
// Are defined in the included _only_test.rs file

// --- Inner Struct Definition ---
// Also defined in the included _only_test.rs file,
// but conceptually needed here for the enum definition.
// #[ derive( Debug, Clone, PartialEq, Default, former::Former ) ]
// pub struct InnerG6< U : BoundB > { pub inner_field : U }

// --- Enum Definition with Bounds ---
// Apply Former derive here. This is what we are testing.
#[ derive( Debug, PartialEq, Clone, former::Former ) ]
// #[ debug ] // Uncomment to see generated code later
pub enum EnumG6< T : BoundA > // BoundA required by enum
{
  V1 // Struct-like variant
  {
    // Field holding the inner struct instantiated with a *concrete* type
    inner : InnerG6< TypeForU >, // TypeForU satisfies BoundB implicitly via _only_test.rs
    // A non-generic field for testing
    flag : bool,
    // FIX: Added PhantomData to use the generic parameter T, resolving E0392
    _phantom_t : std::marker::PhantomData<T>,
  },
}

// --- Include the Test Logic ---
// This file contains the actual #[ test ] functions.
include!( "generics_independent_struct_only_test.rs" );