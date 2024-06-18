use core::fmt::Debug;

#[ allow( dead_code ) ]
struct BoundsMixed< T : ToString, U >( T, U )
where
  U : Debug;

impl< T : ToString, U > AsMut< T > for BoundsMixed< T, U >
where
  U : Debug,
{
  fn as_mut( &mut self ) -> &mut T
  {
    &mut self.0
  }
}


include!( "./only_test/bounds_mixed.rs" );
