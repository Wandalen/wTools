use core::ops::Not;

#[ allow( dead_code ) ]
struct TupleDefaultOffReferenceOn< 'a >( &'a bool, u8 );

impl< 'a > Not for TupleDefaultOffReferenceOn< 'a >
{
  type Output = Self;

  fn not(self) -> Self::Output
  {
    Self( self.0, self.1 )
  }
}

include!( "./only_test/tuple_default_off_reference_on.rs" );
