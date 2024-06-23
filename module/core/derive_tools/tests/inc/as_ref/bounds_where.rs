trait Trait<'a> {}
impl<'a> Trait<'a> for i32 {}

use derive_tools::AsRef;

#[ allow( dead_code ) ]
#[ derive( AsRef ) ]
struct BoundsWhere< T, U >( T, U )
where
  T : ToString,
  for< 'a > U : Trait< 'a >;

include!( "./only_test/bounds_where.rs" );
