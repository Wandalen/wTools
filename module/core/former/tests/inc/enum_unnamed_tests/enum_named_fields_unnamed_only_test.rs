// Purpose: Tests enum with named fields in unnamed context
// This file is included by enum_named_fields_unnamed derive/manual files

#[ test ]
fn enum_named_fields_test()
{
  // Test the zero-field scalar variants
  let got_scalar = EnumWithNamedFields::variant_zero_unnamed_scalar();
  let expected_scalar = EnumWithNamedFields::VariantZeroUnnamedScalar();
  assert_eq!( got_scalar, expected_scalar );

  let got_default = EnumWithNamedFields::variant_zero_unnamed_default();
  let expected_default = EnumWithNamedFields::VariantZeroUnnamedDefault();
  assert_eq!( got_default, expected_default );
}