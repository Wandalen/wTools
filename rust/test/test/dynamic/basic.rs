#[ allow( unused_imports ) ]
use wtest_basic::*;

//

fn pass1()
{
  a_id!( true, true );
}

//

fn fail1()
{
  // a_id!( true, false );
}

//

// #[cfg(any())]
fn never()
{
  println!( "never_test" );
}

//

#[cfg(all())]
fn always()
{
  println!( "always_test" );
}

//

test_suite!
{
  pass1,
  fail1,
  // #[cfg(any())]
  never,
  // #[cfg(all())]
  // #[cfg(all())]
  always,
}

//

#[ allow( dead_code ) ]
fn main()
{
}
