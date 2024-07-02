use core::ops::Deref;

#[ allow( dead_code) ]
enum EnumNamedEmpty
{
  A {},
  B {},
}

impl Deref for EnumNamedEmpty
{
  type Target = ();
  fn deref( &self ) -> &Self::Target
  {
    &()
  }
}

include!( "./only_test/enum_named_empty.rs" );
