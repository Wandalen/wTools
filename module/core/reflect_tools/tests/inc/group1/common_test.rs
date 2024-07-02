use super::*;
pub use the_module::reflect;

#[ test ]
fn reflect_common_test()
{
  use reflect::{ Entity, reflect };

  // for understanding
  println!( "TypeId< i32 > : {:?}", core::any::TypeId::of::< i32 >() );
  println!( "TypeId< &i32 > : {:?}", core::any::TypeId::of::< & i32 >() ); // qqq : qqq  fro Yuliia : problem. should be distinct id
  println!( "TypeId< String > : {:?}", core::any::TypeId::of::< String >() );
  println!( "TypeId< &String > : {:?}", core::any::TypeId::of::< & String >() );
  println!( "TypeId< str > : {:?}", core::any::TypeId::of::< str >() );
  println!( "TypeId< &str > : {:?}", core::any::TypeId::of::< & str >() );

  println!( "reflect( i32 ) : {:?}", reflect::reflect( &1i32 ) );
  println!( "reflect( &i32 ) : {:?}", reflect::reflect( &&1i32 ) );

  println!( "i32 : {:?}", reflect( &1i32 ).type_id() );
  println!( "&i32 : {:?}", reflect( &&1i32 ).type_id() );
  println!( "String : {:?}", reflect( &"abc" ).type_id() );
  println!( "&String : {:?}", reflect( &( "abc".to_string() ) ).type_id() );
  println!( "str : {:?}", reflect( &"abc" ).type_id() );
  println!( "&str : {:?}", reflect( &&"abc" ).type_id() );

  //

  a_id!( reflect( &0i8 ).is_container(), false );
  a_id!( reflect( &0i8 ).len(), 0 );
  a_id!( reflect( &0i8 ).type_name(), "i8" );
  a_id!( reflect( &0i8 ).type_id(), core::any::TypeId::of::< i8 >() );
  a_id!( reflect( &0i8 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( reflect( &0i16 ).is_container(), false );
  a_id!( reflect( &0i16 ).len(), 0 );
  a_id!( reflect( &0i16 ).type_name(), "i16" );
  a_id!( reflect( &0i16 ).type_id(), core::any::TypeId::of::< i16 >() );
  a_id!( reflect( &0i16 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( reflect( &0i32 ).is_container(), false );
  a_id!( reflect( &0i32 ).len(), 0 );
  a_id!( reflect( &0i32 ).type_name(), "i32" );
  a_id!( reflect( &0i32 ).type_id(), core::any::TypeId::of::< i32 >() );
  a_id!( reflect( &0i32 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( reflect( &0i64 ).is_container(), false );
  a_id!( reflect( &0i64 ).len(), 0 );
  a_id!( reflect( &0i64 ).type_name(), "i64" );
  a_id!( reflect( &0i64 ).type_id(), core::any::TypeId::of::< i64 >() );
  a_id!( reflect( &0i64 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( reflect( &&0i8 ).is_container(), false );
  a_id!( reflect( &&0i8 ).len(), 0 );
  a_id!( reflect( &&0i8 ).type_name(), "&i8" );
  a_id!( reflect( &&0i8 ).type_id(), core::any::TypeId::of::< &i8 >() );
  a_id!( reflect( &&0i8 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( reflect( &&0i16 ).is_container(), false );
  a_id!( reflect( &&0i16 ).len(), 0 );
  a_id!( reflect( &&0i16 ).type_name(), "&i16" );
  a_id!( reflect( &&0i16 ).type_id(), core::any::TypeId::of::< &i16 >() );
  a_id!( reflect( &&0i16 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( reflect( &&0i32 ).is_container(), false );
  a_id!( reflect( &&0i32 ).len(), 0 );
  a_id!( reflect( &&0i32 ).type_name(), "&i32" );
  a_id!( reflect( &&0i32 ).type_id(), core::any::TypeId::of::< &i32 >() );
  a_id!( reflect( &&0i32 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( reflect( &&0i64 ).is_container(), false );
  a_id!( reflect( &&0i64 ).len(), 0 );
  a_id!( reflect( &&0i64 ).type_name(), "&i64" );
  a_id!( reflect( &&0i64 ).type_id(), core::any::TypeId::of::< &i64 >() );
  a_id!( reflect( &&0i64 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  //

  a_id!( reflect( &0u8 ).is_container(), false );
  a_id!( reflect( &0u8 ).len(), 0 );
  a_id!( reflect( &0u8 ).type_name(), "u8" );
  a_id!( reflect( &0u8 ).type_id(), core::any::TypeId::of::< u8 >() );
  a_id!( reflect( &0u8 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( reflect( &0u16 ).is_container(), false );
  a_id!( reflect( &0u16 ).len(), 0 );
  a_id!( reflect( &0u16 ).type_name(), "u16" );
  a_id!( reflect( &0u16 ).type_id(), core::any::TypeId::of::< u16 >() );
  a_id!( reflect( &0u16 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( reflect( &0u32 ).is_container(), false );
  a_id!( reflect( &0u32 ).len(), 0 );
  a_id!( reflect( &0u32 ).type_name(), "u32" );
  a_id!( reflect( &0u32 ).type_id(), core::any::TypeId::of::< u32 >() );
  a_id!( reflect( &0u32 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( reflect( &0u64 ).is_container(), false );
  a_id!( reflect( &0u64 ).len(), 0 );
  a_id!( reflect( &0u64 ).type_name(), "u64" );
  a_id!( reflect( &0u64 ).type_id(), core::any::TypeId::of::< u64 >() );
  a_id!( reflect( &0u64 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( reflect( &&0u8 ).is_container(), false );
  a_id!( reflect( &&0u8 ).len(), 0 );
  a_id!( reflect( &&0u8 ).type_name(), "&u8" );
  a_id!( reflect( &&0u8 ).type_id(), core::any::TypeId::of::< &u8 >() );
  a_id!( reflect( &&0u8 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( reflect( &&0u16 ).is_container(), false );
  a_id!( reflect( &&0u16 ).len(), 0 );
  a_id!( reflect( &&0u16 ).type_name(), "&u16" );
  a_id!( reflect( &&0u16 ).type_id(), core::any::TypeId::of::< &u16 >() );
  a_id!( reflect( &&0u16 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( reflect( &&0u32 ).is_container(), false );
  a_id!( reflect( &&0u32 ).len(), 0 );
  a_id!( reflect( &&0u32 ).type_name(), "&u32" );
  a_id!( reflect( &&0u32 ).type_id(), core::any::TypeId::of::< &u32 >() );
  a_id!( reflect( &&0u32 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( reflect( &&0u64 ).is_container(), false );
  a_id!( reflect( &&0u64 ).len(), 0 );
  a_id!( reflect( &&0u64 ).type_name(), "&u64" );
  a_id!( reflect( &&0u64 ).type_id(), core::any::TypeId::of::< &u64 >() );
  a_id!( reflect( &&0u64 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  //

  a_id!( reflect( &0.1f32 ).is_container(), false );
  a_id!( reflect( &0.1f32 ).len(), 0 );
  a_id!( reflect( &0.1f32 ).type_name(), "f32" );
  a_id!( reflect( &0.1f32 ).type_id(), core::any::TypeId::of::< f32 >() );
  a_id!( reflect( &0.1f32 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( reflect( &0.1f64 ).is_container(), false );
  a_id!( reflect( &0.1f64 ).len(), 0 );
  a_id!( reflect( &0.1f64 ).type_name(), "f64" );
  a_id!( reflect( &0.1f64 ).type_id(), core::any::TypeId::of::< f64 >() );
  a_id!( reflect( &0.1f64 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( reflect( &&0.1f32 ).is_container(), false );
  a_id!( reflect( &&0.1f32 ).len(), 0 );
  a_id!( reflect( &&0.1f32 ).type_name(), "&f32" );
  a_id!( reflect( &&0.1f32 ).type_id(), core::any::TypeId::of::< &f32 >() );
  a_id!( reflect( &&0.1f32 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  a_id!( reflect( &&0.1f64 ).is_container(), false );
  a_id!( reflect( &&0.1f64 ).len(), 0 );
  a_id!( reflect( &&0.1f64 ).type_name(), "&f64" );
  a_id!( reflect( &&0.1f64 ).type_id(), core::any::TypeId::of::< &f64 >() );
  a_id!( reflect( &&0.1f64 ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

  //

}
