#[ test ]
fn reflect_basic_test()
{
  use reflect::{ Instance, Entity };

  let ins = Struct1
  {
    f1 : 1,
    f2 : "2".into(),
    f3 : "3",
  };

  a_id!( ins.reflect().is_container(), true );
  a_id!( ins.reflect().len(), 3 );
  a_id!( ins.reflect().type_name(), "derive_tests::inc::reflect_struct_manual_test::Struct1" );
  let names = ins.reflect().elements().map( | e | e.key ).collect::< Vec< _ > >();
  a_id!( names, vec![ "f1", "f2", "f3" ] );
  let types = ins.reflect().elements().map( | e | e.val.type_name() ).collect::< Vec< _ > >();
  a_id!( types, vec![ "i32", "alloc::string::String", "&str" ] );

  let f1 = ins.reflect().elements().next().unwrap();
  a_id!( f1.key, "f1" );
  a_id!( f1.val.is_container(), false );
  a_id!( f1.val.len(), 0 );
  a_id!( f1.val.type_name(), "i32" );
  a_id!( f1.val.elements().collect::< Vec< _ > >(), vec![] );

}
