trait Trait<'a> {}
impl<'a> Trait<'a> for i32 {}

use core::ops::{ Deref, DerefMut };

#[ allow( dead_code ) ]
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
impl< T, U > DerefMut for BoundsWhere< T, U >
where
  T : ToString,
  for< 'a > U : Trait< 'a >
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

include!( "./only_tests/bounds_where.rs" );
