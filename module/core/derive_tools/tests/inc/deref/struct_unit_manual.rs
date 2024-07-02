use core::ops::Deref;

#[ allow( dead_code ) ]
struct StructUnit;

impl Deref for StructUnit
{
  type Target = ();
  fn deref( &self ) -> &Self::Target
  {
    &()
  }
}

include!( "./only_test/struct_unit.rs" );
