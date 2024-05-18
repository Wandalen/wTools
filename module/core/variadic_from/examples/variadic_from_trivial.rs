//! This test function validates the `VariadicFrom` trait implementation for the `MyStruct` struct.
//! It checks the conversion from tuples and individual values into an instance of `MyStruct`.

#[ cfg( not( all(feature = "enabled", feature = "type_variadic_from", feature = "derive_variadic_from" ) ) ) ]
fn main(){}

#[ cfg( all(feature = "enabled", feature = "type_variadic_from", feature = "derive_variadic_from" ) )]
fn main()
{
  use variadic_from::exposed::*;

  #[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
  struct MyStruct
  {
    a : i32,
    b : i32,
  }

  impl From_1< i32 > for MyStruct
  {
    fn from_1( a : i32 ) -> Self { Self { a, b : a } }
  }

  let got : MyStruct = from!();
  let exp = MyStruct { a : 0, b : 0 };
  assert_eq!( got, exp );

  let got : MyStruct = from!( 13 );
  let exp = MyStruct { a : 13, b : 13 };
  assert_eq!( got, exp );

  let got : MyStruct = from!( 13, 14 );
  let exp = MyStruct { a : 13, b : 14 };
  assert_eq!( got, exp );
  dbg!( exp );
  //> MyStruct {
  //>   a: 13,
  //>   b: 14,
  //> }

}
