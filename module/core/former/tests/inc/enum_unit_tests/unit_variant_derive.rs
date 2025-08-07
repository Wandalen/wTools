//! Purpose: Tests the `#[derive(Former)]` macro's generation of constructors for unit variants,
//! including with `#[standalone_constructors]`. This file focuses on verifying the derive-based implementation.
//!
//! Coverage:
//! - Rule 3a (Unit + Default): Verifies `Enum::variant() -> Enum`.
//! - Rule 1a (Unit + `#[scalar]`): Verifies `Enum::variant() -> Enum` (as default for unit is scalar).
//! - Rule 4a (`#[standalone_constructors]`): Verifies generation of top-level constructor functions.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `Status` with unit variants `Pending` and `Complete`, and the `#[former( standalone_constructors )]` attribute.
//! - Relies on the derived static methods (`Status::pending()`, `Status::complete()`) and standalone functions (`pending()`, `complete()`) defined in `unit_variant_only_test.rs`.
//! - Asserts that these constructors produce the correct `Status` enum instances by comparing with manually constructed variants.
// File: module/core/former/tests/inc/former_enum_tests/unit_variant_derive.rs
use super::*;
#[allow(unused_imports)]
use ::former::prelude::*;
use ::former::Former; // Import derive macro

/// Enum with only unit variants for testing.
#[derive(Debug, PartialEq, Former)]
#[standalone_constructors] // Added standalone_constructors attribute
#[allow(dead_code)] // Enum itself might not be directly used, but its Former methods are
pub enum Status {
  Pending,
  Complete,
}

// Include the test logic
include!("unit_variant_only_test.rs");
