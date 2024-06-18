use core::fmt::Debug;

#[ allow( dead_code ) ]
struct BoundsInlined< T : ToString, U : Debug >( T, U );

impl< T : ToString, U : Debug > AsMut< T > for BoundsInlined< T, U >
{
  fn as_mut( &mut self ) -> &mut T
  {
    &mut self.0
  }
}

include!( "./only_test/bounds_inlined.rs" );
