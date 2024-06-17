#[ allow( dead_code) ]
enum EnumUnit
{
  A,
  B,
}

impl AsRef< () > for EnumUnit
{
  fn as_ref( &self ) -> &()
  {
    &()
  }
}

include!( "./only_test/enum_unit.rs" );
