//! Purpose: Provides a manual implementation of constructors for an enum with zero-field
//! unnamed (tuple) variants using named fields syntax, including static methods, to serve
//! as a reference for verifying the `#[derive(Former)]` macro's behavior.
//!
//! Coverage:
//! - Rule 3b (Tuple + Zero-Field + Default): Manual implementation of static method `EnumWithNamedFields::variant_zero_unnamed_default()`.
//! - Rule 1b (Tuple + Zero-Field + `#[scalar]`): Manual implementation of static method `EnumWithNamedFields::variant_zero_unnamed_scalar()`.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `EnumWithNamedFields` with two zero-field unnamed variants: `VariantZeroUnnamedDefault()` and `VariantZeroUnnamedScalar()`.
//! - Manually implements static methods (`EnumWithNamedFields::variant_zero_unnamed_scalar()`, `EnumWithNamedFields::variant_zero_unnamed_default()`)
//!   that mirror the expected generated code for scalar zero-field variants.
//! - This file is included by `enum_named_fields_unnamed_only_test.rs` to provide the manual implementations
//!   that the shared tests compare against.
// File: module/core/former/tests/inc/former_enum_tests/unnamed_tests/enum_named_fields_unnamed_manual.rs
use super::*;
use former::
{
  FormingEnd, StoragePreform, FormerDefinition, FormerDefinitionTypes, Storage,
  ReturnPreformed, FormerBegin, FormerMutator,
};
use std::marker::PhantomData;

// Define the enum with zero-field unnamed (tuple) variants for manual testing.
#[ derive( Debug, PartialEq ) ]
pub enum EnumWithNamedFields
{
  // --- Zero Fields (Unnamed - Tuple-like) ---
  VariantZeroUnnamedScalar(), // New
  VariantZeroUnnamedDefault(), // New
}

// --- Manual implementation of static methods on the Enum ---
impl EnumWithNamedFields
{
  // --- Zero Fields (Unnamed - Tuple-like) ---
  #[ inline( always ) ]
  pub fn variant_zero_unnamed_scalar() -> Self { Self::VariantZeroUnnamedScalar() } // New
  #[ inline( always ) ]
  pub fn variant_zero_unnamed_default() -> Self { Self::VariantZeroUnnamedDefault() } // New (Default is scalar)
}

// --- FormingEnd Implementations for End Structs ---

// End for Break variant
impl former::FormingEnd
<
  BreakFormerDefinitionTypes< (), FunctionStep > // Context is (), Formed is FunctionStep
>
for FunctionStepBreakEnd
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    sub_storage : BreakFormerStorage, // Storage of the inner type (Break)
    _context : Option< () >,          // Context is () from ::begin
  ) -> FunctionStep                   // Returns the Enum type
  {
    let data = sub_storage.preform(); // Get the Break data
    FunctionStep::Break( data )       // Construct the enum variant
  }
}

// End for Run variant
impl former::FormingEnd
<
  RunFormerDefinitionTypes< (), FunctionStep > // Context is (), Formed is FunctionStep
>
for FunctionStepRunEnd
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    sub_storage : RunFormerStorage, // Storage of the inner type (Run)
    _context : Option< () >,        // Context is () from ::begin
  ) -> FunctionStep                 // Returns the Enum type
  {
    let data = sub_storage.preform(); // Get the Run data
    FunctionStep::Run( data )         // Construct the enum variant
  }
}

// Include the test logic file
include!( "enum_named_fields_unnamed_only_test.rs" );