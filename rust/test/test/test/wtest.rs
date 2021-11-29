
use wtest::test_suite;

//

fn _pass1()
{
  assert_eq!( true, true );
}

//

fn _fail1()
{
  // assert_eq!( true, false );
}

//

test_suite!
{
  pass1,
  fail1,
}
