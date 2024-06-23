#[ allow( dead_code) ]
enum EnumUnit
{
  A,
  B,
}

impl From< EnumUnit > for ()
{
  fn from( _ : EnumUnit ) -> Self
  {
    ()
  }
}

include!( "./only_test/enum_unit.rs" );
