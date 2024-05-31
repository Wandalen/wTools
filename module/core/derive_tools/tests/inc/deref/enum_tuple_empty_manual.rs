use core::ops::Deref;

#[ allow( dead_code) ]
enum EnumTupleEmpty
{
  A(),
  B(),
}

impl Deref for EnumTupleEmpty
{
  type Target = ();
  fn deref( &self ) -> &Self::Target
  {
    &()
  }
}

include!( "./only_tests/enum_tuple_empty.rs" );
