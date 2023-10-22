#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn from_named_fields()
{
  use TheModule::prelude::*;

  #[ derive( Debug, PartialEq, TheModule::Make ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
    c : i32,
    d : i32,
  }

  include!( "./only_test/from_named_fields.rs" );
}

//

#[ test ]
fn from_tuple()
{
  use TheModule::prelude::*;

  #[ derive( Debug, PartialEq, TheModule::Make ) ]
  struct StructTuple( i32, i32, i32, i32 );

  // include!( "./only_test/from_tuple.rs" );
}

//

#[ test ]
fn sample()
{
  use TheModule::prelude::*;
  use TheModule::Make;

  #[ derive( Debug, PartialEq, Make ) ]
  struct MyStruct
  {
    a : i32,
    b : i32,
  }

  let got : MyStruct = from!();
  let exp = MyStruct { a : 0, b : 0 };
  a_id!( got, exp );

  let got : MyStruct = from!( 13 );
  let exp = MyStruct { a : 13, b : 13 };
  a_id!( got, exp );

}
