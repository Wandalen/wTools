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

  #[ cfg( feature = "compiletime_assertions" ) ]
  #[ test ]
  fn cta_true_test()
  {

    cta_true!( all( feature = "compiletime_assertions", all() ) );
    // xxx : try ( 1 + 2 == 3 )

  }

}

//

tests_index!
{
  assertions,
  cta_true_test,
}
