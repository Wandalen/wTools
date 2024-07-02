#[ test ]
fn reflect_basic_test()
{
  use reflect::Entity;

  let ins = Struct1
  {
    f1 : 1,
    f2 : "2".into(),
    f3 : "3",
  };

  a_id!( reflect::reflect( &ins ).is_container(), true );
  a_id!( reflect::reflect( &ins ).len(), 3 );
  a_id!( reflect::reflect( &ins ).type_name(), "tests::inc::group1::struct_manual_test::Struct1" );
  let names = reflect::reflect( &ins ).elements().map( | e | e.key ).collect::< Vec< _ > >();
  a_id!( names, vec![ reflect::Primitive::str( "f1" ), reflect::Primitive::str( "f2" ), reflect::Primitive::str( "f3" ) ] );
  let types = reflect::reflect( &ins ).elements().map( | e | e.val.type_name() ).collect::< Vec< _ > >();
  a_id!( types, vec![ "i32", "alloc::string::String", "&str" ] );

  let f1 = reflect::reflect( &ins ).elements().next().unwrap();
  a_id!( f1.key, reflect::Primitive::str( "f1" ) );
  a_id!( f1.val.is_container(), false );
  a_id!( f1.val.len(), 0 );
  a_id!( f1.val.type_name(), "i32" );
  a_id!( f1.val.elements().collect::< Vec< _ > >(), vec![] );

}
