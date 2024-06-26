use core::ops::Not;

#[ allow( dead_code ) ]
struct StructTuple( bool, u8 );

impl Not for StructTuple
{
  type Output = Self;

  fn not( self ) -> Self::Output
  {
    Self( !self.0, !self.1 )
  }
}

include!( "./only_test/struct_tuple.rs" );
