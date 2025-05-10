// Test Matrix Row: T0.1 (Default, None)
#[ test ]
fn test_zero_field_default()
{
  use super::*;
  let got = EnumWithZeroFieldTuple::variant_zero_default();
  let expected = EnumWithZeroFieldTuple::VariantZeroDefault;
  assert_eq!( got, expected );
}

// Test Matrix Row: T0.2 (#[scalar], None)
#[ test ]
fn test_zero_field_scalar()
{
  use super::*;
  let got = EnumWithZeroFieldTuple::variant_zero_scalar();
  let expected = EnumWithZeroFieldTuple::VariantZeroScalar;
  assert_eq!( got, expected );
}

// Test Matrix Row: T0.3 (Default, #[standalone_constructors])
#[ test ]
fn test_zero_field_default_standalone()
{
  use super::*;
  let got = standalone_variant_zero_default();
  let expected = EnumWithZeroFieldTuple::VariantZeroDefault;
  assert_eq!( got, expected );
}

// Test Matrix Row: T0.4 (#[scalar], #[standalone_constructors])
#[ test ]
fn test_zero_field_scalar_standalone()
{
  use super::*;
  let got = standalone_variant_zero_scalar();
  let expected = EnumWithZeroFieldTuple::VariantZeroScalar;
  assert_eq!( got, expected );
}