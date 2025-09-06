use super::*;
pub use the_module::reflect;
use test_tools::a_id;

#[ test ]
fn data_basic()
{
  use reflect::Primitive;

  let got = Primitive::i32( 13i32 );
  a_id!( got, Primitive::i32( 13i32 ) );

}
