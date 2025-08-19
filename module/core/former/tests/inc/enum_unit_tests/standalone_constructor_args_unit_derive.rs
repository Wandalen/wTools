#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Purpose: Tests the `#[ derive( Former ) ]` macro's generation of standalone constructors for unit variants
//! within an enum that also has the `#[ standalone_constructors ]` attribute. This file focuses on verifying
//! the derive-based implementation.
//!
//! Coverage:
//! - Rule 3a (Unit + Default): Covered by the default behavior of unit variants.
//! - Rule 1a (Unit + `#[ scalar ]`): Unit variants implicitly behave as scalar.
//! - Rule 4a (#[ standalone_constructors ]): Verifies the generation of a top-level constructor function.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines a unit variant `UnitVariantArgs` in `TestEnumArgs` with `#[ derive( Former ) ]` and `#[ standalone_constructors ]` on the enum.
//! - Relies on the shared test logic in `standalone_constructor_args_unit_only_test.rs` which invokes the generated standalone constructor `unit_variant_args()`.
//! - Asserts that the result matches the direct enum variant `TestEnumArgs::UnitVariantArgs`, confirming the constructor produces the correct variant instance.

#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former; // Import derive macro

// === Enum Definition ===

/// Enum using derive for standalone constructors with arguments.
#[ derive( Debug, PartialEq, Clone, Former, debug ) ] // Added debug attribute
#[ standalone_constructors ] // Enable standalone constructors
pub enum TestEnumArgs // Use the distinct name
{
  /// A unit variant.
  UnitVariantArgs, // Use the distinct name
}

// === Include Test Logic ===
include!( "standalone_constructor_args_unit_only_test.rs" ); // Include the specific test file