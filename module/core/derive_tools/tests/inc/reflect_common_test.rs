use super::*;
pub use TheModule::reflect;

#[ test ]
fn reflect_basic_test()
{
  use reflect::{ Entity, Instance };

  a_id!( 0i8.reflect().is_container(), false );
  a_id!( 0i8.reflect().len(), 0 );
  a_id!( 0i8.reflect().type_name(), "i8" );
  a_id!( 0i8.reflect().elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( 0i16.reflect().is_container(), false );
  a_id!( 0i16.reflect().len(), 0 );
  a_id!( 0i16.reflect().type_name(), "i16" );
  a_id!( 0i16.reflect().elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( 0i32.reflect().is_container(), false );
  a_id!( 0i32.reflect().len(), 0 );
  a_id!( 0i32.reflect().type_name(), "i32" );
  a_id!( 0i32.reflect().elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( 0i64.reflect().is_container(), false );
  a_id!( 0i64.reflect().len(), 0 );
  a_id!( 0i64.reflect().type_name(), "i64" );
  a_id!( 0i64.reflect().elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  //

  a_id!( 0u8.reflect().is_container(), false );
  a_id!( 0u8.reflect().len(), 0 );
  a_id!( 0u8.reflect().type_name(), "u8" );
  a_id!( 0u8.reflect().elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( 0u16.reflect().is_container(), false );
  a_id!( 0u16.reflect().len(), 0 );
  a_id!( 0u16.reflect().type_name(), "u16" );
  a_id!( 0u16.reflect().elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( 0u32.reflect().is_container(), false );
  a_id!( 0u32.reflect().len(), 0 );
  a_id!( 0u32.reflect().type_name(), "u32" );
  a_id!( 0u32.reflect().elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( 0u64.reflect().is_container(), false );
  a_id!( 0u64.reflect().len(), 0 );
  a_id!( 0u64.reflect().type_name(), "u64" );
  a_id!( 0u64.reflect().elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

}
