// File: module/core/former/tests/inc/former_enum_tests/unit_tests/enum_named_fields_unit_only_test.rs
use super::*; // Imports EnumWithNamedFields

// --- Unit Variant ---

#[ test ]
fn unit_variant_scalar_test() // New Test
{
  // Expect a direct static constructor taking no arguments.
  let got = EnumWithNamedFields::unit_variant_scalar();
  let expected = EnumWithNamedFields::UnitVariantScalar;
  assert_eq!( got, expected );
}

#[ test ]
fn unit_variant_default_construction() // Renamed Test
{
  // Expect a direct static constructor taking no arguments (default is scalar).
  let got = EnumWithNamedFields::unit_variant_default();
  let expected = EnumWithNamedFields::UnitVariantDefault;
  assert_eq!( got, expected );
}