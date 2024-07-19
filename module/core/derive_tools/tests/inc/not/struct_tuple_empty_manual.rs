use core::ops::Not;

#[ allow( dead_code ) ]
struct StructTupleEmpty();

impl Not for StructTupleEmpty
{
  type Output = Self;

  fn not( self ) -> Self::Output
  {
    Self()
  }
}

include!( "./only_test/struct_tuple_empty.rs" );
