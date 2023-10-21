#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn make_named_fields()
{

  #[ derive( Debug, PartialEq, TheModule::Make ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
    c : i32,
    d : i32,
  }

  include!( "./only_test/make_named_fields.rs" );
}

//

#[ test ]
fn make_tuple()
{

  #[ derive( Debug, PartialEq, TheModule::Make ) ]
  struct StructTuple( i32, i32, i32, i32 );

  include!( "./only_test/make_tuple.rs" );
}

// xxx

// #[ test ]
// fn sample()
// {
//   use TheModule::exposed::*;
//
//   #[ derive( Debug, PartialEq, Make ) ]
//   struct MyStruct
//   {
//     a : i32,
//     b : i32,
//   }
//
//   let got : MyStruct = make!();
//   let exp = MyStruct { a : 0, b : 0 };
//   a_id!( got, exp );
//
//   let got : MyStruct = make!( 13 );
//   let exp = MyStruct { a : 13, b : 13 };
//   a_id!( got, exp );
//
//   let got : MyStruct = make!( 1, 3 );
//   let exp = MyStruct { a : 1, b : 3 };
//   a_id!( got, exp );
//
// }
