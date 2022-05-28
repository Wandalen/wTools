#[ allow( unused_imports ) ]
use super::TheModule;
use test_tools::*;
#[ allow( unused_imports ) ]
use TheModule::prelude::*;

// qqq : do negative testing

tests_impls!
{

  #[ cfg( any( feature = "runtime_assertions", feature = "diagnostics_runtime_assertions" ) ) ]
  #[ test ]
  fn assertions()
  {

    a_id!( "abc", "abc" );
    // a_id!( "abc", "abd" );

    // a_not_id!( "abc", "abc" );
    a_not_id!( "abc", "abd" );

  }

  #[ cfg( any( feature = "compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) ) ]
  #[ test ]
  fn cta_true_test()
  {

    cta_true!( any( feature = "compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) );
    // xxx : try ( 1 + 2 == 3 )

  }

}

//

tests_index!
{
  assertions,
  cta_true_test,
}
