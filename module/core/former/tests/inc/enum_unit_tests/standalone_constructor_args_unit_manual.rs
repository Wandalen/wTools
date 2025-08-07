//! Purpose: Provides a manual implementation of the standalone constructor for a unit variant within an enum,
//! corresponding to the derive-based test in `standalone_constructor_args_unit_derive.rs`. This file verifies
//! the expected behavior of the manual implementation.
//!
//! Coverage:
//! - Rule 3a (Unit + Default): Covered by the default behavior of unit variants.
//! - Rule 1a (Unit + `#[scalar]`): Unit variants implicitly behave as scalar.
//! - Rule 4a (#[standalone_constructors]): Verifies the manual implementation of a top-level constructor function.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines a unit variant `UnitVariantArgs` in `TestEnumArgs`.
//! - Manually implements the standalone constructor function `unit_variant_args()` which returns `TestEnumArgs::UnitVariantArgs`.
//! - Relies on the shared test logic in `standalone_constructor_args_unit_only_test.rs` which invokes the manual standalone constructor `unit_variant_args()`.
//! - Asserts that the result matches the direct enum variant `TestEnumArgs::UnitVariantArgs`, confirming the constructor produces the correct variant instance.

#[ allow( unused_imports ) ]
use ::former::prelude::*;
#[ allow( unused_imports ) ]
use ::former_types::
{
  Storage, StoragePreform,
  FormerDefinitionTypes, FormerMutator, FormerDefinition,
  FormingEnd, ReturnPreformed,
};

// === Enum Definition ===

/// Enum for manual testing of standalone constructors with arguments.
#[ derive( Debug, PartialEq, Clone ) ]
pub enum TestEnumArgs // New name
{
  /// A unit variant.
  UnitVariantArgs, // New name
}

// === Standalone Constructors (Manual - Argument Taking) ===

/// Manual standalone constructor for TestEnumArgs::UnitVariantArgs.
pub fn unit_variant_args() -> TestEnumArgs
{
  TestEnumArgs::UnitVariantArgs
}

// === Include Test Logic ===
include!( "standalone_constructor_args_unit_only_test.rs" );