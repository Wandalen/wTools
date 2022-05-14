#[cfg( feature = "in_wtools" )]
use wtools::meta as TheModule;
#[cfg( not( feature = "in_wtools" ) )]
use meta_tools as TheModule;
use TheModule::prelude::*;

// trace_macros!( true );
impls!
{

  #[ test ]
  fn pass1_test()
  {
    assert_eq!( true, true );
  }

  //

  #[ test ]
  fn fail1_test()
  {
    // assert_eq!( true, false );
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
index!
{
  pass1_test,
  fail1_test,
  never_test,
  always_test,
}
// trace_macros!( false );
