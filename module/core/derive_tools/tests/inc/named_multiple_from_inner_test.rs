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
    b : bool,
  }

  let got = MyStruct::from(( 13, true ));
  let exp = MyStruct { a : 13, b: true };
  a_id!( got, exp );

}
