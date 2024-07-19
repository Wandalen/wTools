use core::ops::Not;

#[ allow( dead_code ) ]
struct DefaultOffSomeOn
{
  a : bool,
  b : u8,
}

impl Not for  DefaultOffSomeOn {
  type Output = Self;

  fn not( self ) -> Self::Output {
    Self { a: self.a, b: !self.b }
  }
}

include!( "./only_test/default_off_some_on.rs" );
