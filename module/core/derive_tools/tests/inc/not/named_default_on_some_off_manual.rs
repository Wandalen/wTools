use core::ops::Not;

#[ allow( dead_code ) ]
struct NamedDefaultOnSomeOff
{
  a : bool,
  b : u8,
}

impl Not for NamedDefaultOnSomeOff
{
  type Output = Self;

  fn not( self ) -> Self::Output
  {
    Self { a: !self.a, b: self.b }
  }
}

include!( "only_test/named_default_on_some_off.rs" );
