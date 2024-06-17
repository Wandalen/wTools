#[ allow( dead_code) ]
enum EnumTupleEmpty
{
  A(),
  B(),
}

impl AsRef< () > for EnumTupleEmpty
{
  fn as_ref( &self ) -> &()
  {
    &()
  }
}

include!( "./only_test/enum_tuple_empty.rs" );
