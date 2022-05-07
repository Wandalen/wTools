use wtest_basic::*;

//

fn main()
{
  _pass1();
  _pass2();
}

//

fn _pass1()
{
  assert_eq!( true, true );
}

//

fn _pass2()
{
  assert_eq!( 1, 1 );
}

//

test_suite!
{
  pass1,
  pass2,
}
