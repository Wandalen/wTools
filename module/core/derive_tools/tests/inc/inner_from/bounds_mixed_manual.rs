use core::fmt::Debug;

#[ allow( dead_code ) ]
struct BoundsMixed< T : ToString, U >( T, U )
where
  U : Debug;

impl< T : ToString, U > From< BoundsMixed< T, U > > for ( T, U )
where
  U : Debug,
{
  fn from( other : BoundsMixed< T, U > ) -> Self
  {
    ( other.0, other.1 )
  }
}

include!( "./only_test/bounds_mixed.rs" );
