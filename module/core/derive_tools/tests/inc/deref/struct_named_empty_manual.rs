use core::ops::Deref;

#[ allow( dead_code ) ]
struct StructNamedEmpty{}

impl Deref for StructNamedEmpty
{
  type Target = ();
  fn deref( &self ) -> &Self::Target
  {
    &()
  }
}

include!( "./only_test/struct_named_empty.rs" );
