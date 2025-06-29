//! This test file contains manual implementations of `From` for `variadic_from` to serve as a baseline.

use variadic_from::exposed::{ From1, From2, From3, from };

// For `MyStruct`
#[ derive( Default ) ]
struct MyStruct
{
  a : i32,
  b : i32,
}

impl From1< i32 > for MyStruct
{
  fn from1( a : i32 ) -> Self { Self { a, b : a } }
}

impl From2< i32, i32 > for MyStruct
{
  fn from2( a : i32, b : i32 ) -> Self { Self { a, b } }
}

// For `NamedStruct`
#[ derive( Default ) ]
struct NamedStruct
{
  field : i32,
}

impl From1< i32 > for NamedStruct
{
  fn from1( a : i32 ) -> Self { Self { field : a } }
}

impl From1< f32 > for NamedStruct
{
  fn from1( a : f32 ) -> Self { Self { field : a as i32 } }
}

include!( "variadic_from_only_test.rs" );