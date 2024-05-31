#[ allow( unused_imports ) ]
use super::*;

/// This test function validates the `VariadicFrom` trait implementation for the `MyStruct` struct.
/// It checks the conversion from tuples and individual values into an instance of `MyStruct`.
#[ test ]
fn sample()
{
  use variadic_from::exposed::*;

  // Define a struct `MyStruct` with fields `a` and `b`.
  // The struct derives common traits like `Debug`, `PartialEq`, `Default`, and `VariadicFrom`.
  #[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
  // Use `#[ debug ]` to expand and debug generate code.
  // #[ debug ]
  struct MyStruct
  {
    a : i32,
    b : i32,
  }

  // Implement the `From1` trait for `MyStruct`, which allows constructing a `MyStruct` instance
  // from a single `i32` value by assigning it to both `a` and `b` fields.
  impl From1< i32 > for MyStruct
  {
    fn from1( a : i32 ) -> Self { Self { a, b : a } }
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

  let got : MyStruct = From::from( ( 13, 14 ) );
  let exp = MyStruct { a : 13, b : 14 };
  assert_eq!( got, exp );

  let got : MyStruct = ( 13, 14 ).into();
  let exp = MyStruct { a : 13, b : 14 };
  assert_eq!( got, exp );

}
