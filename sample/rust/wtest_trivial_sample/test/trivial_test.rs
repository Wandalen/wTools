use wtest::*;

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
