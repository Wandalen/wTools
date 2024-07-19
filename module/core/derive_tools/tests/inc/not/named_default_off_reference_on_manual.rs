use core::ops::Not;

#[ allow( dead_code ) ]
struct NamedDefaultOffReferenceOn< 'a >
{
  a : &'a bool,
  b : u8,
}

impl< 'a > Not for NamedDefaultOffReferenceOn< 'a >
{
  type Output = Self;

  fn not(self) -> Self::Output
  {
    Self { a: self.a, b : self.b }
  }
}

include!( "only_test/named_default_off_reference_on.rs" );
