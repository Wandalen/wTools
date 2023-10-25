#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn sample()
{
  use TheModule::exposed::*;

  #[ derive( Debug, PartialEq, FromInner ) ]
  struct MyStruct
  {
    a : i32,
  }

  let got : MyStruct = MyStruct::from( 13 );
  let exp = MyStruct { a : 13 };
  a_id!( got, exp );

}
