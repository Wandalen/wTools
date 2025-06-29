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

  // Define a struct `MyStruct` with a single field `value`.
  // It derives common traits and `VariadicFrom`.
  #[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
  
  #[ from( f32 ) ]
  struct MyStruct
  {
    value : i32,
  }

  // Test `MyStruct` conversions
  let got : MyStruct = 10.into();
  let exp = MyStruct { value : 10 };
  assert_eq!( got, exp );

  let got : MyStruct = 20.0.into();
  let exp = MyStruct { value : 20 };
  assert_eq!( got, exp );

  dbg!( exp );
  //> MyStruct {
  //>   value : 20,
  //> }

  // Example with a tuple struct
  #[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
  #[ from( i16 ) ]
  #[ from( u8 ) ]
  struct MyTupleStruct( i32 );

  let got_tuple : MyTupleStruct = 50i16.into();
  let exp_tuple = MyTupleStruct( 50 );
  assert_eq!( got_tuple, exp_tuple );

  let got_tuple : MyTupleStruct = 100u8.into();
  let exp_tuple = MyTupleStruct( 100 );
  assert_eq!( got_tuple, exp_tuple );

  dbg!( exp_tuple );
  //> MyTupleStruct( 100 )
}
