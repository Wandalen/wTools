use core::ops::Not;

struct StructUnit;

impl Not for StructUnit
{
  type Output = Self;

  fn not( self ) -> Self::Output
  {
    Self {}
  }
}

include!( "./only_test/struct_unit.rs" );
