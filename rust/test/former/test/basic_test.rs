
// #[macro_use]
// extern crate maplit;
// extern crate former_runtime;

// use anyhow;

//

fn test_int() -> anyhow::Result< () >
{

  // test.case( "basic" );

  let command = Command::former()
  .int_1( 13 )
  .form();
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 13,
    string_1 : "".to_string(),
    vec_1 : vec![],
    hashmap_strings_1 : maplit::hashmap!{},
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  // test.case( "rewriting" );

  // should_throw( ||
  // {
  //   let _command = Command::former()
  //   .int_1( 1 )
  //   .int_1( 3 )
  //   .form();
  //   Ok( () )
  // })?;

  Ok( () )
}

//

fn test_string() -> anyhow::Result< () >
{

  // test.case( "string : object" );

  let command = Command::former()
  .string_1( "Abcd".to_string() )
  .form();
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "Abcd".to_string(),
    vec_1 : vec![],
    hashmap_strings_1 : maplit::hashmap!{},
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  // test.case( "string : slice" );

  let command = Command::former()
  .string_1( "Abcd" )
  .form();
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "Abcd".to_string(),
    vec_1 : vec![],
    hashmap_strings_1 : maplit::hashmap!{},
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  // test.case( "string : rewriting" );

  // should_throw( ||
  // {
  //   let _command = Command::former()
  //   .string_1( "dir1" )
  //   .string_1( "dir2" )
  //   .form();
  //   Ok( () )
  // })?;

  Ok( () )
}

//

fn test_vector() -> anyhow::Result< () >
{

  // test.case( "vector : implicit construction" );

  let command = Command::former()
  .vec_1().push( "ghi" ).push( "klm" ).end()
  .form()
  ;
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "".to_string(),
    vec_1 : vec![ "ghi".to_string(), "klm".to_string() ],
    hashmap_strings_1 : maplit::hashmap!{},
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  // test.case( "vector : replace" );

  let command = Command::former()
  .vec_1().replace( vec![ "a".to_string(), "bc".to_string(), "def".to_string() ] ).end()
  .form();
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "".to_string(),
    vec_1 : vec![ "a".to_string(), "bc".to_string(), "def".to_string() ],
    hashmap_strings_1 : maplit::hashmap!{},
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  // test.case( "vector : replace and push" );

  let command = Command::former()
  .vec_1().replace( vec![ "a".to_string(), "bc".to_string(), "def".to_string() ] ).push( "gh" ).end()
  .form();
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "".to_string(),
    vec_1 : vec![ "a".to_string(), "bc".to_string(), "def".to_string(), "gh".to_string() ],
    hashmap_strings_1 : maplit::hashmap!{},
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  Ok( () )
}

//

fn test_hashmap() -> anyhow::Result< () >
{

  // test.case( "implicit construction" );

  let command = Command::former()
  .hashmap_strings_1().insert( "k1", "v1" ).insert( "k2", "v2" ).end()
  .form()
  ;
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "".to_string(),
    vec_1 : vec![],
    hashmap_strings_1 : maplit::hashmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  // test.case( "replace" );

  let command = Command::former()
  .hashmap_strings_1().replace( maplit::hashmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } ).end()
  .form()
  ;
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "".to_string(),
    vec_1 : vec![],
    hashmap_strings_1 : maplit::hashmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  // test.case( "replace and insert" );

  let command = Command::former()
  .hashmap_strings_1().replace( maplit::hashmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } ).insert( "k3", "v3" ).end()
  .form()
  ;
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "".to_string(),
    vec_1 : vec![],
    hashmap_strings_1 : maplit::hashmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string(), "k3".to_string() => "v3".to_string() },
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  Ok( () )
}

//

fn test_optional_string() -> anyhow::Result< () >
{

  // test.case( "basic" );

  let command = Command::former()
  .string_optional_1( "dir1" )
  .form();
  dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "".to_string(),
    vec_1 : vec![],
    hashmap_strings_1 : maplit::hashmap!{},
    int_optional_1 : None,
    string_optional_1 : Some( "dir1".to_string() ),
  };
  assert_eq!( command, expected );

  // test.case( "none" );

  let command = Command::former()
  .form();
  dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "".to_string(),
    vec_1 : vec![],
    hashmap_strings_1 : maplit::hashmap!{},
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  // test.case( "optional : rewriting" );

  // should_throw( ||
  // {
  //   let _command = Command::former()
  //   .string_optional_1( "dir1" )
  //   .string_optional_1( "dir2" )
  //   .form();
  //   Ok( () )
  // })?;

  Ok( () )
}

//

fn main()
{
  test_int().unwrap();
  test_string().unwrap();
  test_vector().unwrap();
  test_hashmap().unwrap();
  test_optional_string().unwrap();
}

//

#[ test ]
fn main_test()
{
  test_int().unwrap();
  test_string().unwrap();
  test_vector().unwrap();
  test_hashmap().unwrap();
  test_optional_string().unwrap();
}
