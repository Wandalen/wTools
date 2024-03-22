#![ allow( unused_imports ) ]
use super::*;

//

tests_impls!
{
  fn debug_assert_id_pass()
  {
    // test.case( "identical" );
    the_module::debug_assert_id!( 1, 1 );
  }

  //

  #[ cfg( debug_assertions ) ]
  #[ should_panic ]
  fn debug_assert_id_fail()
  {
    // test.case( "not identical" );
    the_module::debug_assert_id!( 1, 2 );
  }

  //

  fn debug_assert_identical_pass()
  {
    // test.case( "identical" );
    the_module::debug_assert_identical!( 1, 1 );
  }

  //

  #[ cfg( debug_assertions ) ]
  #[ should_panic ]
  fn debug_assert_identical_fail()
  {
    // test.case( "not identical" );
    the_module::debug_assert_identical!( 1, 2 );
  }

  //

  fn debug_assert_ni_pass()
  {
    // test.case( "not identical" );
    the_module::debug_assert_ni!( 1, 2 );
  }

  //

  #[ cfg( debug_assertions ) ]
  #[ should_panic ]
  fn debug_assert_ni_fail()
  {
    // test.case( "identical" );
    the_module::debug_assert_ni!( 1, 1 );
  }

  //

  fn debug_assert_not_identical_pass()
  {
    // test.case( "not identical" );
    the_module::debug_assert_not_identical!( 1, 2 );
  }

  //

  #[ cfg( debug_assertions ) ]
  #[ should_panic ]
  fn debug_assert_not_identical_fail()
  {
    // test.case( "identical" );
    the_module::debug_assert_not_identical!( 1, 1 );
  }
}

//

tests_index!
{
  debug_assert_id_pass,
  debug_assert_id_fail,
  debug_assert_identical_pass,
  debug_assert_identical_fail,

  debug_assert_ni_pass,
  debug_assert_ni_fail,
  debug_assert_not_identical_pass,
  debug_assert_not_identical_fail,
}
