use core::ops::Not;

#[ allow( dead_code ) ]
struct NamedDefaultOnMutReferenceOff< 'a >
{
  a : &'a bool,
  b : u8,
}

impl< 'a > Not for NamedDefaultOnMutReferenceOff< 'a >
{
  type Output = Self;

  fn not(self) -> Self::Output
  {
    Self { a :self.a, b : !self.b }
  }
}

include!( "only_test/named_default_on_mut_reference_off.rs" );
