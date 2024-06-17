use core::fmt::Debug;

#[ allow( dead_code ) ]
struct BoundsMixed< T : ToString, U >( T, U )
where
  U : Debug;

impl< T : ToString, U > AsRef< T > for BoundsMixed< T, U >
where
  U : Debug,
{
  fn as_ref( &self ) -> &T
  {
    &self.0
  }
}

include!( "./only_test/bounds_mixed.rs" );
