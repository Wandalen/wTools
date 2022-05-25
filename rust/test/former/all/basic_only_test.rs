#[ allow( unused_imports ) ]
use super::*;

//

tests_impls!
{
  #[ test ]
  fn test_int()
  {

    // test.case( "basic" );

    let command = Struct1::former()
    .int_1( 13 )
    .form();
    // dbg!( &command );

    let expected = Struct1
    {
      int_1 : 13,
      string_1 : "".to_string(),
      int_optional_1 : None,
      string_optional_1 : None,
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{},
    };
    assert_eq!( command, expected );

    // test.case( "rewriting" );

    // should_throw( ||
    // {
    //   let _command = Struct1::former()
    //   .int_1( 1 )
    //   .int_1( 3 )
    //   .form();
    //   Ok( () )
    // })?;
  }

  //

  #[ test ]
  fn test_string()
  {

    // test.case( "string : object" );

    let command = Struct1::former()
    .string_1( "Abcd".to_string() )
    .form();
    // dbg!( &command );

    let expected = Struct1
    {
      int_1 : 0,
      string_1 : "Abcd".to_string(),
      int_optional_1 : None,
      string_optional_1 : None,
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{},
    };
    assert_eq!( command, expected );

    // test.case( "string : slice" );

    let command = Struct1::former()
    .string_1( "Abcd" )
    .form();
    // dbg!( &command );

    let expected = Struct1
    {
      int_1 : 0,
      string_1 : "Abcd".to_string(),
      int_optional_1 : None,
      string_optional_1 : None,
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{},
    };
    assert_eq!( command, expected );

    // test.case( "string : rewriting" );

    // should_throw( ||
    // {
    //   let _command = Struct1::former()
    //   .string_1( "dir1" )
    //   .string_1( "dir2" )
    //   .form();
    //   Ok( () )
    // })?;
  }

  //

  #[ test ]
  fn test_optional_string()
  {

    // test.case( "basic" );

    let command = Struct1::former()
    .string_optional_1( "dir1" )
    .form();
    // dbg!( &command );

    let expected = Struct1
    {
      int_1 : 0,
      string_1 : "".to_string(),
      int_optional_1 : None,
      string_optional_1 : Some( "dir1".to_string() ),
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{},
    };
    assert_eq!( command, expected );

    // test.case( "none" );

    let command = Struct1::former()
    .form();
    // dbg!( &command );

    let expected = Struct1
    {
      int_1 : 0,
      string_1 : "".to_string(),
      int_optional_1 : None,
      string_optional_1 : None,
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{},
    };
    assert_eq!( command, expected );

    // test.case( "optional : rewriting" );

    // should_throw( ||
    // {
    //   let _command = Struct1::former()
    //   .string_optional_1( "dir1" )
    //   .string_optional_1( "dir2" )
    //   .form();
    //   Ok( () )
    // })?;
  }

  //

  #[ test ]
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
      int_1 : 0,
      string_1 : "".to_string(),
      int_optional_1 : None,
      string_optional_1 : None,
      vec_1 : vec![ "ghi".to_string(), "klm".to_string() ],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{},
    };
    assert_eq!( command, expected );

    // test.case( "vector : replace" );

    let command = Struct1::former()
    .vec_1().replace( vec![ "a".to_string(), "bc".to_string(), "def".to_string() ] ).end()
    .form();
    // dbg!( &command );

    let expected = Struct1
    {
      int_1 : 0,
      string_1 : "".to_string(),
      int_optional_1 : None,
      string_optional_1 : None,
      vec_1 : vec![ "a".to_string(), "bc".to_string(), "def".to_string() ],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{},
    };
    assert_eq!( command, expected );

    // test.case( "vector : replace and push" );

    let command = Struct1::former()
    .vec_1().replace( vec![ "a".to_string(), "bc".to_string(), "def".to_string() ] ).push( "gh" ).end()
    .form();
    // dbg!( &command );

    let expected = Struct1
    {
      int_1 : 0,
      string_1 : "".to_string(),
      int_optional_1 : None,
      string_optional_1 : None,
      vec_1 : vec![ "a".to_string(), "bc".to_string(), "def".to_string(), "gh".to_string() ],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{},
    };
    assert_eq!( command, expected );
  }

  //

  #[ test ]
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
      int_1 : 0,
      string_1 : "".to_string(),
      int_optional_1 : None,
      string_optional_1 : None,
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
      hashset_strings_1 : hset!{},
    };
    assert_eq!( command, expected );

    // test.case( "replace" );

    let command = Struct1::former()
    .hashmap_strings_1().replace( hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } ).end()
    .form()
    ;
    // dbg!( &command );

    let expected = Struct1
    {
      int_1 : 0,
      string_1 : "".to_string(),
      int_optional_1 : None,
      string_optional_1 : None,
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
      hashset_strings_1 : hset!{},
    };
    assert_eq!( command, expected );

    // test.case( "replace and insert" );

    let command = Struct1::former()
    .hashmap_strings_1().replace( hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } ).insert( "k3", "v3" ).end()
    .form()
    ;
    // dbg!( &command );

    let expected = Struct1
    {
      int_1 : 0,
      string_1 : "".to_string(),
      int_optional_1 : None,
      string_optional_1 : None,
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string(), "k3".to_string() => "v3".to_string() },
      hashset_strings_1 : hset!{},
    };
    assert_eq!( command, expected );
  }

  //

  #[ test ]
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
      int_1 : 0,
      string_1 : "".to_string(),
      int_optional_1 : None,
      string_optional_1 : None,
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{ "v1".to_string(), "v2".to_string() },
    };
    assert_eq!( command, expected );

    // test.case( "replace" );

    let command = Struct1::former()
    .hashset_strings_1().replace( hset!{ "v1".to_string(), "v2".to_string() } ).end()
    .form()
    ;
    // dbg!( &command );

    let expected = Struct1
    {
      int_1 : 0,
      string_1 : "".to_string(),
      int_optional_1 : None,
      string_optional_1 : None,
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{ "v1".to_string(), "v2".to_string() },
    };
    assert_eq!( command, expected );

    // test.case( "replace and insert" );

    let command = Struct1::former()
    .hashset_strings_1().replace( hset!{ "v1".to_string(), "v2".to_string() } ).insert( "v3" ).end()
    .form()
    ;
    // dbg!( &command );

    let expected = Struct1
    {
      int_1 : 0,
      string_1 : "".to_string(),
      int_optional_1 : None,
      string_optional_1 : None,
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{ "v1".to_string(), "v2".to_string(), "v3".to_string() },
    };
    assert_eq!( command, expected );
  }

  //

  #[ test ]
  fn test_underscored_form()
  {
    // test.case( "basic" );
    let command = Struct1::former()
    .int_1( 13 )
    .form();

    let expected = Struct1
    {
      int_1 : 13,
      string_1 : "".to_string(),
      int_optional_1 : None,
      string_optional_1 : None,
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{},
    };
    assert_eq!( command, expected );
  }

  //

  #[ test ]
  fn test_complex()
  {
    let command = Struct1::former()
    .int_1( 13 )
    .string_1( "Abcd".to_string() )
    .vec_1().push( "ghi" ).push( "klm" ).end()
    .hashmap_strings_1().insert( "k1", "v1" ).insert( "k2", "v2" ).end()
    .string_optional_1( "dir1" )
    .form();
    // dbg!( &command );

    let expected = Struct1
    {
      int_1 : 13,
      string_1 : "Abcd".to_string(),
      int_optional_1 : None,
      string_optional_1 : Some( "dir1".to_string() ),
      vec_1 : vec![ "ghi".to_string(), "klm".to_string() ],
      hashmap_strings_1 : hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
      hashset_strings_1 : hset!{},
    };
    assert_eq!( command, expected );

    #[ cfg( debug_assertions ) ]
    println!("Debugging enabled");
    #[ cfg( not( debug_assertions ) ) ]
    println!("Debugging disabled");
  }
}

//

tests_index!
{
  test_int,
  test_string,
  test_optional_string,
  test_vector,
  test_hashmap,
  test_hashset,
  test_underscored_form,
  test_complex,
}
