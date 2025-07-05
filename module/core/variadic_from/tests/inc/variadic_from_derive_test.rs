//! This test file contains derive implementations of `From` for `variadic_from`.

use variadic_from_meta::VariadicFrom;
use variadic_from::exposed::{ From1, From2, From3, from };

#[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
pub struct MyStruct
{
  a : i32,
  b : i32,
}

#[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
pub struct NamedStruct
{
  field : i32,
}
#[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
pub struct ThreeFieldStruct
{
  x : i32,
  y : i32,
  z : i32,
}


// Explicitly implement From1<f32> for NamedStruct to satisfy the test in variadic_from_only_test.rs
impl From1< f32 > for NamedStruct
{
  fn from1( a : f32 ) -> Self { Self { field : a as i32 } }
}




#[ test ]
fn single_field_conversion_test()
{
  let x : NamedStruct = 200.into();
  assert_eq!( x.field, 200 );
}

#[ test ]
fn blanket_from1_two_tuple_test()
{
  let x : MyStruct = ( 30, 40 ).into();
  assert_eq!( x.a, 30 );
  assert_eq!( x.b, 40 );
}

#[ test ]

fn blanket_from1_three_tuple_test()
{
  let x : ThreeFieldStruct = ( 4, 5, 6 ).into();
  assert_eq!( x.x, 4 );
  assert_eq!( x.y, 5 );
  assert_eq!( x.z, 6 );
}
