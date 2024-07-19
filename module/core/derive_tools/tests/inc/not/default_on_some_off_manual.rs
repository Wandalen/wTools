use core::ops::Not;

#[ allow( dead_code ) ]
struct DefaultOnSomeOff
{
  a : bool,
  b : u8,
}

impl Not for DefaultOnSomeOff {
  type Output = Self;

  fn not( self ) -> Self::Output {
    Self { a: !self.a, b: self.b }
  }
}

include!( "./only_test/default_on_some_off.rs" );
