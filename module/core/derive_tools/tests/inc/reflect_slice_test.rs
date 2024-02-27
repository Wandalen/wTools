use super::*;
pub use TheModule::reflect;

#[ test ]
fn reflect_slice_test()
{
  use reflect::{ Entity, reflect };

  // for understanding
  println!( "TypeId< &[ i32 ] > : {:?}", core::any::TypeId::of::< [ i32 ] >() );
  println!( "TypeId< &[ i32 ] > : {:?}", core::any::TypeId::of::< &[ i32 ] >() );
  println!( "TypeId< &[ &i32 ] > : {:?}", core::any::TypeId::of::< &[ &i32 ] >() ); // qqq : qqq  fro Yuliia : problem. should be distinct id
  println!( "TypeId< i32 > : {:?}", core::any::TypeId::of::< i32 >() );
  println!( "TypeId< &i32 > : {:?}", core::any::TypeId::of::< & i32 >() ); 
  let vec = vec![ 1i32, 2, 3 ];
  let slice : &[ i32 ] = &[ 1, 2, 3 ];
  println!( "reflect( &[ i32 ] ) : {:?}", reflect::reflect( &slice ) );

  println!( "&[ i32 ] : {:?}", reflect( &slice ).type_id() );

  a_id!( reflect( &slice ).is_container(), true );
  // a_id!( reflect( &slice ).len(), 3 );
  a_id!( reflect( &slice ).type_name(), "&[i32]" );
  // a_id!( reflect( &slice ).type_id(), core::any::TypeId::of::< &i64 >() );
  // a_id!( reflect( &slice ).elements().collect::< Vec< _ > >(), Vec::< _ >::new() );

}

#[ test ]
fn reflect_array_test()
{
  use reflect::{ Entity, reflect, KeyVal, Instance, Primitive };

  // for understanding
  println!( "TypeId< [ i32; 2 ] > : {:?}", core::any::TypeId::of::< [ i32; 2 ] >() );
  println!( "TypeId< [ &i32; 2 ] > : {:?}", core::any::TypeId::of::< [ &i32; 3 ] >() );
  let arr = [ 1i32, 2, 3 ];
  println!( "reflect( [ i32; 3 ] ) : {:?}", reflect::reflect( &arr ) );

  a_id!( reflect( &arr ).is_container(), true );
  a_id!( reflect( &arr ).len(), 3 );
  a_id!( reflect( &arr ).type_name(), "[i32; 3]" );
  a_id!( reflect( &arr ).type_id(), core::any::TypeId::of::< [ i32; 3 ] >() );
  a_id!( reflect( &arr ).elements().collect::< Vec< _ > >()[ 0 ], KeyVal{ key : Primitive::usize( 0 ), val : Box::new( < i32 as Instance >::Reflect() ) } );

}