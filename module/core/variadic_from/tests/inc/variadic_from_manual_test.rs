//! This test file contains manual implementations of `From` for `variadic_from` to serve as a baseline.

// For `MyStruct`
struct MyStruct( i32 );

impl From< i32 > for MyStruct
{
  fn from( value : i32 ) -> Self
  {
    Self( value )
  }
}

impl From< f32 > for MyStruct
{
  fn from( value : f32 ) -> Self
  {
    Self( value as i32 )
  }
}

// For `NamedStruct`
struct NamedStruct
{
  field : i32,
}

impl From< i32 > for NamedStruct
{
  fn from( value : i32 ) -> Self
  {
    Self { field : value }
  }
}

impl From< f32 > for NamedStruct
{
  fn from( value : f32 ) -> Self
  {
    Self { field : value as i32 }
  }
}

include!( "variadic_from_only_test.rs" );