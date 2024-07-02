use super::*;

#[ derive( Debug, PartialEq, Eq ) ]
struct StructNamedFields
{
  a : i32,
  b : bool,
}

impl StructNamedFields
{
  #[ inline( always ) ]
  fn new( a : i32, b : bool ) -> Self
  {
    Self{ a, b }
  }
}

include!( "./only_test/multiple_named.rs" );
