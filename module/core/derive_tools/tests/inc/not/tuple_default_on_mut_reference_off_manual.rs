use core::ops::Not;

#[ allow( dead_code ) ]
struct TupleDefaultOnMutReferenceOff< 'a >( &'a bool, u8 );

impl< 'a > Not for TupleDefaultOnMutReferenceOff< 'a >
{
  type Output = Self;

  fn not(self) -> Self::Output
  {
    Self( self.0, !self.1 )
  }
}

include!( "only_test/tuple_default_on_mut_reference_off.rs" );
