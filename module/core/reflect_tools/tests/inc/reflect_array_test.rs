use super::*;
pub use the_module::reflect;

#[ test ]
fn reflect_array_test()
{
  use reflect::{ Entity, reflect, KeyVal, Instance, Primitive };

  // for understanding
  println!( "TypeId< [ i32; 3 ] > : {:?}", core::any::TypeId::of::< [ i32; 3 ] >() );
  println!( "TypeId< [ &i32; 3 ] > : {:?}", core::any::TypeId::of::< [ &i32; 3 ] >() );
  let arr = [ 1i32, 2, 3 ];
  println!( "reflect( [ i32; 3 ] ) : {:?}", reflect::reflect( &arr ) );

  a_id!( reflect( &arr ).is_container(), true );
  a_id!( reflect( &arr ).len(), 3 );
  a_id!( reflect( &arr ).type_name(), "[i32; 3]" );
  a_id!( reflect( &arr ).type_id(), core::any::TypeId::of::< [ i32; 3 ] >() );

  let expected = vec!
  [
    KeyVal{ key : Primitive::usize( 0 ), val : Box::new( < i32 as Instance >::Reflect() ) },
    KeyVal{ key : Primitive::usize( 1 ), val : Box::new( < i32 as Instance >::Reflect() ) },
    KeyVal{ key : Primitive::usize( 2 ), val : Box::new( < i32 as Instance >::Reflect() ) },
  ];

  a_id!( reflect( &arr ).elements().collect::< Vec< _ > >(), expected );
}