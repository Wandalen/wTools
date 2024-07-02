use core::fmt::Debug;

use core::ops::Deref;

#[ allow( dead_code ) ]
struct BoundsMixed< T : ToString, U >( T, U )
where
  U : Debug;

impl< T : ToString, U > Deref for BoundsMixed< T, U >
where
  U : Debug,
{
  type Target = T;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

include!( "./only_test/bounds_mixed.rs" );
