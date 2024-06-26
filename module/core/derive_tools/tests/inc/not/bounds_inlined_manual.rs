use std::fmt::Debug;
use core::ops::Not;

#[ allow( dead_code ) ]
struct BoundsInlined< T : ToString + Not< Output = T >, U : Debug + Not< Output = U > >
{
  a: T,
  b: U,
}

impl< T : ToString + Not< Output = T >, U : Debug + Not< Output = U > > Not for BoundsInlined< T, U >
{
  type Output = Self;

  fn not( self ) -> Self::Output
  {
    Self { a : !self.a, b : !self.b }
  }
}

include!( "./only_test/bounds_inlined.rs" );
