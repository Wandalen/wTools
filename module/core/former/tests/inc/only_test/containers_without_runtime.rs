#[ allow( unused_imports ) ]
use super::*;

//

tests_impls!
{

  //

  fn test_vector()
  {

    // test.case( "vector : construction" );

    let command = Struct1::former()
    .vec_1( vec![ "ghi".to_string(), "klm".to_string() ] )
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
  }

  //

  fn test_hashmap()
  {

    // test.case( "construction" );

    let command = Struct1::former()
    .hashmap_strings_1( hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } )
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
  }

  //

  fn test_hashset()
  {

    // test.case( "construction" );
    let command = Struct1::former()
    .hashset_strings_1( hset!{ "v1".to_string(), "v2".to_string() } )
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
  }

  //

  fn test_underscored_form()
  {
    // test.case( "basic" );
    let command = Struct1::former()
    .form();

    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{},
    };
    a_id!( command, expected );
  }

  //

  fn test_complex()
  {
    let command = Struct1::former()
    .vec_1( vec![ "ghi".to_string(), "klm".to_string() ] )
    .hashmap_strings_1( hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } )
    .form();
    // dbg!( &command );

    let expected = Struct1
    {
      vec_1 : vec![ "ghi".to_string(), "klm".to_string() ],
      hashmap_strings_1 : hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
      hashset_strings_1 : hset!{},
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
  test_vector,
  test_hashmap,
  test_hashset,
  test_underscored_form,
  test_complex,
}
