use super::*;

#[ derive( Debug, PartialEq, Eq ) ]
struct StructWithManyFields( i32, bool );

impl StructWithManyFields
{
  #[ inline( always ) ]
  fn new( src1 : i32, src2 : bool ) -> Self
  {
    Self( src1, src2 )
  }
}

include!( "./only_test/multiple_unnamed.rs" );
