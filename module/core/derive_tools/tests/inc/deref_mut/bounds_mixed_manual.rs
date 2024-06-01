use core::fmt::Debug;

use core::ops::{ Deref, DerefMut };

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
impl< T : ToString, U > DerefMut for BoundsMixed< T, U >
where
  U : Debug,
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}


include!( "./only_tests/bounds_mixed.rs" );
