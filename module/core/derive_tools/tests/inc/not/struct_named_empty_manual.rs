use core::ops::Not;

#[ allow( dead_code ) ]
struct StructNamedEmpty{}

impl Not for StructNamedEmpty
{
  type Output = Self;

  fn not( self ) -> Self::Output {
    StructNamedEmpty {}
  }
}

include!( "./only_test/struct_named_empty.rs" );
