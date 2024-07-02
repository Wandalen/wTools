use super::*;

#[ derive( Debug, PartialEq, Eq ) ]
struct MyStruct
{
  a : i32,
}

impl From< MyStruct > for i32
{
  #[ inline( always ) ]
  fn from( src : MyStruct ) -> Self
  {
    src.a
  }
}

include!( "./only_test/named.rs" );
