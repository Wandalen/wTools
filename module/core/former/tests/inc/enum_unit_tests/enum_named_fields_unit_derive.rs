// File: module/core/former/tests/inc/former_enum_tests/unit_tests/enum_named_fields_unit_derive.rs
use super::*;

// Define the enum with unit variants for testing.
#[ derive( Debug, PartialEq, former::Former ) ]
#[ debug ]
#[ standalone_constructors ]
pub enum EnumWithNamedFields
{
  // --- Unit Variant ---
  // Expect: unit_variant_default() -> Enum (Default is scalar for unit)
  UnitVariantDefault, // Renamed from UnitVariant
  #[ scalar ] // Expect: unit_variant_scalar() -> Enum
  UnitVariantScalar, // New
}

// Include the test logic file
include!( "enum_named_fields_unit_only_test.rs" );