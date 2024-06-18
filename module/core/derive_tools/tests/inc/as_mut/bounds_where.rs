trait Trait<'a> {}
impl<'a> Trait<'a> for i32 {}

use derive_tools::AsMut;

#[ allow( dead_code ) ]
#[ derive( AsMut ) ]
struct BoundsWhere< T, U >( T, U )
where
  T : ToString,
  for< 'a > U : Trait< 'a >;

include!( "./only_test/bounds_where.rs" );
