// File: module/core/former/tests/inc/former_enum_tests/unit_tests/standalone_constructor_args_unit_manual.rs

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