#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn sample()
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
    fn from_1( a : i32 ) -> Self { Self{ a, b : a } }
  }

  let got : MyStruct = from!();
  let exp = MyStruct { a : 0, b : 0 };
  assert_eq!( got, exp );

  let got : MyStruct = from!( 13 );
  let exp = MyStruct { a : 13, b : 13 };
  assert_eq!( got, exp );

}

// qqq : xxx : add to examples and to readme
