trait Trait<'a> {}
impl<'a> Trait<'a> for i32 {}

use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code ) ]
#[ derive( Deref ) ]
struct BoundsWhere< T, U >( T, U )
where
  T : ToString,
  for< 'a > U : Trait< 'a >;

include!( "./only_tests/bounds_where.rs" );
