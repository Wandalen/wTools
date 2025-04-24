// File: module/core/former/tests/inc/former_enum_tests/enum_named_fields_only_test.rs
use super::*; // Imports EnumWithNamedFields and InnerForSubform

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

// --- Zero Fields (Named) ---

#[ test ]
fn variant_zero_scalar_test()
{
  // Expect a direct static constructor taking no arguments.
  let got = EnumWithNamedFields::variant_zero_scalar();
  let expected = EnumWithNamedFields::VariantZeroScalar {};
  assert_eq!( got, expected );
}

// #[test]
// fn variant_zero_default_test() { /* Compile Error Expected */ }

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

// --- One Field (Named) ---

#[ test ]
fn variant_one_scalar_test()
{
  // Expect a direct static constructor taking one argument.
  let got = EnumWithNamedFields::variant_one_scalar( "value_a".to_string() );
  let expected = EnumWithNamedFields::VariantOneScalar { field_a : "value_a".to_string() };
  assert_eq!( got, expected );
}

#[ test ]
fn variant_one_subform_test()
{
  // Expect a static method returning a subformer for InnerForSubform.
  let got = EnumWithNamedFields::variant_one_subform()
    .value( 101 ) // Use InnerForSubformFormer's setter
    .form();
  let expected = EnumWithNamedFields::VariantOneSubform { field_b: InnerForSubform { value: 101 } };
  assert_eq!( got, expected );
}

#[ test ]
fn variant_one_default_test()
{
  // Expect a static method returning a subformer for InnerForSubform (default behavior).
  let got = EnumWithNamedFields::variant_one_default()
    .value( 102 ) // Use InnerForSubformFormer's setter
    .form();
  let expected = EnumWithNamedFields::VariantOneDefault { field_c: InnerForSubform { value: 102 } };
  assert_eq!( got, expected );
}

// --- Two Fields (Named) ---

#[ test ]
fn variant_two_scalar_test()
{
  // Expect a direct static constructor taking multiple arguments.
  let got = EnumWithNamedFields::variant_two_scalar( 42, true );
  let expected = EnumWithNamedFields::VariantTwoScalar { field_d : 42, field_e : true };
  assert_eq!( got, expected );
}

// #[test]
// fn variant_two_default_test() { /* Compile Error Expected */ }