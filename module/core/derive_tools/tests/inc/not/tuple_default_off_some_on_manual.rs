use core::ops::Not;

#[ allow( dead_code ) ]
struct TupleDefaultOffSomeOn( bool, u8 );

impl Not for  TupleDefaultOffSomeOn
{
  type Output = Self;

  fn not( self ) -> Self::Output
  {
    Self( self.0, !self.1 )
  }
}

include!( "only_test/tuple_default_off_some_on.rs" );
