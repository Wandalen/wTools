trait Trait<'a> {}
impl<'a> Trait<'a> for i32 {}

use core::ops::Deref;

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

include!( "./only_tests/bounds_where.rs" );
