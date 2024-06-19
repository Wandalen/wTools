#[ allow( dead_code) ]
enum EnumNamedSingle
{
  A { a : i32 },
  B { a : i32 },
}

impl From< EnumNamedSingle > for i32
{
  fn from( other : EnumNamedSingle ) -> Self
  {
    match other
    {
      EnumNamedSingle::A { a } | EnumNamedSingle::B { a } => a,
    }
  }
}

include!( "./only_test/enum_named_single.rs" );
