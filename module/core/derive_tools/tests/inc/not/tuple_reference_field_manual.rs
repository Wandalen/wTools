use core::ops::Not;

#[ allow( dead_code ) ]
struct TupleReferenceField< 'a >( &'a bool, u8 );

impl< 'a > Not for TupleReferenceField< 'a >
{
  type Output = Self;

  fn not( self ) -> Self::Output
  {
    Self( self.0, !self.1 )
  }
}

include!( "./only_test/tuple_reference_field.rs" );
