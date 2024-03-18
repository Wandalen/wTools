use super::*;
pub use the_module::reflect;

#[ test ]
fn reflect_slice_test()
{
  use reflect::{ Entity, reflect, KeyVal, Primitive, Instance };

  // for understanding
  println!( "TypeId< &[ i32 ] > : {:?}", core::any::TypeId::of::< [ i32 ] >() );
  println!( "TypeId< &[ i32 ] > : {:?}", core::any::TypeId::of::< &[ i32 ] >() );
  println!( "TypeId< &[ &i32 ] > : {:?}", core::any::TypeId::of::< &[ &i32 ] >() ); // qqq : qqq  fro Yuliia : problem. should be distinct id

  let slice : &[ i32 ] = &[ 1, 2, 3 ];
  println!( "reflect( &[ i32 ] ) : {:?}", reflect::reflect( &slice ) );
  println!( "&[ i32 ] : {:?}", reflect( &slice ).type_id() );

  a_id!( reflect( &slice ).is_container(), true );
  a_id!( reflect( &slice ).len(), 3 );
  a_id!( reflect( &slice ).type_name(), "&[i32]" );
  a_id!( reflect( &slice ).type_id(), core::any::TypeId::of::< &[ i32 ] >() );

  let expected = vec!
  [
    KeyVal{ key : Primitive::usize( 0 ), val : Box::new( < i32 as Instance >::Reflect() ) },
    KeyVal{ key : Primitive::usize( 1 ), val : Box::new( < i32 as Instance >::Reflect() ) },
    KeyVal{ key : Primitive::usize( 2 ), val : Box::new( < i32 as Instance >::Reflect() ) },
  ];
  a_id!( reflect( &slice ).elements().collect::< Vec< _ > >(), expected );
}
