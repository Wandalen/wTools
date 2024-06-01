use core::fmt::Debug;

use core::ops::Deref;

#[ allow( dead_code ) ]
struct BoundsInlined< T : ToString, U : Debug >( T, U );

impl< T : ToString, U : Debug > Deref for BoundsInlined< T, U >
{
  type Target = T;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

include!( "./only_tests/bounds_inlined.rs" );
