#[ allow( unused_imports ) ]
use super::*;

//

tests_impls!
{

  //

  fn internals()
  {

    // // test.case( "vector : construction" );
    // int_1,
    // string_1,
    // int_optional_1,
    // string_optional_1,

    let former = Struct1::former();
    a_id!( former.container.int_1, None );
    a_id!( former.container.string_1, None );
    a_id!( former.container.int_optional_1, None );
    a_id!( former.container.string_optional_1, None );
    a_id!( former.context, None );
    a_id!( print!( "{:?}", former.on_end ), print!( "{:?}", Some( former::ReturnContainer ) ) );
    let former2 = Struct1Former::< Struct1, former::ReturnContainer >::new();
    a_id!( std::mem::size_of_val( &former ), std::mem::size_of_val( &former2 ) );

    let command = Struct1::former().form();
    a_id!( command.int_1, 0 );
    a_id!( command.string_1, "".to_string() );
    a_id!( command.int_optional_1, None );
    a_id!( command.string_optional_1, None );

    let command = Struct1::former().perform();
    a_id!( command.int_1, 0 );
    a_id!( command.string_1, "".to_string() );
    a_id!( command.int_optional_1, None );
    a_id!( command.string_optional_1, None );

    let command = Struct1::former().end();
    a_id!( command.int_1, 0 );
    a_id!( command.string_1, "".to_string() );
    a_id!( command.int_optional_1, None );
    a_id!( command.string_optional_1, None );

  }

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
    };
    a_id!( command, expected );

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
    };
    a_id!( command, expected );

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
    };
    a_id!( command, expected );

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
    };
    a_id!( command, expected );

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
    };
    a_id!( command, expected );

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
    };
    a_id!( command, expected );
  }

  //

  fn test_complex()
  {
    let command = Struct1::former()
    .int_1( 13 )
    .string_1( "Abcd".to_string() )
    // .vec_1().push( "ghi" ).push( "klm" ).end()
    // .hashmap_strings_1().insert( "k1", "v1" ).insert( "k2", "v2" ).end()
    .string_optional_1( "dir1" )
    .form();
    // dbg!( &command );

    let expected = Struct1
    {
      int_1 : 13,
      string_1 : "Abcd".to_string(),
      int_optional_1 : None,
      string_optional_1 : Some( "dir1".to_string() ),
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
  test_int,
  test_string,
  test_optional_string,
  test_underscored_form,
  test_complex,
}
