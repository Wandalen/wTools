#[ allow( unused_imports ) ]
use wtest_basic::*;

//

fn pass1_test()
{
  assert_eq!( true, true );
}

//

fn fail1_test()
{
  // assert_eq!( true, false );
}

//

test_suite!
{
  pass1,
  fail1,
}

//

#[ allow( dead_code ) ]
fn main()
{
}
