#[ allow( unused_imports ) ]
use super::TheModule;
use test_tools::*;
#[ allow( unused_imports ) ]
use TheModule::prelude::*;

// qqq : do negative testing

tests_impls!
{

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
  cta_true_test,
}
