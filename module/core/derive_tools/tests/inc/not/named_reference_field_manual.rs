use core::ops::Not;

#[ allow( dead_code ) ]
struct NamedReferenceField< 'a >
{
  a : &'a bool,
  b : u8,
}

impl< 'a > Not for NamedReferenceField< 'a >
{
  type Output = Self;

  fn not( self ) -> Self::Output
  {
    Self { a : self.a, b : !self.b }
  }
}

include!( "only_test/named_reference_field.rs" );
