use core::ops::Deref;

#[ allow( dead_code ) ]
struct StructTupleEmpty();

impl Deref for StructTupleEmpty
{
  type Target = ();
  fn deref( &self ) -> &Self::Target
  {
    &()
  }
}

include!( "./only_test/struct_tuple_empty.rs" );
