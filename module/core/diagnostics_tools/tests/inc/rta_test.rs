#[ allow( unused_imports ) ]
use super::*;
// use test_tools::exposed::*;
#[ allow( unused_imports ) ]
use the_module::prelude::*;
// xxx: temporarily disabled due to macro resolution issues
/*
use test_tools::impls_index::tests_impls;
use test_tools::impls_index::tests_index;
*/
use diagnostics_tools::a_true;
use diagnostics_tools::a_id;
use diagnostics_tools::a_not_id;
use diagnostics_tools::a_dbg_true;
use diagnostics_tools::a_dbg_id;
use diagnostics_tools::a_dbg_not_id;

// qqq : do negative testing, don't forget about optional arguments /* aaa : Dmytro : done */
// Test implementations (available on all platforms)
// xxx: temporarily disabled due to macro resolution issues
/*
tests_impls! {
  fn a_true_pass()
  {
    a_true!( 1 == 1 );
  }

  #[ should_panic( expected = "assertion failed" ) ]
  fn a_true_fail_simple()
  {
    a_true!( 1 == 2 );
  }

  #[ should_panic( expected = "not equal" ) ]
  fn a_true_fail_with_msg()
  {
    a_true!( 1 == 2, "not equal" );
  }

  #[ should_panic( expected = "not equal" ) ]
  fn a_true_fail_with_msg_template()
  {
    let v = 2;
    a_true!( 1 == v, "not equal 1 == {}", v );
  }

  //

  fn a_id_pass()
  {
    a_id!( "abc", "abc" );
  }

  #[ should_panic( expected = "assertion failed" ) ]
  fn a_id_fail_simple()
  {
    a_id!( 1, 2 );
  }

  #[ should_panic( expected = "not equal" ) ]
  fn a_id_fail_with_msg()
  {
    a_id!( 1, 2, "not equal" );
  }

  #[ should_panic( expected = "not equal" ) ]
  fn a_id_fail_with_msg_template()
  {
    let v = 2;
    a_id!( 1, v, "not equal 1 == {}", v );
  }



  //

  fn a_not_id_pass()
  {
    a_not_id!( "abc", "abd" );
  }

  #[ should_panic( expected = "assertion failed" ) ]
  fn a_not_id_fail_simple()
  {
    a_not_id!( 1, 1 );
  }

  #[ should_panic( expected = "equal" ) ]
  fn a_not_id_fail_with_msg()
  {
    a_not_id!( 1, 1, "equal" );
  }

  #[ should_panic( expected = "equal" ) ]
  fn a_not_id_fail_with_msg_template()
  {
    let v = 1;




    a_not_id!( 1, v, "equal 1 == {}", v );
  }

  //

  fn a_dbg_true_pass()
  {
    a_dbg_true!( 1 == 1 );

    let mut x = 0;
    let mut f1 = ||-> i32
    {
      x += 1;
      x
    };
    a_dbg_true!( f1() == 1 );

    #[ cfg( debug_assertions ) ]
    assert_eq!( x, 1 );
    #[ cfg( not( debug_assertions ) ) ]
    assert_eq!( x, 0 );

  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic( expected = "assertion failed" ) ]
  fn a_dbg_true_fail_simple()
  {
    a_dbg_true!( 1 == 2 );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic( expected = "not equal" ) ]
  fn a_dbg_true_fail_with_msg()
  {
    a_dbg_true!( 1 == 2, "not equal" );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic( expected = "not equal" ) ]
  fn a_dbg_true_fail_with_msg_template()
  {
    let v = 2;
    a_dbg_true!( 1 == v, "not equal 1 == {}", v );
  }

  //

  fn a_dbg_id_pass()
  {
    a_dbg_id!( "abc", "abc" );

    let mut x = 0;
    let mut f1 = ||-> i32
    {
      x += 1;
      x
    };
    a_dbg_id!( f1(), 1 );

    #[ cfg( debug_assertions ) ]
    assert_eq!( x, 1 );
    #[ cfg( not( debug_assertions ) ) ]
    assert_eq!( x, 0 );

  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic( expected = "assertion failed" ) ]
  fn a_dbg_id_fail_simple()
  {
    a_dbg_id!( 1, 2 );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic( expected = "not equal" ) ]
  fn a_dbg_id_fail_with_msg()
  {
    a_dbg_id!( 1, 2, "not equal" );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic( expected = "not equal" ) ]
  fn a_dbg_id_fail_with_msg_template()
  {
    let v = 2;
    a_dbg_id!( 1, v, "not equal 1 == {}", v );
  }

  //

  fn a_dbg_not_id_pass()
  {
    a_dbg_not_id!( "abc", "bdc" );

    let mut x = 0;
    let mut f1 = ||-> i32
    {
      x += 1;
      x
    };
    a_dbg_not_id!( f1(), 0 );

    #[ cfg( debug_assertions ) ]
    assert_eq!( x, 1 );
    #[ cfg( not( debug_assertions ) ) ]
    assert_eq!( x, 0 );

  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic( expected = "assertion failed" ) ]
  fn a_dbg_not_id_fail_simple()
  {
    a_dbg_not_id!( 1, 1 );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic( expected = "equal" ) ]
  fn a_dbg_not_id_fail_with_msg()
  {
    a_dbg_not_id!( 1, 1, "equal" );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic( expected = "equal" ) ]
  fn a_dbg_not_id_fail_with_msg_template()
  {
    let v = 1;
    a_dbg_not_id!( 1, v, "equal 1 == {}", v );
  }
}

// Windows-specific test index (cfg directive disabled as requested)
tests_index! {
  a_true_pass,
  a_true_fail_simple,
  a_true_fail_with_msg,
  a_true_fail_with_msg_template,

  a_id_pass,
  a_id_fail_simple,
  a_id_fail_with_msg,
  a_id_fail_with_msg_template,

  a_not_id_pass,
  a_not_id_fail_simple,
  a_not_id_fail_with_msg,
  a_not_id_fail_with_msg_template,

  a_dbg_true_pass,
  a_dbg_true_fail_simple,
  a_dbg_true_fail_with_msg,
  a_dbg_true_fail_with_msg_template,

  a_dbg_id_pass,
  a_dbg_id_fail_simple,
  a_dbg_id_fail_with_msg,
  a_dbg_id_fail_with_msg_template,

  a_dbg_not_id_pass,
  a_dbg_not_id_fail_simple,
  a_dbg_not_id_fail_with_msg,
  a_dbg_not_id_fail_with_msg_template,
}
*/

