trait Trait<'a> {}
impl<'a> Trait<'a> for i32 {}

use core::ops::Deref;
use derive_tools::DerefMut;

#[ allow( dead_code ) ]
#[ derive( DerefMut ) ]
struct BoundsWhere< T, U >( T, U )
where
  T : ToString,
  for< 'a > U : Trait< 'a >;

impl< T, U > Deref for BoundsWhere< T, U >
where
  T : ToString,
  for< 'a > U : Trait< 'a >
{
  type Target = T;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

include!( "./only_test/bounds_where.rs" );
