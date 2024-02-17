#[ test ]
fn reflect_struct_in_struct()
{
  use reflect::Entity;

  let ins = Struct1
  {
    f1 : 1,
    f2 : "2".into(),
    f3 : Struct2 { s1 : 10, s2 : "20".into(), s3 : "30" },
  };

  a_id!( ins.reflect_is_container(), true );
  a_id!( ins.reflect_len(), 3 );
  a_id!( ins.reflect_type_name(), "derive_tests::inc::reflect_struct_in_struct_manual_test::Struct1" );
  let names = ins.reflect_elements().map( | e | e.key ).collect::< Vec< _ > >();
  a_id!( names, vec![ "f1", "f2", "f3" ] );
  let types = ins.reflect_elements().map( | e | e.val.reflect_type_name() ).collect::< Vec< _ > >();
  a_id!( types, vec![ "i32", "alloc::string::String", "derive_tests::inc::reflect_struct_in_struct_manual_test::Struct2" ] );

  let f3 = ins.reflect_elements().skip( 2 ).next().unwrap();
  a_id!( f3.key, "f3" );
  a_id!( f3.val.reflect_is_container(), true );
  a_id!( f3.val.reflect_len(), 3 );
  a_id!( f3.val.reflect_type_name(), "derive_tests::inc::reflect_struct_in_struct_manual_test::Struct2" );
  let names = f3.val.reflect_elements().map( | e | e.key ).collect::< Vec< _ > >();
  a_id!( names, vec![ "s1", "s2", "s3" ] );
  let types = f3.val.reflect_elements().map( | e | e.val.reflect_type_name() ).collect::< Vec< _ > >();
  a_id!( types, vec![ "i32", "alloc::string::String", "&str" ] );

}
