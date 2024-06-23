#[ allow( dead_code ) ]
struct StructTupleSingle( i32 );

impl From< StructTupleSingle > for i32
{
  fn from( other : StructTupleSingle ) -> Self
  {
    other.0
  }
}

include!( "./only_test/struct_tuple_single.rs" );
