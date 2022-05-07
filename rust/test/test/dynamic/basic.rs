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

#[cfg(any())]
fn never_test()
{
  println!( "never_test" );
}

//

#[cfg(all())]
fn always_test()
{
  println!( "always_test" );
}

//

test_suite!
{
  pass1,
  fail1,
  #[cfg(any())]
  never,
  #[cfg(all())]
  #[cfg(all())]
  always,
}

//

#[ allow( dead_code ) ]
fn main()
{
}
