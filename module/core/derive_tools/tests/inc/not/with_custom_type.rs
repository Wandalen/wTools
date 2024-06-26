use core::ops::Not;
use super::*;

#[ allow( dead_code ) ]
struct CustomType
{
  a : bool,
  b : u8,
}

impl Not for CustomType
{
  type Output = Self;

  fn not( self ) -> Self::Output
  {
    Self { a : !self.a, b : !self.b }
  }
}

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
struct WithCustomType
{
  custom_type : CustomType,
}

include!( "./only_test/with_custom_type.rs" );
