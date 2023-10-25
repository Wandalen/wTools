#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn sample()
{
  use TheModule::exposed::*;

  #[ derive( Debug, PartialEq, InnerFrom ) ]
  struct MyStruct
  {
    a : i32,
  }

  let got : i32 = MyStruct{ a: 13 }.into();
  let exp : i32  = 13;
  a_id!( got, exp );

}
