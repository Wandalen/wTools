use core::ops::Not;

#[ allow( dead_code ) ]
struct DefaultOff
{
  a : bool,
  b : u8,
}

impl Not for DefaultOff {
  type Output = Self;

  fn not( self ) -> Self::Output {
    Self { a : self.a, b : self.b }
  }
}

include!( "./only_test/default_off.rs" );
