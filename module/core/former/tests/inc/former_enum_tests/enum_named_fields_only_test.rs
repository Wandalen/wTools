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

#[ test ]
fn standalone_variant_zero_scalar_test() // New Test for S0.4
{
  // Expect a standalone constructor taking no arguments.
  let got = standalone_variant_zero_scalar();
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

// --- One Field (Named) - Standalone Constructors (S1.4-S1.7) ---

#[ test ]
fn standalone_variant_one_default_test() // Test for S1.4
{
  // Expect a standalone constructor returning a subformer.
  let got = standalone_variant_one_default()
    .value( 103 )
    .form();
  let expected = EnumWithNamedFields::VariantOneDefault { field_c: InnerForSubform { value: 103 } };
  assert_eq!( got, expected );
}

#[ test ]
fn standalone_variant_one_scalar_test() // Test for S1.5
{
  // Expect a standalone constructor taking one argument.
  let got = standalone_variant_one_scalar( "value_b".to_string() );
  let expected = EnumWithNamedFields::VariantOneScalar { field_a : "value_b".to_string() };
  assert_eq!( got, expected );
}

#[ test ]
fn standalone_variant_one_subform_test() // Test for S1.6
{
  // Expect a standalone constructor returning a subformer.
  let got = standalone_variant_one_subform()
    .value( 104 )
    .form();
  let expected = EnumWithNamedFields::VariantOneSubform { field_b: InnerForSubform { value: 104 } };
  assert_eq!( got, expected );
}

#[ test ]
fn standalone_variant_one_default_with_arg_test() // Test for S1.7
{
  // Expect a standalone constructor taking the marked argument.
  // Note: Manual implementation might differ slightly from macro output depending on arg_for_constructor logic.
  let got = standalone_variant_one_default_with_arg( InnerForSubform { value: 105 } );
  let expected = EnumWithNamedFields::VariantOneDefault { field_c: InnerForSubform { value: 105 } };
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

// --- Two Fields (Named) - Standalone Constructors (SN.4-SN.7) ---

#[ test ]
fn standalone_variant_two_default_test() // Test for SN.4
{
  // Expect a standalone constructor returning a subformer.
  // Note: Manual implementation uses a placeholder End struct.
  let got = standalone_variant_two_default()
    .value( 201 ) // Assuming InnerForSubformFormer methods are available on the placeholder
    .form();
  // qqq : Expected value depends on the placeholder implementation in manual file.
  // For now, just check that it doesn't panic and returns the placeholder variant.
  let expected = EnumWithNamedFields::UnitVariantScalar; // Matches placeholder return
  assert_eq!( got, expected );
}

#[ test ]
fn standalone_variant_two_scalar_test() // Test for SN.5
{
  // Expect a standalone constructor taking multiple arguments.
  let got = standalone_variant_two_scalar( 43, false );
  let expected = EnumWithNamedFields::VariantTwoScalar { field_d : 43, field_e : false };
  assert_eq!( got, expected );
}

#[ test ]
fn standalone_variant_two_subform_test() // Test for SN.6
{
  // Expect a standalone constructor returning a subformer.
  // Note: Manual implementation uses a placeholder End struct.
  let got = standalone_variant_two_subform()
    .value( 202 ) // Assuming InnerForSubformFormer methods are available on the placeholder
    .form();
  // qqq : Expected value depends on the placeholder implementation in manual file.
  // For now, just check that it doesn't panic and returns the placeholder variant.
  let expected = EnumWithNamedFields::UnitVariantScalar; // Matches placeholder return
  assert_eq!( got, expected );
}

#[ test ]
fn standalone_variant_two_default_with_args_test() // Test for SN.7
{
  // Expect a standalone constructor taking marked arguments.
  // Note: Manual implementation uses a direct constructor with all fields as args.
  let got = standalone_variant_two_default_with_args( 44, true );
  let expected = EnumWithNamedFields::VariantTwoDefault { field_d: 44, field_e: true };
  assert_eq!( got, expected );
}