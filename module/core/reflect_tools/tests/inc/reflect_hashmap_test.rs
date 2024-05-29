use super::*;
pub use the_module::reflect;

#[ test ]
fn reflect_hashmap_test()
{
  use reflect::{ Entity, reflect, KeyVal, Primitive, Instance };
  use std::collections::HashMap;

  // for understanding
  println!( "TypeId< HashMap< i32, String > > : {:?}", core::any::TypeId::of::< HashMap< i32, String > >() );
  println!( "TypeId< &HashSMap< i32, String > > : {:?}", core::any::TypeId::of::< &HashMap< i32, String > >() );
  println!( "TypeId< HashMap< &i32, String > > : {:?}", core::any::TypeId::of::< HashMap< &i32, String > >() );

  let map : HashMap< i32, String > = [ ( 1, String::from( "one" ) ), ( 10, String::from( "ten" ) ) ].into_iter().collect();
  println!( "reflect( HashMap< i32, String > ) : {:?}", reflect::reflect( &map ) );
  println!( "HashMap< i32, String > : {:?}", reflect( &map ).type_id() );

  a_id!( reflect( &map ).is_container(), true );
  a_id!( reflect( &map ).len(), 2 );
  a_id!( reflect( &map ).type_name(), "std::collections::hash::map::HashMap<i32, alloc::string::String>" );
  a_id!( reflect( &map ).type_id(), core::any::TypeId::of::< HashMap< i32, String > >() );

  let expected = vec!
  [
    KeyVal{ key : Primitive::i32( 1 ), val : Box::new( < String as Instance >::Reflect() ) },
    KeyVal{ key : Primitive::i32( 10 ), val : Box::new( < String as Instance >::Reflect() ) },
  ];

  let elements = reflect( &map ).elements().collect::< Vec< _ > >();
  a_id!( elements.len(), 2 );
  a_true!( elements.contains( &expected[ 0 ] ) && elements.contains( &expected[ 1 ] ) );

  let empty_map : HashMap< String, String > = HashMap::new();
  a_id!( reflect( &empty_map ).is_container(), true );
  a_id!( reflect( &empty_map ).len(), 0 );
  a_id!( reflect( &empty_map ).type_name(), "std::collections::hash::map::HashMap<alloc::string::String, alloc::string::String>" );
  a_id!( reflect( &empty_map ).type_id(), core::any::TypeId::of::< HashMap< String, String > >() );

  a_id!( reflect( &empty_map ).elements().collect::< Vec< _ > >(), Vec::new() );
}