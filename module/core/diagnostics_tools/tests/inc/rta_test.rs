#[ allow( unused_imports ) ]
use super::*;
// use test_tools::exposed::*;
#[ allow( unused_imports ) ]
use the_module::prelude::*;

// qqq : do negative testing, don't forget about optional arguments /* aaa : Dmytro : done */
#[ cfg( not( target_os = "windows" ) ) ]
tests_impls!
{
  fn a_true_pass()
  {
    a_true!( 1 == 1 );
  }

  #[ should_panic ]
  fn a_true_fail_simple()
  {
    a_true!( 1 == 2 );
  }

  #[ should_panic ]
  fn a_true_fail_with_msg()
  {
    a_true!( 1 == 2, "not equal" );
  }

  #[ should_panic ]
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

  #[ should_panic ]
  fn a_id_fail_simple()
  {
    a_id!( 1, 2 );
  }

  #[ should_panic ]
  fn a_id_fail_with_msg()
  {
    a_id!( 1, 2, "not equal" );
  }

  #[ should_panic ]
  fn a_id_fail_with_msg_template()
  {
    let v = 2;
    a_id!( 1, v, "not equal 1 == {}", v );
  }

  #[ allow( unused_macros ) ]
  fn a_id_run()
  {
    use std::path::PathBuf;
    let t = test_tools::compiletime::TestCases::new();
    let relative_path = "diagnostics_tools/tests/inc/snipet/rta_id.rs";
    let absolute_path = std::env::current_dir().unwrap();
    let current_dir_str = absolute_path.to_string_lossy();

    let trimmed_path = if let Some( index ) = current_dir_str.find( "core/" )
    {
      &current_dir_str[ 0..index + "core/".len() ]
    }
    else
    {
      relative_path
    };

    let res = trimmed_path.to_string() + relative_path;

    t.pass( res );
    // t.pass( "tests/inc/snipet/rta_id_fail.rs" );
    // zzz : make testing utility to check output and use

    // let ins1 = ( 13, 15, 16 );
    // let ins2 = ( 13, 15, 17 );
    // a_id!( ins1, ins2 );

  }

  //

  fn a_not_id_pass()
  {
    a_not_id!( "abc", "abd" );
  }

  #[ should_panic ]
  fn a_not_id_fail_simple()
  {
    a_not_id!( 1, 1 );
  }

  #[ should_panic ]
  fn a_not_id_fail_with_msg()
  {
    a_not_id!( 1, 1, "equal" );
  }

  #[ should_panic ]
  fn a_not_id_fail_with_msg_template()
  {
    let v = 1;
    a_not_id!( 1, v, "equal 1 == {}", v );
  }

  #[ allow( unused_macros ) ]
  fn a_not_id_run()
  {
    use std::path::PathBuf;
    let t = test_tools::compiletime::TestCases::new();
    let relative_path = "diagnostics_tools/tests/inc/snipet/rta_id.rs";
    let absolute_path = std::env::current_dir().unwrap();
    let current_dir_str = absolute_path.to_string_lossy();

    let trimmed_path = if let Some( index ) = current_dir_str.find( "core/" )
    {
      &current_dir_str[ 0..index + "core/".len() ]
    }
    else
    {
      relative_path
    };

    let res = trimmed_path.to_string() + relative_path;

    t.pass( res );
    // t.pass( "tests/inc/snipet/rta_not_id_fail.rs" );
    // zzz : make testing utility to check output and use

    // let ins1 = ( 13, 15, 16 );
    // let ins2 = ( 13, 15, 16 );
    // a_not_id!( ins1, ins2 );
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
  #[ should_panic ]
  fn a_dbg_true_fail_simple()
  {
    a_dbg_true!( 1 == 2 );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic ]
  fn a_dbg_true_fail_with_msg()
  {
    a_dbg_true!( 1 == 2, "not equal" );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic ]
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
  #[ should_panic ]
  fn a_dbg_id_fail_simple()
  {
    a_dbg_id!( 1, 2 );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic ]
  fn a_dbg_id_fail_with_msg()
  {
    a_dbg_id!( 1, 2, "not equal" );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic ]
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
  #[ should_panic ]
  fn a_dbg_not_id_fail_simple()
  {
    a_dbg_not_id!( 1, 1 );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic ]
  fn a_dbg_not_id_fail_with_msg()
  {
    a_dbg_not_id!( 1, 1, "equal" );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic ]
  fn a_dbg_not_id_fail_with_msg_template()
  {
    let v = 1;
    a_dbg_not_id!( 1, v, "equal 1 == {}", v );
  }
}

//
#[ cfg( target_os = "windows" ) ]
tests_impls!
{
  fn a_true_pass()
  {
    a_true!( 1 == 1 );
  }

  #[ should_panic ]
  fn a_true_fail_simple()
  {
    a_true!( 1 == 2 );
  }

  #[ should_panic ]
  fn a_true_fail_with_msg()
  {
    a_true!( 1 == 2, "not equal" );
  }

  #[ should_panic ]
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

  #[ should_panic ]
  fn a_id_fail_simple()
  {
    a_id!( 1, 2 );
  }

  #[ should_panic ]
  fn a_id_fail_with_msg()
  {
    a_id!( 1, 2, "not equal" );
  }

  #[ should_panic ]
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

  #[ should_panic ]
  fn a_not_id_fail_simple()
  {
    a_not_id!( 1, 1 );
  }

  #[ should_panic ]
  fn a_not_id_fail_with_msg()
  {
    a_not_id!( 1, 1, "equal" );
  }

  #[ should_panic ]
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
  #[ should_panic ]
  fn a_dbg_true_fail_simple()
  {
    a_dbg_true!( 1 == 2 );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic ]
  fn a_dbg_true_fail_with_msg()
  {
    a_dbg_true!( 1 == 2, "not equal" );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic ]
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
  #[ should_panic ]
  fn a_dbg_id_fail_simple()
  {
    a_dbg_id!( 1, 2 );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic ]
  fn a_dbg_id_fail_with_msg()
  {
    a_dbg_id!( 1, 2, "not equal" );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic ]
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
  #[ should_panic ]
  fn a_dbg_not_id_fail_simple()
  {
    a_dbg_not_id!( 1, 1 );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic ]
  fn a_dbg_not_id_fail_with_msg()
  {
    a_dbg_not_id!( 1, 1, "equal" );
  }

  #[ cfg( debug_assertions ) ]
  #[ should_panic ]
  fn a_dbg_not_id_fail_with_msg_template()
  {
    let v = 1;
    a_dbg_not_id!( 1, v, "equal 1 == {}", v );
  }
}


#[ cfg( target_os = "windows" ) ]
tests_index!
{
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

#[ cfg( not( target_os = "windows" ) ) ]
tests_index!
{
  a_true_pass,
  a_true_fail_simple,
  a_true_fail_with_msg,
  a_true_fail_with_msg_template,

  a_id_pass,
  a_id_fail_simple,
  a_id_fail_with_msg,
  a_id_fail_with_msg_template,
  a_id_run,

  a_not_id_pass,
  a_not_id_fail_simple,
  a_not_id_fail_with_msg,
  a_not_id_fail_with_msg_template,
  a_not_id_run,

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
