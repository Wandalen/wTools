use super::*;

#[ derive( Debug, PartialEq, Eq ) ]
struct MyStruct
{
  a: i32,
}

impl From< i32 > for MyStruct
{
  #[ inline( always ) ]
  fn from( src : i32 ) -> Self
  {
    Self{ a: src }
  }
}

include!( "./only_test/from_inner_named.rs" );
