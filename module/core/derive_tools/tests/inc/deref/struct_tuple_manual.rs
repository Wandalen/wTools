use core::ops::Deref;

#[ allow( dead_code ) ]
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
