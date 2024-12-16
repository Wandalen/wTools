//
// use super::*;
//
// #[ path = "../dynamic/basic.rs" ]
// mod basic;
//
// //
//
// the_module::tests_index!
// {
//   trybuild_test,
// }

#[ allow( unused_imports ) ]
use super::*;
use ::test_tools as the_module;

#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
the_module::tests_impls!
{

  //

  fn pass1_test()
  {
    the_module::a_id!( true, true );
  }

  //

  fn fail1_test()
  {
    // the_module::a_id!( true, false );
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

//

#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
the_module::tests_index!
{
  pass1_test,
  fail1_test,
  never_test,
  always_test,
}
