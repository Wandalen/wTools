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
#[ derive( Debug, PartialEq, Default, VariadicFrom ) ]

#[ from( f32 ) ]
pub struct FromAttributeStruct
{
  value : i32,
}

#[ test ]
fn from_attribute_test()
{
  let x : FromAttributeStruct = From::from( 10 );
  assert_eq!( x.value, 10 );

  let x : FromAttributeStruct = From::from( 20.0f32 );
  assert_eq!( x.value, 20 );
}

// Explicitly implement From1<f32> for NamedStruct to satisfy the test in variadic_from_only_test.rs
impl From1< f32 > for NamedStruct
{
  fn from1( a : f32 ) -> Self { Self { field : a as i32 } }
}

include!( "variadic_from_only_test.rs" );