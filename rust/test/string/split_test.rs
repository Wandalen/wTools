
use wtest_basic::*;
use wstring_tools as TheModule;

//

fn _basic()
{

  // test.case( "delimeter : "b" );
  let src = "abc def";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( "bc" )
  .form();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "bc", " def" ] );

}

//

fn _basic()
{

  // test.case( "delimeter : "b" );
  let src = "abc def";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( "bc" )
  .form();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "bc", " def" ] );

}

//

test_suite!
{
  basic,
}
