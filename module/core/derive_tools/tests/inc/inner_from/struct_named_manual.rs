#[ allow( dead_code ) ]
struct StructNamed
{
  a : String,
  b : i32,
}

impl From< StructNamed > for ( String, i32)
{
  fn from( other : StructNamed ) -> Self
  {
    ( other.a, other.b )
  }
}

include!( "./only_test/struct_named.rs" );
