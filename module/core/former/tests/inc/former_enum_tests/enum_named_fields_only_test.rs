use super::*; // Imports EnumWithNamedFields

#[ test ]
fn variant_zero_fields()
{
  // Expect a static method `variant_zero()` returning a former with no setters.
  let got = EnumWithNamedFields::variant_zero()
  .form(); // .form() calls the End struct's logic

  let expected = EnumWithNamedFields::VariantZero {};
  assert_eq!( got, expected );
}

#[ test ]
fn variant_one_field()
{
  // Expect a static method `variant_one()` returning a former with a `.field_a()` setter.
  let got = EnumWithNamedFields::variant_one()
  .field_a( "value_a".to_string() )
  .form();

  let expected = EnumWithNamedFields::VariantOne
  {
    field_a : "value_a".to_string(),
  };
  assert_eq!( got, expected );
}

#[ test ]
fn variant_two_fields()
{
  // Expect a static method `variant_two()` returning a former with `.field_b()` and `.field_c()` setters.
  let got = EnumWithNamedFields::variant_two()
  .field_b( 42 )
  .field_c( true )
  .form();

  let expected = EnumWithNamedFields::VariantTwo
  {
    field_b : 42,
    field_c : true,
  };
  assert_eq!( got, expected );
}

#[ test ]
fn unit_variant_construction()
{
  // Ensure the unit variant constructor still works.
  let got = EnumWithNamedFields::unit_variant();
  let expected = EnumWithNamedFields::UnitVariant;
  assert_eq!( got, expected );
}