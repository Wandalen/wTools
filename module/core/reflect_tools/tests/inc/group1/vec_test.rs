use super::*;
pub use the_module::reflect;

#[ test ]
fn reflect_vec_test()
{
  use reflect::{ Entity, reflect, KeyVal, Primitive, Instance };

  // for understanding
  println!( "TypeId< Vec< i32 > > : {:?}", core::any::TypeId::of::< Vec< i32 > >() );
  println!( "TypeId< &Vec< i32 > > : {:?}", core::any::TypeId::of::< &Vec< i32 > >() );
  println!( "TypeId< Vec< &i32 > > : {:?}", core::any::TypeId::of::< Vec< &i32 > >() );

  let vec : Vec< i32 > = vec![ 1, 2, 3 ];
  println!( "reflect( Vec< i32 > ) : {:?}", reflect::reflect( &vec ) );
  println!( "Vec< i32 > : {:?}", reflect( &vec ).type_id() );

  a_id!( reflect( &vec ).is_container(), true );
  a_id!( reflect( &vec ).len(), 3 );
  a_id!( reflect( &vec ).type_name(), "alloc::vec::Vec<i32>" );
  a_id!( reflect( &vec ).type_id(), core::any::TypeId::of::< Vec< i32 > >() );

  let expected = vec!
  [
    KeyVal{ key : Primitive::usize( 0 ), val : Box::new( < i32 as Instance >::Reflect() ) },
    KeyVal{ key : Primitive::usize( 1 ), val : Box::new( < i32 as Instance >::Reflect() ) },
    KeyVal{ key : Primitive::usize( 2 ), val : Box::new( < i32 as Instance >::Reflect() ) },
  ];
  a_id!( reflect( &vec ).elements().collect::< Vec< _ > >(), expected );

  let vec : Vec< String > = Vec::new();
  a_id!( reflect( &vec ).is_container(), true );
  a_id!( reflect( &vec ).len(), 0 );
  a_id!( reflect( &vec ).type_name(), "alloc::vec::Vec<alloc::string::String>" );
  a_id!( reflect( &vec ).type_id(), core::any::TypeId::of::< Vec< String > >() );

  a_id!( reflect( &vec ).elements().collect::< Vec< _ > >(), Vec::new() );
}