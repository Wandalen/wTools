use core::ops::Deref;
use derive_tools::DerefMut;

#[ allow( dead_code ) ]
#[ derive ( DerefMut ) ]
struct StructTuple( String, i32 );

impl Deref for StructTuple
{
  type Target = String;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

include!( "./only_test/struct_tuple.rs" );
