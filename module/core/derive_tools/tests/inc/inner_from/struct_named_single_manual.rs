#[ allow( dead_code ) ]
struct StructNamedSingle
{
  a : i32,
}

impl From< StructNamedSingle > for i32
{
  fn from( other : StructNamedSingle ) -> Self
  {
    other.a
  }
}

include!( "./only_test/struct_named_single.rs" );
