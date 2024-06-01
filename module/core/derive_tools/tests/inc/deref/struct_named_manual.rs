use core::ops::Deref;

#[ allow( dead_code ) ]
struct StructNamed
{
  a : String,
  b : i32,
}

impl Deref for StructNamed
{
  type Target = String;
  fn deref( &self ) -> &Self::Target
  {
    &self.a
  }
}

include!( "./only_tests/struct_named.rs" );
