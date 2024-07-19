use core::ops::Not;

#[ allow( dead_code ) ]
struct TupleDefaultOnSomeOff( bool, u8 );

impl Not for TupleDefaultOnSomeOff
{
  type Output = Self;

  fn not( self ) -> Self::Output
  {
    Self( !self.0, self.1 )
  }
}

include!( "only_test/tuple_default_on_some_off.rs" );
