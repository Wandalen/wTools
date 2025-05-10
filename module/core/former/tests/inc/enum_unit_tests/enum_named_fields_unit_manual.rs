// File: module/core/former/tests/inc/former_enum_tests/unit_tests/enum_named_fields_unit_manual.rs
use super::*;
use former::
{
  FormingEnd, StoragePreform, FormerDefinition, FormerDefinitionTypes, Storage,
  ReturnPreformed, FormerBegin, FormerMutator,
};
use std::marker::PhantomData;

// Define the enum with unit variants for manual testing.
#[ derive( Debug, PartialEq ) ]
pub enum EnumWithNamedFields
{
  // --- Unit Variant ---
  UnitVariantScalar, // New
  UnitVariantDefault, // Renamed
}

// --- Manual implementation of static methods on the Enum ---
impl EnumWithNamedFields
{
  // --- Unit Variant ---
  #[ inline( always ) ]
  pub fn unit_variant_scalar() -> Self { Self::UnitVariantScalar } // New
  #[ inline( always ) ]
  pub fn unit_variant_default() -> Self { Self::UnitVariantDefault } // Renamed (Default is scalar)
}

// Include the test logic file
include!( "enum_named_fields_unit_only_test.rs" );