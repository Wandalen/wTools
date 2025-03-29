use core::ops::Not;

#[ allow( dead_code ) ]
struct StructNamed
{
  a : bool,
  b : u8,
}

impl Not for StructNamed
{
  type Output = Self;

  fn not( self ) -> Self::Output
  {
    Self { a : !self.a, b : !self.b }
  }
}

include!( "./only_test/struct_named.rs" );
