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

// Include the test logic file
include!( "enum_named_fields_unnamed_only_test.rs" );