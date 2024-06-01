use super::*;

#[ derive( Debug, PartialEq, Eq ) ]
struct StructNamedFields
{
  a : i32,
  b : bool,
}

impl From< StructNamedFields > for ( i32, bool )
{
  #[ inline( always ) ]
  fn from( src : StructNamedFields ) -> Self
  {
    ( src.a, src.b )
  }
}

include!( "./only_test/multiple_named.rs" );
