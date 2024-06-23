#[ allow( dead_code) ]
enum EnumNamed
{
  A { a : String, b : i32 },
  B { a : String, b : i32 },
}

impl AsRef< String > for EnumNamed
{
  fn as_ref( &self ) -> &String
  {
    match self
    {
      Self::A { a : v, ..} | Self::B { a : v, .. } => v
    }
  }
}

include!( "./only_test/enum_named.rs" );
