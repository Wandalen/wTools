use core::fmt::Debug;

#[ allow( dead_code ) ]
struct BoundsInlined< T : ToString, U : Debug >( T, U );

impl< T : ToString, U : Debug > From< BoundsInlined< T, U > > for ( T, U )
{
  fn from( other : BoundsInlined< T, U > ) -> Self
  {
    ( other.0, other.1 )
  }
}

include!( "./only_test/bounds_inlined.rs" );
