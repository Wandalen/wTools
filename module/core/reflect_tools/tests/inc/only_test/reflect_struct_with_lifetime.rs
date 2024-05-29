#[ test ]
fn reflect_struct_with_lifetime()
{
  use reflect::Entity;

  // assumptions
  a_id!( core::any::TypeId::of::< &'static str >(), core::any::TypeId::of::< &str >() );

  // structure
  let x = 1;
  let z = "3";
  let ins = Struct1
  {
    f1 : &x,
    f2 : 2,
    f3 : &z,
  };

  // for information
  println!( "Struct1 : {:?}", reflect( &ins ).type_id() );
  println!( "Struct1.f1 : {:?}", reflect( &ins ).elements().next().unwrap().val.type_id() );
  println!( "Struct1.f2 : {:?}", reflect( &ins ).elements().skip( 1 ).next().unwrap().val.type_id() );
  println!( "Struct1.f3 : {:?}", reflect( &ins ).elements().skip( 2 ).next().unwrap().val.type_id() );

  println!( "i32.type_id : {:?}", reflect( &1i32 ).type_id() );
  println!( "i32.type_name : {:?}", reflect( &1i32 ).type_name() );
  println!( "&i32.type_id : {:?}", reflect( &&1i32 ).type_id() );
  println!( "&i32.type_name : {:?}", reflect( &&1i32 ).type_name() );

  // inspection of structure
  a_id!( reflect::reflect( &ins ).is_container(), true );
  a_id!( reflect::reflect( &ins ).len(), 3 );
  a_id!( reflect::reflect( &ins ).type_name(), "tests::inc::reflect_struct_with_lifetime_manual_test::Struct1" );
  a_id!( reflect::reflect( &ins ).type_id(), core::any::TypeId::of::< Struct1< 'static, 'static > >() );
  let names = reflect::reflect( &ins ).elements().map( | e | e.key ).collect::< Vec< _ > >();
  a_id!( names, vec![ reflect::Primitive::str( "f1" ), reflect::Primitive::str( "f2" ), reflect::Primitive::str( "f3" ) ] );
  let types = reflect::reflect( &ins ).elements().map( | e | e.val.type_name() ).collect::< Vec< _ > >();
  a_id!( types, vec![ "&i32", "i32", "&str" ] );

  // inspection of a field
  let f1 = reflect::reflect( &ins ).elements().next().unwrap();
  a_id!( f1.key, reflect::Primitive::str( "f1" ) );
  a_id!( f1.val.is_container(), false );
  a_id!( f1.val.len(), 0 );
  a_id!( f1.val.type_name(), "&i32" );
  a_id!( f1.val.type_id(), core::any::TypeId::of::< &'static i32 >() );
  a_id!( f1.val.elements().collect::< Vec< _ > >(), vec![] );

}
