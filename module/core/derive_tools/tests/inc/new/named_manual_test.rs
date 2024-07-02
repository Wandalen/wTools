use super::*;

#[ derive( Debug, PartialEq, Eq ) ]
struct MyStruct
{
  a : i32,
}

impl MyStruct
{
  #[ inline( always ) ]
  fn new( src : i32 ) -> Self
  {
    Self{ a : src }
  }
}

include!( "./only_test/named.rs" );
