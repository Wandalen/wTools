#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Purpose: Tests the `#[ derive( Former ) ]` macro's generation of a former builder for a named
//! (struct-like) variant (`V1`) within a generic enum (`EnumG4<T>`), where the variant contains
//! a field with a shared generic type (`InnerG4<T>`). This file focuses on verifying the
//! derive-based implementation's handling of shared generics and the generation of appropriate
//! setters in the implicit former.
//!
//! Coverage:
//! - Rule 3g (Struct + Multi-Field + Default): Verifies that for a named variant without specific attributes, the derived constructor is a former builder (`v_1()` returns a former).
//! - Rule 4b (Option 2 Logic): Demonstrates the usage of the former builder's setters (`.inner()`, `.flag()`) and `.form()` method, verifying the subformer mechanism in the context of shared generics.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines a generic enum `EnumG4<T: BoundA + BoundB>` with a named variant `V1 { inner: InnerG4<T>, flag: bool }`.
//! - Defines the inner struct `InnerG4<T: BoundB>` which also derives `Former`.
//! - Defines dummy bounds (`BoundA`, `BoundB`) and a concrete type (`MyType`) in the included test file.
//! - Applies `#[ derive( Former ) ]` to both `EnumG4` and `InnerG4`.
//! - Includes shared test logic from `generics_shared_struct_only_test.rs`.
//! - The included tests call the derived static method `EnumG4::<MyType>::v_1()`, use the returned former's setters (`.inner()`, `.flag()`), and call `.form()`.
//! - Asserts that the resulting enum instances match manually constructed expected values. This verifies that the derived former builder correctly handles fields with shared generic types and non-generic fields within a generic enum.

// File: module/core/former/tests/inc/former_enum_tests/generics_shared_struct_derive.rs

//! # Derive Test: Shared Generics in Struct Variants
//!
//! This test file focuses on verifying the `#[ derive( Former ) ]` macro's ability to handle
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