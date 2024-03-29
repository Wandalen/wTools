#[ allow( unused_imports ) ]
use super::*;

//

tests_impls!
{

  //

  fn internals()
  {

    let former = Struct1::former();
    a_id!( former.storage.int_1, None );
    a_id!( former.storage.string_1, None );
    a_id!( former.storage.int_optional_1, None );
    a_id!( former.storage.string_optional_1, None );
    a_id!( former.context, None );
    a_id!( print!( "{:?}", former.on_end ), print!( "{:?}", Some( the_module::ReturnPreformed ) ) );
    let former2 = Struct1Former::new();
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

  //

  fn begin()
  {

    let mut storage = Struct1FormerStorage::default();
    storage.int_1 = Some( 13 );
    // Struct1Former::< (), the_module::ReturnFormed >::begin( storage, None, the_module::ReturnFormed )
    // xxx

  }

  //

  fn preform()
  {

    // formation should have method preform
    let got = Struct1::former().preform();
    let exp = Struct1::former().form();
    a_id!( got, exp );

    // storage should have method preform
    let got = the_module::StoragePerform::preform( Struct1::former().storage );
    let exp = Struct1::former().form();
    a_id!( got, exp );

    // storage should have method preform
    use the_module::StoragePerform;
    let got = Struct1::former().storage.preform();
    let exp = Struct1::former().form();
    a_id!( got, exp );

  }

  //

  fn descriptor()
  {

    // descriptor exists and has Formed
    let got = < Struct1FormerDefinition as the_module::FormerDefinitionTypes >::Formed::former().form();
    let exp = Struct1::former().form();
    a_id!( got, exp );

    // descriptor exists and has Storage
    use former::StoragePerform;
    let got = < Struct1FormerDefinition as the_module::FormerDefinitionTypes >::Storage::preform( Struct1::former().storage );
    let exp = Struct1::former().form();
    a_id!( got, exp );

  }

  //

  fn storage()
  {

    // descriptor exists and has Storage
    let got = < Struct1FormerStorage as the_module::StoragePerform >::preform( Struct1::former().storage );
    let exp = Struct1::former().form();
    a_id!( got, exp );

    // default is implemented for Storage
    let got = Struct1FormerStorage::default().preform();
    let exp = Struct1::former().storage.preform();
    a_id!( got, exp );

    // descriptor exists and has Storage
    use former::StoragePerform;
    let got = Struct1::former().storage.preform();
    let exp = Struct1::former().form();
    a_id!( got, exp );

    // storage exists
    let got = < Struct1FormerStorage as the_module::Storage >::Formed::former().form();
    let exp = Struct1::former().form();
    a_id!( got, exp );

  }

  //

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
  begin,
  preform,
  descriptor,
  storage,
  test_int,
  test_string,
  test_optional_string,
  test_underscored_form,
  test_complex,
}
