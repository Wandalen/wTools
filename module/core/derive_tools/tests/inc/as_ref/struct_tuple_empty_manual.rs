#[ allow( dead_code ) ]
struct StructTupleEmpty();

impl AsRef< () > for StructTupleEmpty
{
  fn as_ref( &self ) -> &()
  {
    &()
  }
}

include!( "./only_test/struct_tuple_empty.rs" );
