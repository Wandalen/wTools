#[ allow( unused_imports ) ]
use super::TheModule;
use test_tools::*;
#[ allow( unused_imports ) ]
use TheModule::prelude::*;

// qqq : do negative testing

tests_impls!
{

  #[ cfg( feature = "assertions" ) ]
  #[ test ]
  fn assertions()
  {

    a_id!( "abc", "abc" );
    // a_id!( "abc", "abd" );

    // a_not_id!( "abc", "abc" );
    a_not_id!( "abc", "abd" );

  }

  #[ cfg( feature = "ct" ) ]
  #[ test ]
  fn ct_true_test()
  {

    ct_true!( all( feature = "ct", all() ) );

  }

  #[ cfg( feature = "ct" ) ]
  #[ test ]
  fn ct_type_same_size_test()
  {

    struct Int( i16 );
    let got = ct_type_same_size!( Int, i16 );
    assert!( got );
    // ct_type_same_size!( Int, i32 );

  }

  #[ cfg( feature = "ct" ) ]
  #[ test ]
  fn ct_ptr_same_size_test()
  {

    struct Int( i16 );
    let ins1 = Int( 31 );
    let ins2 = 13_i16;
    let got = ct_ptr_same_size!( &ins1, &ins2 );
    assert!( got );
    let got = ct_ptr_same_size!( &ins1, &ins2 );
    assert!( got );
    let got = ct_ptr_same_size!( &ins1, &31_i16 );
    assert!( got );
    // ct_ptr_same_size!( &ins1, &13_i32 );

  }

  #[ cfg( feature = "ct" ) ]
  #[ test ]
  fn ct_mem_same_size_test()
  {

    struct Int( i16 );
    let ins1 = Int( 31 );
    let ins2 = 13_i16;
    let got = ct_mem_same_size!( ins1, ins2 );
    assert!( got );
    let got = ct_mem_same_size!( ins1, ins2 );
    assert!( got );
    let got = ct_mem_same_size!( ins1, 31_i16 );
    assert!( got );
    // ct_mem_same_size!( ins1, 13_i32 );

  }

}

//

tests_index!
{
  assertions,
  ct_true_test,
  ct_type_same_size_test,
  ct_ptr_same_size_test,
  ct_mem_same_size_test,
}
