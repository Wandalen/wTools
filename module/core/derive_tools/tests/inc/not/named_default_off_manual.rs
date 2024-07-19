use core::ops::Not;

#[ allow( dead_code ) ]
struct NamedDefaultOff
{
  a : bool,
  b : u8,
}

impl Not for NamedDefaultOff
{
  type Output = Self;

  fn not( self ) -> Self::Output
  {
    Self { a : self.a, b : self.b }
  }
}

include!( "only_test/named_default_off.rs" );
