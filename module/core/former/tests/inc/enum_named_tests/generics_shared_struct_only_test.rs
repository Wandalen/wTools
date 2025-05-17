// File: module/core/former/tests/inc/former_enum_tests/generics_shared_struct_only_test.rs
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
fn shared_generics_struct_variant()
{
  //! Tests the construction of a struct variant (`V1`) where the inner field (`inner`)
  //! uses a generic type (`InnerG4<T>`) that shares the enum's generic parameter (`T`).
  //! It verifies that the implicit former's setters for both the generic inner field
  //! and the simple `flag` field work correctly.

  // CORRECTED: Use v_1() instead of v1()
  let inner_val = InnerG4::< MyType > { inner_field : MyType( 42 ) };
  let got = EnumG4::< MyType >::v_1() // Expects static method `v_1` returning the implicit former
    .inner( inner_val.clone() )      // Use the `inner` setter
    .flag( true )                    // Use the `flag` setter
    .form();                         // Calls the specialized End struct
                                     // qqq : xxx : check if this test is correct

  let expected_inner = InnerG4::< MyType > { inner_field : MyType( 42 ) };
  let expected = EnumG4::< MyType >::V1 { inner : expected_inner, flag : true }; // Construct expected enum

  assert_eq!( got, expected );
}

#[ test ]
fn default_construction_shared_struct_variant()
{
  //! Tests the construction of a struct variant (`V1`) relying on the `Default`
  //! implementation for the inner field (`inner`) which has a generic type (`InnerG4<T>`).
  //! It verifies that the implicit former correctly uses the default value when the setter is not called.

  // Test that default construction works if the inner type has defaults
  // CORRECTED: Use v_1() instead of v1()
  let got = EnumG4::< MyType >::v_1()
    // .inner is not called, relying on default
    .flag( false ) // Set the non-generic field
    .form();
                                     // qqq : xxx : check if this test is correct

  let expected_inner = InnerG4::< MyType > { inner_field : MyType::default() }; // Expect default inner
  // Construct expected enum with default inner and specified flag
  let expected = EnumG4::< MyType >::V1 { inner : expected_inner, flag : false };

  assert_eq!( got, expected );
}