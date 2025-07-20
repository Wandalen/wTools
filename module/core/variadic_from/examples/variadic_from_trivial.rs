// variadic_from_trivial.rs

//! This example demonstrates the use of the `VariadicFrom` derive macro.
//! It allows a struct with a single field to automatically implement the `From` trait
//! for multiple source types, as specified by `#[from(Type)]` attributes.

#[ cfg( not( all(feature = "enabled", feature = "type_variadic_from", feature = "derive_variadic_from" ) ) ) ]
fn main(){}
#[ cfg( all( feature = "enabled", feature = "type_variadic_from", feature = "derive_variadic_from" ) )]
fn main()
{
  use variadic_from::exposed::*;
  use variadic_from_meta::VariadicFrom;

  // Define a struct `MyStruct` with a single field `value`.
  // It derives common traits and `VariadicFrom`.
  #[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
  struct MyStruct
  {
    value : i32,
  }

  // Example with a tuple struct
  #[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
  struct MyTupleStruct( i32 );

  // Test `MyStruct` conversions
  let got : MyStruct = 10.into();
  let exp = MyStruct { value : 10 };
  assert_eq!( got, exp );

  let got_tuple : MyTupleStruct = 50.into();
  let exp_tuple = MyTupleStruct( 50 );
  assert_eq!( got_tuple, exp_tuple );

  dbg!( exp );
  //> MyStruct {
  //>   value : 10,
  //> }

  dbg!( exp_tuple );
  //> MyTupleStruct( 50 )
}
