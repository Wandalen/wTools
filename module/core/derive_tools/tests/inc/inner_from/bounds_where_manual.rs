trait Trait<'a> {}
impl<'a> Trait<'a> for i32 {}

#[ allow( dead_code ) ]
struct BoundsWhere< T, U >( T, U )
where
  T : ToString,
  for< 'a > U : Trait< 'a >;

impl< T, U > From< BoundsWhere< T, U > > for ( T, U )
where
  T : ToString,
  for< 'a > U : Trait< 'a >
{
  fn from( other : BoundsWhere< T, U > ) -> Self
  {
    ( other.0, other.1 )
  }
}

include!( "./only_test/bounds_where.rs" );
