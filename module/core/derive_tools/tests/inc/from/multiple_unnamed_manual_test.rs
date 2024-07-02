use super::*;

#[ derive( Debug, PartialEq, Eq ) ]
struct StructWithManyFields( i32, bool );

impl From< ( i32, bool ) > for StructWithManyFields
{
  #[ inline( always ) ]
  fn from( src : ( i32, bool ) ) -> Self
  {
    Self( src.0, src.1 )
  }
}

include!( "./only_test/multiple_unnamed.rs" );
