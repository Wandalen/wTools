#[ allow( dead_code) ]
enum EnumNamed
{
  A { a : String, b : i32 },
  B { a : String, b : i32 },
}

impl From< EnumNamed > for ( String, i32 )
{
  fn from( other : EnumNamed ) -> Self
  {
    match other
    {
      EnumNamed::A { a, b } | EnumNamed::B { a, b } => ( a, b ),
    }
  }
}

include!( "./only_test/enum_named.rs" );
