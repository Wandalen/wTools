#[ allow( unused_imports ) ]
use super::*;

//

tests_impls_optional!
{

  //

  fn internals()
  {

    // test.case( "vector : construction" );

    let former = Struct1::former();
    a_id!( former.container.vec_1, None );
    a_id!( former.container.hashmap_strings_1, None );
    a_id!( former.container.hashset_strings_1, None );
    a_id!( former.context, None );
    a_id!( print!( "{:?}", former.on_end ), print!( "{:?}", Some( former::ReturnContainer ) ) );
    let former2 = Struct1Former::< Struct1, former::ReturnContainer >::new();
    a_id!( std::mem::size_of_val( &former ), std::mem::size_of_val( &former2 ) );

    let command = Struct1::former().form();
    a_id!( command.vec_1, Vec::< String >::new() );
    a_id!( command.hashmap_strings_1, hmap!{} );
    a_id!( command.hashset_strings_1, hset![] );

    let command = Struct1::former().perform();
    a_id!( command.vec_1, Vec::< String >::new() );
    a_id!( command.hashmap_strings_1, hmap!{} );
    a_id!( command.hashset_strings_1, hset![] );

    let command = Struct1::former().end();
    a_id!( command.vec_1, Vec::< String >::new() );
    a_id!( command.hashmap_strings_1, hmap!{} );
    a_id!( command.hashset_strings_1, hset![] );

  }

  //

  fn test_vector()
  {

    // test.case( "vector : implicit construction" );

    let command = Struct1::former()
    .vec_1().push( "ghi" ).push( "klm" ).end()
    .form()
    ;
    // dbg!( &command );

    let expected = Struct1
    {
      vec_1 : vec![ "ghi".to_string(), "klm".to_string() ],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{},
    };
    a_id!( command, expected );

    // test.case( "vector : replace" );

    let command = Struct1::former()
    .vec_1().replace( vec![ "a".to_string(), "bc".to_string(), "def".to_string() ] ).end()
    .form();
    let expected = Struct1
    {
      vec_1 : vec![ "a".to_string(), "bc".to_string(), "def".to_string() ],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{},
    };
    a_id!( command, expected );

    let command = Struct1::former()
    .vec_1().push( "x" ).replace( vec![ "a".to_string(), "bc".to_string(), "def".to_string() ] ).end()
    .form();
    let expected = Struct1
    {
      vec_1 : vec![ "a".to_string(), "bc".to_string(), "def".to_string() ],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{},
    };
    a_id!( command, expected );

    // test.case( "vector : replace and push" );

    let command = Struct1::former()
    .vec_1().replace( vec![ "a".to_string(), "bc".to_string(), "def".to_string() ] ).push( "gh" ).end()
    .form();
    // dbg!( &command );

    let expected = Struct1
    {
      vec_1 : vec![ "a".to_string(), "bc".to_string(), "def".to_string(), "gh".to_string() ],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{},
    };
    a_id!( command, expected );
  }

  //

  fn test_hashmap()
  {

    // test.case( "implicit construction" );

    let command = Struct1::former()
    .hashmap_strings_1().insert( "k1", "v1" ).insert( "k2", "v2" ).end()
    .form()
    ;
    // dbg!( &command );

    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
      hashset_strings_1 : hset!{},
    };
    a_id!( command, expected );

    // test.case( "replace" );

    let command = Struct1::former()
    .hashmap_strings_1().replace( hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } ).end()
    .form()
    ;
    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
      hashset_strings_1 : hset!{},
    };
    a_id!( command, expected );

    let command = Struct1::former()
    .hashmap_strings_1().insert( "x", "v1" ).replace( hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } ).end()
    .form()
    ;
    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
      hashset_strings_1 : hset!{},
    };
    a_id!( command, expected );

    // test.case( "replace and insert" );

    let command = Struct1::former()
    .hashmap_strings_1().replace( hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } ).insert( "k3", "v3" ).end()
    .form()
    ;
    // dbg!( &command );

    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string(), "k3".to_string() => "v3".to_string() },
      hashset_strings_1 : hset!{},
    };
    a_id!( command, expected );
  }

  //

  fn test_hashset()
  {

    // test.case( "implicit construction" );

    let command = Struct1::former()
    .hashset_strings_1().insert( "v1" ).insert( "v2" ).end()
    .form()
    ;
    // dbg!( &command );

    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{ "v1".to_string(), "v2".to_string() },
    };
    a_id!( command, expected );

    // test.case( "replace" );

    let command = Struct1::former()
    .hashset_strings_1().replace( hset!{ "v1".to_string(), "v2".to_string() } ).end()
    .form()
    ;
    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{ "v1".to_string(), "v2".to_string() },
    };
    a_id!( command, expected );

    let command = Struct1::former()
    .hashset_strings_1().insert( "x" ).replace( hset!{ "v1".to_string(), "v2".to_string() } ).end()
    .form()
    ;
    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{ "v1".to_string(), "v2".to_string() },
    };
    a_id!( command, expected );

    // test.case( "replace and insert" );

    let command = Struct1::former()
    .hashset_strings_1().replace( hset!{ "v1".to_string(), "v2".to_string() } ).insert( "v3" ).end()
    .form()
    ;
    // dbg!( &command );

    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{ "v1".to_string(), "v2".to_string(), "v3".to_string() },
    };
    a_id!( command, expected );
  }

  //

  fn test_complex()
  {

    let command = Struct1::former()
    .vec_1().push( "ghi" ).push( "klm" ).end()
    .hashmap_strings_1().insert( "k1", "v1" ).insert( "k2", "v2" ).end()
    .hashset_strings_1().insert( "k1" ).end()
    .form();
    // dbg!( &command );

    let expected = Struct1
    {
      vec_1 : vec![ "ghi".to_string(), "klm".to_string() ],
      hashmap_strings_1 : hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
      hashset_strings_1 : hset!{ "k1".to_string() },
    };
    a_id!( command, expected );

  }

}

//

tests_index!
{
  internals,
  test_vector,
  test_hashmap,
  test_hashset,
  test_complex,
}
