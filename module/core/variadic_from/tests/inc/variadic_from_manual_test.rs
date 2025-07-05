//! This test file contains manual implementations of `From` for `variadic_from` to serve as a baseline.

use variadic_from::exposed::{ From1, From2, From3, from };

// For `MyStruct`
#[ derive( Default ) ]
#[ allow( dead_code ) ]
pub struct MyStruct
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
#[ allow( dead_code ) ]
pub struct NamedStruct
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

// For `ThreeFieldStruct`
#[ derive( Default ) ]
#[ allow( dead_code ) ]
pub struct ThreeFieldStruct
{
  x : i32,
  y : i32,
  z : i32,
}

impl From1< i32 > for ThreeFieldStruct
{
  fn from1( a : i32 ) -> Self { Self { x : a, y : a, z : a } }
}

impl From2< i32, i32 > for ThreeFieldStruct
{
  fn from2( a : i32, b : i32 ) -> Self { Self { x : a, y : b, z : b } }
}

impl From3< i32, i32, i32 > for ThreeFieldStruct
{
  fn from3( a : i32, b : i32, c : i32 ) -> Self { Self { x : a, y : b, z : c } }
}


