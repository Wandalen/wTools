use super::*;
pub use TheModule::reflect;

#[ test ]
fn data_basic()
{
  use reflect::Data;

  let got = Data::i32( 13i32 );
  a_id!( got, Data::i32( 13i32 ) );

}
