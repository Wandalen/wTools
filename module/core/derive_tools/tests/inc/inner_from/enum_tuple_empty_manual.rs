#[ allow( dead_code) ]
enum EnumTupleEmpty
{
  A(),
  B(),
}

impl From< EnumTupleEmpty > for ()
{
  fn from( _ : EnumTupleEmpty ) -> Self
  {
    ()
  }
}

include!( "./only_test/enum_tuple_empty.rs" );
