#![ allow( unused_imports ) ]
use super::*;
use core::convert::AsMut;

struct StructNamed
{
  field1 : i32,
  
}

impl AsMut< i32 > for StructNamed
{
  fn as_mut( &mut self ) -> &mut i32
  {
    &mut self.field1
  }
}

include!( "only_test/struct_named.rs" );