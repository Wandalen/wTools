#[ allow( unused_imports ) ]
use super::*;

//

tests_impls_optional!
{

  //

  fn internals()
  {

    // test.case( "vector : construction" );

    // fields
    let former = Struct1::former();
    a_id!( former.storage.vec_1, None );
    a_id!( former.storage.hashmap_strings_1, None );
    a_id!( former.storage.hashset_strings_1, None );
    a_id!( former.context, None );

    // forming
    let command = Struct1::former().form();
    a_id!( command.vec_1, Vec::< String >::new() );
    a_id!( command.hashmap_strings_1, hmap!{} );
    a_id!( command.hashset_strings_1, hset![] );

    // performing
    let command = Struct1::former().perform();
    a_id!( command.vec_1, Vec::< String >::new() );
    a_id!( command.hashmap_strings_1, hmap!{} );
    a_id!( command.hashset_strings_1, hset![] );

    // ending
    let command = Struct1::former().end();
    a_id!( command.vec_1, Vec::< String >::new() );
    a_id!( command.hashmap_strings_1, hmap!{} );
    a_id!( command.hashset_strings_1, hset![] );

  }

  //

  fn new()
  {

    // former with explicit definition
    let former = Struct1::former();
    a_id!( print!( "{:?}", former.on_end ), print!( "{:?}", Some( the_module::ReturnPreformed ) ) );
    let former2 = Struct1Former::< Struct1FormerDefinition >::new( former::ReturnPreformed );
    a_id!( std::mem::size_of_val( &former ), std::mem::size_of_val( &former2 ) );

    // default parameters
    let former = Struct1::former();
    let former2 : Struct1Former = Struct1Former::new( former::ReturnPreformed );
    a_id!( std::mem::size_of_val( &former ), std::mem::size_of_val( &former2 ) );

    // default explicit params with wrapper and closure
    let got : Struct1 = Struct1Former
    ::< Struct1FormerWithClosure< (), Struct1 > >
    ::new( | storage, _context | { former::StoragePreform::preform( storage ) } )
    .vec_1().replace( vec![ "a".to_string(), "b".to_string() ] )
    .form();
    let exp : Struct1 = Struct1
    {
      vec_1 : vec![ "a".to_string(), "b".to_string() ],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{},
    };
    // a_id!( got, exp );
    // xxx : ?

  }

  //

  fn test_vector()
  {

    // test.case( "vector : implicit construction" );

    let command = Struct1::former()
    .vec_1().add( "ghi" ).add( "klm" ).end()
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
    .vec_1().add( "x" ).replace( vec![ "a".to_string(), "bc".to_string(), "def".to_string() ] ).end()
    .form();
    let expected = Struct1
    {
      vec_1 : vec![ "a".to_string(), "bc".to_string(), "def".to_string() ],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{},
    };
    a_id!( command, expected );

    // test.case( "vector : replace and add" );

    let command = Struct1::former()
    .vec_1().replace( vec![ "a".to_string(), "bc".to_string(), "def".to_string() ] ).add( "gh" ).end()
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
    .hashmap_strings_1().add( ( "k1".to_string(), "v1".to_string() ) ).add( ( "k2".to_string(), "v2".to_string() ) ).end()
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
    .hashmap_strings_1().add( ( "x".to_string(), "v1".to_string() ) ).replace( hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } ).end()
    .form()
    ;
    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
      hashset_strings_1 : hset!{},
    };
    a_id!( command, expected );

    // test.case( "replace and add" );

    let command = Struct1::former()
    .hashmap_strings_1().replace( hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } )
    .add( ( "k3".to_string(), "v3".to_string() ) ).end()
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
    .hashset_strings_1().add( "v1" ).add( "v2" ).end()
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
    .hashset_strings_1().add( "x" ).replace( hset!{ "v1".to_string(), "v2".to_string() } ).end()
    .form()
    ;
    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{ "v1".to_string(), "v2".to_string() },
    };
    a_id!( command, expected );

    // test.case( "replace and add" );

    let command = Struct1::former()
    .hashset_strings_1().replace( hset!{ "v1".to_string(), "v2".to_string() } ).add( "v3" ).end()
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
    .vec_1().add( "ghi" ).add( "klm" ).end()
    .hashmap_strings_1().add( ( "k1".to_string(), "v1".to_string() ) ).add( ( "k2".to_string(), "v2".to_string() ) ).end()
    .hashset_strings_1().add( "k1" ).end()
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
  new,
  test_vector,
  test_hashmap,
  test_hashset,
  test_complex,
}
