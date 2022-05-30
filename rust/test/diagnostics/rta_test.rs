
#[ allow( unused_imports ) ]
use super::TheModule;
use test_tools::*;
#[ allow( unused_imports ) ]
use TheModule::prelude::*;

// qqq : do negative testing, don't forget about optional arguments

tests_impls!
{

  #[ test ]
  fn a_true_test()
  {
    a_true!( 1 == 1 );
    // a_true!( 1 == 2 );
  }

  #[ test ]
  fn a_id_test()
  {
    a_id!( "abc", "abc" );
    // a_id!( "abc", "abd" );
  }

  #[ test ]
  fn a_not_id_test()
  {
    // a_not_id!( "abc", "abc" );
    a_not_id!( "abc", "abd" );
  }

  #[ test ]
  fn a_dbg_true_test()
  {
    a_dbg_true!( 1 == 1 );
    // a_dbg_true!( 1 == 2 );

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

  #[ test ]
  fn a_dbg_id_test()
  {
    a_dbg_id!( "abc", "abc" );
    // a_dbg_id!( "abc", "abd" );

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

  #[ test ]
  fn a_dbg_not_id_test()
  {
//     // a_dbg_not_id!( "abc", "abc" );
//     a_dbg_not_id!( "abc", "abd" );
//
//     let mut x = 0;
//     let mut f1 = ||-> i32
//     {
//       x += 1;
//       x
//     };
//     a_dbg_not_id!( f1(), 1 );
//
//     #[ cfg( debug_assertions ) ]
//     assert_eq!( x, 1 );
//     #[ cfg( not( debug_assertions ) ) ]
//     assert_eq!( x, 0 );

  }

}

//

tests_index!
{

  a_true_test,
  a_id_test,
  a_not_id_test,

  a_dbg_true_test,
  a_dbg_id_test,
  a_dbg_not_id_test,

}
