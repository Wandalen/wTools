use super::*;
use the_module::exposed::*;

// trace_macros!( true );
tests_impls!
{

  fn pass1_test()
  {
    a_id!( true, true );
  }

  //

  fn fail1_test()
  {
    // a_id!( true, false );
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
}
// trace_macros!( false );

// trace_macros!( true );
// pass1_test!();
// trace_macros!( false );

// trace_macros!( true );
tests_index!
{
  pass1_test,
  fail1_test,
  never_test,
  always_test,
}
// trace_macros!( false );
