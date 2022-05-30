use wtest::*;

//

fn main()
{
  pass1();
  pass2();
}

//

fn pass1()
{
  assert_eq!( true, true );
}

//

fn pass2()
{
  assert_eq!( 1, 1 );
}

//

test_suite!
{
  pass1,
  pass2,
}
