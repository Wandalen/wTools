#[ allow( dead_code) ]
enum EnumNamedEmpty
{
  A {},
  B {},
}

impl From< EnumNamedEmpty > for ()
{
  fn from( _ : EnumNamedEmpty ) -> Self
  {
    ()
  }
}

include!( "./only_test/enum_named_empty.rs" );
