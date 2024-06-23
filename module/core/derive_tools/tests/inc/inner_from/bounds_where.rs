trait Trait<'a> {}
impl<'a> Trait<'a> for i32 {}

use derive_tools::InnerFrom;

#[ allow( dead_code ) ]
#[ derive( InnerFrom ) ]
struct BoundsWhere< T, U >( T, U )
where
  T : ToString,
  for< 'a > U : Trait< 'a >;

include!( "./only_test/bounds_where.rs" );
