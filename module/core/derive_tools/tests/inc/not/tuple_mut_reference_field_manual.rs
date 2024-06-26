use core::ops::Not;

#[ allow( dead_code ) ]
struct TupleMutReferenceField< 'a >( &'a mut bool, u8 );

impl< 'a > Not for TupleMutReferenceField< 'a >
{
  type Output = Self;

  fn not( self ) -> Self::Output
  {
    *self.0 = !*self.0;
    Self( self.0, !self.1 )
  }
}

include!( "./only_test/tuple_mut_reference_field.rs" );
