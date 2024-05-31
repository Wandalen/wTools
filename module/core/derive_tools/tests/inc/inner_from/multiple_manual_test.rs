use super::*;

#[ derive( Debug, PartialEq, Eq ) ]
struct StructWithManyFields( i32, bool );

impl From< StructWithManyFields > for ( i32, bool )
{
  #[ inline( always ) ]
  fn from( src : StructWithManyFields ) -> Self
  {
    ( src.0, src.1 )
  }
}

include!( "./only_test/multiple.rs" );
