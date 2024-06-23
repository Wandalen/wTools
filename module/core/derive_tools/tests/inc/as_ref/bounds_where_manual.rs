trait Trait<'a> {}
impl<'a> Trait<'a> for i32 {}

#[ allow( dead_code ) ]
struct BoundsWhere< T, U >( T, U )
where
  T : ToString,
  for< 'a > U : Trait< 'a >;

impl< T, U > AsRef< T > for BoundsWhere< T, U >
where
  T : ToString,
  for< 'a > U : Trait< 'a >
{
  fn as_ref( &self ) -> &T
  {
    &self.0
  }
}

include!( "./only_test/bounds_where.rs" );
