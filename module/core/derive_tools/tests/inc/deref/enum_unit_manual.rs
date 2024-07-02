use core::ops::Deref;

#[ allow( dead_code) ]
enum EnumUnit
{
  A,
  B,
}

impl Deref for EnumUnit
{
  type Target = ();
  fn deref( &self ) -> &Self::Target
  {
    &()
  }
}

include!( "./only_test/enum_unit.rs" );
