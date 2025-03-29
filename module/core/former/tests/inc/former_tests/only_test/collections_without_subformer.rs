#[ allow( unused_imports ) ]
use super::*;

//

tests_impls!
{

  //

  fn internals()
  {

    // test.case( "vector : construction" );

    let former = Struct1::former();
    a_id!( former.storage.vec_1, None );
    a_id!( former.storage.hashmap_1, None );
    a_id!( former.storage.hashset_1, None );
    a_id!( former.context, None );
    a_id!( print!( "{:?}", former.on_end ), print!( "{:?}", Some( the_module::ReturnPreformed ) ) );
    let former2 = Struct1Former::< Struct1FormerDefinition >::new_coercing( the_module::ReturnPreformed );
    a_id!( ::std::mem::size_of_val( &former ), ::std::mem::size_of_val( &former2 ) );

    let command = Struct1::former().form();
    a_id!( command.vec_1, ::collection_tools::Vec::< String >::new() );
    a_id!( command.hashmap_1, ::collection_tools::hmap!{} );
    a_id!( command.hashset_1, ::collection_tools::hset![] );

    let command = Struct1::former().perform();
    a_id!( command.vec_1, ::collection_tools::Vec::< String >::new() );
    a_id!( command.hashmap_1, ::collection_tools::hmap!{} );
    a_id!( command.hashset_1, ::collection_tools::hset![] );

    let command = Struct1::former().end();
    a_id!( command.vec_1, ::collection_tools::Vec::< String >::new() );
    a_id!( command.hashmap_1, ::collection_tools::hmap!{} );
    a_id!( command.hashset_1, ::collection_tools::hset![] );

  }

  //

  fn test_vector()
  {

    // test.case( "vector : construction" );

    let command = Struct1::former()
    .vec_1( ::collection_tools::vec![ "ghi".to_string(), "klm".to_string() ] )
    .form()
    ;
    // dbg!( &command );

    let expected = Struct1
    {
      vec_1 : ::collection_tools::vec![ "ghi".to_string(), "klm".to_string() ],
      hashmap_1 : ::collection_tools::hmap!{},
      hashset_1 : ::collection_tools::hset!{},
    };
    a_id!( command, expected );
  }

  //

  fn test_hashmap()
  {

    // test.case( "construction" );

    let command = Struct1::former()
    .hashmap_1( ::collection_tools::hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } )
    .form()
    ;
    // dbg!( &command );

    let expected = Struct1
    {
      vec_1 : ::collection_tools::vec![],
      hashmap_1 : ::collection_tools::hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
      hashset_1 : ::collection_tools::hset!{},
    };
    a_id!( command, expected );
  }

  //

  fn test_hashset()
  {

    // test.case( "construction" );
    let command = Struct1::former()
    .hashset_1( ::collection_tools::hset!{ "v1".to_string(), "v2".to_string() } )
    .form()
    ;
    // dbg!( &command );

    let expected = Struct1
    {
      vec_1 : ::collection_tools::vec![],
      hashmap_1 : ::collection_tools::hmap!{},
      hashset_1 : ::collection_tools::hset!{ "v1".to_string(), "v2".to_string() },
    };
    a_id!( command, expected );
  }

  //

  fn test_underscored_form()
  {
    // test.case( "basic" );
    let command = Struct1::former()
    .form();

    let expected = Struct1
    {
      vec_1 : ::collection_tools::vec![],
      hashmap_1 : ::collection_tools::hmap!{},
      hashset_1 : ::collection_tools::hset!{},
    };
    a_id!( command, expected );
  }

  //

  fn test_complex()
  {
    let command = Struct1::former()
    .vec_1( ::collection_tools::vec![ "ghi".to_string(), "klm".to_string() ] )
    .hashmap_1( ::collection_tools::hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } )
    .form();
    // dbg!( &command );

    let expected = Struct1
    {
      vec_1 : ::collection_tools::vec![ "ghi".to_string(), "klm".to_string() ],
      hashmap_1 : ::collection_tools::hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
      hashset_1 : ::collection_tools::hset!{},
    };
    a_id!( command, expected );

    #[ cfg( debug_assertions ) ]
    println!( "Debugging enabled" );
    #[ cfg( not( debug_assertions ) ) ]
    println!( "Debugging disabled" );
  }
}

//

tests_index!
{

  internals,
  test_vector,
  test_hashmap,
  test_hashset,
  test_underscored_form,
  test_complex,

}
