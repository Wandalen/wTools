use super::*;
pub use the_module::reflect;

#[ test ]
fn reflect_hashset_test()
{
  use reflect::{ Entity, reflect, KeyVal, Primitive, Instance };
  use std::collections::HashSet;

  // for understanding
  println!( "TypeId< HashSet< i32 > > : {:?}", core::any::TypeId::of::< HashSet< i32 > >() );
  println!( "TypeId< &HashSet< i32 > > : {:?}", core::any::TypeId::of::< &HashSet< i32 > >() );
  println!( "TypeId< HashSet< &i32 > > : {:?}", core::any::TypeId::of::< HashSet< &i32 > >() );

  let set : HashSet< i32 > = [ 1, 10, 100 ].into_iter().collect();
  println!( "reflect( HashSet< i32 > ) : {:?}", reflect::reflect( &set ) );
  println!( "HashSet< i32 > : {:?}", reflect( &set ).type_id() );

  a_id!( reflect( &set ).is_container(), true );
  a_id!( reflect( &set ).len(), 3 );
  a_id!( reflect( &set ).type_name(), "std::collections::hash::set::HashSet<i32>" );
  a_id!( reflect( &set ).type_id(), core::any::TypeId::of::< HashSet< i32 > >() );

  let expected = vec!
  [
    KeyVal{ key : Primitive::usize( 0 ), val : Box::new( < i32 as Instance >::Reflect() ) },
    KeyVal{ key : Primitive::usize( 1 ), val : Box::new( < i32 as Instance >::Reflect() ) },
    KeyVal{ key : Primitive::usize( 2 ), val : Box::new( < i32 as Instance >::Reflect() ) },
  ];
  a_id!( reflect( &set ).elements().collect::< Vec< _ > >(), expected );

  let empty_set : HashSet< String > = HashSet::new();
  a_id!( reflect( &empty_set ).is_container(), true );
  a_id!( reflect( &empty_set ).len(), 0 );
  a_id!( reflect( &empty_set ).type_name(), "std::collections::hash::set::HashSet<alloc::string::String>" );
  a_id!( reflect( &empty_set ).type_id(), core::any::TypeId::of::< HashSet< String > >() );

  a_id!( reflect( &empty_set ).elements().collect::< Vec< _ > >(), Vec::new() );
}