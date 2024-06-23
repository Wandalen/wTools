use core::fmt::Debug;

#[ allow( dead_code ) ]
struct BoundsInlined< T : ToString, U : Debug >( T, U );

impl< T : ToString, U : Debug > AsRef< T > for BoundsInlined< T, U >
{
  fn as_ref( &self ) -> &T
  {
    &self.0
  }
}

include!( "./only_test/bounds_inlined.rs" );
