use core::ops::Not;

#[ allow( dead_code ) ]
struct NamedMutReferenceField< 'a >
{
  a : &'a mut bool,
  b : u8,
}

impl< 'a > Not for NamedMutReferenceField< 'a >
{
  type Output = Self;

  fn not( self ) -> Self::Output
  {
    *self.a = !*self.a;
    Self { a : self.a, b : !self.b }
  }
}

include!( "only_test/named_mut_reference_field.rs" );
