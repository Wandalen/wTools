use super::*;
pub use TheModule::reflect;

#[ test ]
fn reflect_basic_test()
{
  use reflect::{ EntityInterface, Instance };

  a_id!( 0i8.reflect().reflect_is_container(), false );
  a_id!( 0i8.reflect().reflect_len(), 0 );
  a_id!( 0i8.reflect().reflect_type_name(), "i8" );
  a_id!( 0i8.reflect().reflect_elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( 0i16.reflect().reflect_is_container(), false );
  a_id!( 0i16.reflect().reflect_len(), 0 );
  a_id!( 0i16.reflect().reflect_type_name(), "i16" );
  a_id!( 0i16.reflect().reflect_elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( 0i32.reflect().reflect_is_container(), false );
  a_id!( 0i32.reflect().reflect_len(), 0 );
  a_id!( 0i32.reflect().reflect_type_name(), "i32" );
  a_id!( 0i32.reflect().reflect_elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( 0i64.reflect().reflect_is_container(), false );
  a_id!( 0i64.reflect().reflect_len(), 0 );
  a_id!( 0i64.reflect().reflect_type_name(), "i64" );
  a_id!( 0i64.reflect().reflect_elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  //

  a_id!( 0u8.reflect().reflect_is_container(), false );
  a_id!( 0u8.reflect().reflect_len(), 0 );
  a_id!( 0u8.reflect().reflect_type_name(), "u8" );
  a_id!( 0u8.reflect().reflect_elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( 0u16.reflect().reflect_is_container(), false );
  a_id!( 0u16.reflect().reflect_len(), 0 );
  a_id!( 0u16.reflect().reflect_type_name(), "u16" );
  a_id!( 0u16.reflect().reflect_elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( 0u32.reflect().reflect_is_container(), false );
  a_id!( 0u32.reflect().reflect_len(), 0 );
  a_id!( 0u32.reflect().reflect_type_name(), "u32" );
  a_id!( 0u32.reflect().reflect_elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( 0u64.reflect().reflect_is_container(), false );
  a_id!( 0u64.reflect().reflect_len(), 0 );
  a_id!( 0u64.reflect().reflect_type_name(), "u64" );
  a_id!( 0u64.reflect().reflect_elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

}
