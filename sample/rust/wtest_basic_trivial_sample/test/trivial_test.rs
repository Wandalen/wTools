use wtest_basic::*;

//

fn main()
{
  pass1_test();
  pass2_test();
}

//

fn pass1_test()
{
  assert_eq!( true, true );
}

//

fn pass2_test()
{
  assert_eq!( 1, 1 );
}

//

test_suite!
{
  pass1,
  pass2,
}
