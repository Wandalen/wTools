#[ allow( dead_code) ]
enum EnumNamedEmpty
{
  A {},
  B {},
}

impl AsRef< () > for EnumNamedEmpty
{
  fn as_ref( &self ) -> &()
  {
    &()
  }
}

include!( "./only_test/enum_named_empty.rs" );
