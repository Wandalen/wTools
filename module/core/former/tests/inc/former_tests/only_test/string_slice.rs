#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

//

tests_impls!
{


  //

  fn api()
  {

    // form
    let command = Struct1::former().form();
    a_id!( command.string_slice_1, "" );

    // end
    let command = Struct1::former().end();
    a_id!( command.string_slice_1, "" );

    // perform
    let command = Struct1::former().perform();
    a_id!( command.string_slice_1, "" );

    // formation should have method preform
    let got = Struct1::former().preform();
    let exp = Struct1::former().form();
    a_id!( got, exp );

    // default explicit params with wrapper and closure
    let got = Struct1Former
    ::< Struct1FormerDefinition< (), Struct1, _ > >
    ::new( | storage, _context | { former::StoragePreform::preform( storage ) } )
    .string_slice_1( "abc" )
    .form();
    let exp = Struct1::former().string_slice_1( "abc" ).form();
    a_id!( got, exp );

    // closure with helper
    let got : Struct1 = Struct1Former
    ::< Struct1FormerDefinition< (), Struct1, _ > >
    ::begin( None, None, | storage, _context | { former::StoragePreform::preform( storage ) } )
    .string_slice_1( "abc" )
    .form();
    let exp = Struct1::former().string_slice_1( "abc" ).form();
    a_id!( got, exp );

  }

  //

  fn test_complex()
  {
    // test.case( "default" );

    let command = Struct1::former().form();
    let expected = Struct1
    {
      string_slice_1 : "",
    };
    a_id!( command, expected );

    // test.case( "from slice" );

    let command = Struct1::former()
    .string_slice_1( "abc" )
    .form();
    let expected = Struct1
    {
      string_slice_1 : "abc",
    };
    a_id!( command, expected );

//     // test.case( "from string" );
//
//     let command = Struct1::former()
//     .string_slice_1( "abc".to_string() )
//     .form();
//     let expected = Struct1
//     {
//       string_slice_1 : "abc",
//     };
//     a_id!( command, expected );

  }

  //

}

//

tests_index!
{
  api,
  test_complex,
}
