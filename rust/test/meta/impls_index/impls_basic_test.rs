use super::*;
#[ allow( unused_imports ) ]
use TheModule::prelude::*;

// trace_macros!( true );
tests_impls!
{

  #[ test ]
  fn pass1_test()
  {
    a_id!( true, true );
  }

  //

  #[ test ]
  fn fail1_test()
  {
    // a_id!( true, false );
  }

  //

  #[cfg(any())]
  #[ test ]
  fn never_test()
  {
    println!( "never_test" );
  }

  //

  #[cfg(all())]
  #[ test ]
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
