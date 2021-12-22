
use wtest_basic::*;

//

fn _err_basic()
{
  use werror as TheModule;

  // test.case( "basic" );
  let err = TheModule::err!( "abc" );
  assert_eq!( err.to_string(), "abc" );

  // test.case( "with args" );
  let err = TheModule::err!( "abc{}{}", "def", "ghi" );
  assert_eq!( err.to_string(), "abcdefghi" );

}

//

test_suite!
{
  err_basic,
}
