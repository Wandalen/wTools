use core::fmt::Debug;

use core::ops::{ Deref, DerefMut };

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
impl< T : ToString, U : Debug > DerefMut for BoundsInlined< T, U >
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

include!( "./only_tests/bounds_inlined.rs" );
