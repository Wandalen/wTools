use core::ops::Not;

#[ allow( dead_code ) ]
struct NamedDefaultOffSomeOn
{
  a : bool,
  b : u8,
}

impl Not for  NamedDefaultOffSomeOn
{
  type Output = Self;

  fn not( self ) -> Self::Output
  {
    Self { a: self.a, b: !self.b }
  }
}

include!( "only_test/named_default_off_some_on.rs" );
