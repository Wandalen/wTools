use core::ops::Deref;
use derive_tools::DerefMut;

#[ allow( dead_code ) ]
#[ derive( DerefMut ) ]
struct StructNamed
{
  a : String,
  b : i32,
}

impl Deref for StructNamed
{
  type Target = String;
  fn deref( &self ) -> &Self::Target
  {
    &self.a
  }
}

include!( "./only_test/struct_named.rs" );
