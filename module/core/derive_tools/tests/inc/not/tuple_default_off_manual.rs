use core::ops::Not;

#[ allow( dead_code ) ]
struct TupleDefaultOff( bool, u8 );

impl Not for TupleDefaultOff
{
  type Output = Self;

  fn not( self ) -> Self::Output
  {
    Self( self.0, self.1 )
  }
}

include!( "only_test/tuple_default_off.rs" );
