// File: module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_only_test.rs
use super::*; // Imports items from the parent file (either manual or derive)

// Define dummy bounds for testing purposes
pub trait BoundA : core::fmt::Debug + Default + Clone + PartialEq {}
pub trait BoundB : core::fmt::Debug + Default + Clone + PartialEq {}

// Define a concrete type that satisfies the bounds
#[ derive( Debug, Default, Clone, PartialEq ) ]
pub struct MyType( i32 );
impl BoundA for MyType {}
impl BoundB for MyType {}

#[ test ]
fn shared_generics_tuple_variant()
{
  // Instantiate the enum using the static method for the variant
  let got = EnumG3::< MyType >::v_1() // Expects static method `v1`
    .inner_field( MyType( 42 ) )     // Use setter from InnerG3Former
    .form();                         // Calls the specialized End struct

  // Define the expected result
  let expected_inner = InnerG3::< MyType > { inner_field : MyType( 42 ) };
  let expected = EnumG3::< MyType >::V1( expected_inner );

  assert_eq!( got, expected );
}

#[ test ]
fn default_construction()
{
  // Test that default construction works if the inner type has defaults
  let got = EnumG3::< MyType >::v_1()
    .form(); // Rely on default for inner_field

  let expected_inner = InnerG3::< MyType > { inner_field : MyType::default() }; // Expect default inner
  let expected = EnumG3::< MyType >::V1( expected_inner );

  assert_eq!( got, expected );
}