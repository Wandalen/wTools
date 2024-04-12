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
    a_id!( former.storage.hashmap_1, None );
    a_id!( former.storage.hashset_1, None );
    a_id!( former.context, None );

    // form
    let got = Struct1::former().form();
    let exp = Struct1::default();
    a_id!( got, exp );

    // preform
    let got = Struct1::former().preform();
    let exp = Struct1::default();
    a_id!( got, exp );

    // perform
    let got = Struct1::former().perform();
    let exp = Struct1::default();
    a_id!( got, exp );

    // end
    let got = Struct1::former().end();
    let exp = Struct1::default();
    a_id!( got, exp );

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

    // closure without helper
    let got : Struct1 = Struct1Former
    ::< Struct1FormerDefinition< _, _, former::FormingEndClosure< Struct1FormerDefinitionTypes< (), Struct1 > > > >
    ::new( | storage : Struct1FormerStorage, _context | { former::StoragePreform::preform( storage ) } )
    .vec_1().replace( vec![ "a".to_string(), "b".to_string() ] ).end()
    .form();
    let exp : Struct1 = Struct1
    {
      vec_1 : vec![ "a".to_string(), "b".to_string() ],
      hashmap_1 : hmap!{},
      hashset_1 : hset!{},
    };
    a_id!( got, exp );

    // closure with helper
    let got : Struct1 = Struct1Former
    ::< Struct1FormerWithClosure< (), Struct1 > >
    ::new( | storage : Struct1FormerStorage, _context | { former::StoragePreform::preform( storage ) } )
    .vec_1().replace( vec![ "a".to_string(), "b".to_string() ] ).end()
    .form();
    let exp : Struct1 = Struct1
    {
      vec_1 : vec![ "a".to_string(), "b".to_string() ],
      hashmap_1 : hmap!{},
      hashset_1 : hset!{},
    };
    a_id!( got, exp );

  }

  //

  fn field_forming_end()
  {

    // use super::*;
    let _got = Struct1FormerVec1End;
    let _got = Struct1FormerHashmap1End;
    let _got = Struct1FormerHashset1End;

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
      hashmap_1 : hmap!{},
      hashset_1 : hset!{},
    };
    a_id!( command, expected );

    // test.case( "vector : replace" );

    let command = Struct1::former()
    .vec_1().replace( vec![ "a".to_string(), "bc".to_string(), "def".to_string() ] ).end()
    .form();
    let expected = Struct1
    {
      vec_1 : vec![ "a".to_string(), "bc".to_string(), "def".to_string() ],
      hashmap_1 : hmap!{},
      hashset_1 : hset!{},
    };
    a_id!( command, expected );

    let command = Struct1::former()
    .vec_1().add( "x" ).replace( vec![ "a".to_string(), "bc".to_string(), "def".to_string() ] ).end()
    .form();
    let expected = Struct1
    {
      vec_1 : vec![ "a".to_string(), "bc".to_string(), "def".to_string() ],
      hashmap_1 : hmap!{},
      hashset_1 : hset!{},
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
      hashmap_1 : hmap!{},
      hashset_1 : hset!{},
    };
    a_id!( command, expected );
  }

  //

  fn test_hashmap()
  {

    // test.case( "implicit construction" );

    let command = Struct1::former()
    .hashmap_1().add( ( "k1".to_string(), "v1".to_string() ) ).add( ( "k2".to_string(), "v2".to_string() ) ).end()
    .form()
    ;
    // dbg!( &command );

    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_1 : hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
      hashset_1 : hset!{},
    };
    a_id!( command, expected );

    // test.case( "replace" );

    let command = Struct1::former()
    .hashmap_1().replace( hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } ).end()
    .form()
    ;
    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_1 : hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
      hashset_1 : hset!{},
    };
    a_id!( command, expected );

    let command = Struct1::former()
    .hashmap_1().add( ( "x".to_string(), "v1".to_string() ) ).replace( hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } ).end()
    .form()
    ;
    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_1 : hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
      hashset_1 : hset!{},
    };
    a_id!( command, expected );

    // test.case( "replace and add" );

    let command = Struct1::former()
    .hashmap_1().replace( hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } )
    .add( ( "k3".to_string(), "v3".to_string() ) ).end()
    .form()
    ;
    // dbg!( &command );

    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_1 : hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string(), "k3".to_string() => "v3".to_string() },
      hashset_1 : hset!{},
    };
    a_id!( command, expected );
  }

  //

  fn test_hashset()
  {

    // test.case( "implicit construction" );

    let command = Struct1::former()
    .hashset_1().add( "v1" ).add( "v2" ).end()
    .form()
    ;
    // dbg!( &command );

    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_1 : hmap!{},
      hashset_1 : hset!{ "v1".to_string(), "v2".to_string() },
    };
    a_id!( command, expected );

    // test.case( "replace" );

    let command = Struct1::former()
    .hashset_1().replace( hset!{ "v1".to_string(), "v2".to_string() } ).end()
    .form()
    ;
    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_1 : hmap!{},
      hashset_1 : hset!{ "v1".to_string(), "v2".to_string() },
    };
    a_id!( command, expected );

    let command = Struct1::former()
    .hashset_1().add( "x" ).replace( hset!{ "v1".to_string(), "v2".to_string() } ).end()
    .form()
    ;
    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_1 : hmap!{},
      hashset_1 : hset!{ "v1".to_string(), "v2".to_string() },
    };
    a_id!( command, expected );

    // test.case( "replace and add" );

    let command = Struct1::former()
    .hashset_1().replace( hset!{ "v1".to_string(), "v2".to_string() } ).add( "v3" ).end()
    .form()
    ;
    // dbg!( &command );

    let expected = Struct1
    {
      vec_1 : vec![],
      hashmap_1 : hmap!{},
      hashset_1 : hset!{ "v1".to_string(), "v2".to_string(), "v3".to_string() },
    };
    a_id!( command, expected );
  }

  //

  fn test_complex()
  {

    let command = Struct1::former()
    .vec_1().add( "ghi" ).add( "klm" ).end()
    .hashmap_1().add( ( "k1".to_string(), "v1".to_string() ) ).add( ( "k2".to_string(), "v2".to_string() ) ).end()
    .hashset_1().add( "k1" ).end()
    .form();
    // dbg!( &command );

    let expected = Struct1
    {
      vec_1 : vec![ "ghi".to_string(), "klm".to_string() ],
      hashmap_1 : hmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
      hashset_1 : hset!{ "k1".to_string() },
    };
    a_id!( command, expected );

  }

}

//

tests_index!
{
  internals,
  new,
  field_forming_end,
  test_vector,
  test_hashmap,
  test_hashset,
  test_complex,
}
