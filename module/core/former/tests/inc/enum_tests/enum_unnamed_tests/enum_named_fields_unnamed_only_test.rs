// File: module/core/former/tests/inc/former_enum_tests/unnamed_tests/enum_named_fields_unnamed_only_test.rs
use super::*; // Imports EnumWithNamedFields

// --- Zero Fields (Unnamed) ---

#[ test ]
fn variant_zero_unnamed_scalar_test() // New Test
{
  // Expect a direct static constructor taking no arguments.
  let got = EnumWithNamedFields::variant_zero_unnamed_scalar();
  let expected = EnumWithNamedFields::VariantZeroUnnamedScalar();
  assert_eq!( got, expected );
}

#[ test ]
fn variant_zero_unnamed_default_test() // New Test
{
  // Expect a direct static constructor taking no arguments (default is scalar).
  let got = EnumWithNamedFields::variant_zero_unnamed_default();
  let expected = EnumWithNamedFields::VariantZeroUnnamedDefault();
  assert_eq!( got, expected );
}