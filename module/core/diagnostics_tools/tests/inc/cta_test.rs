#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use the_module::prelude::*;

tests_impls!
{

  #[ cfg( any( feature = "diagnostics_compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) ) ]
  fn cta_true_pass()
  {
    // test.case( "check feature, true" );
    cta_true!( any( feature = "diagnostics_compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) );
    // zzz : try ( 1 + 2 == 3 )
  }

}

// only_for_terminal_module!
// {

//   #[ cfg( feature = "diagnostics_compiletime_assertions" ) ]
//   #[ test_tools::nightly ]
//   #[ test ]
//   fn cta_trybuild_tests()
//   {
//     let t = test_tools::compiletime::TestCases::new();
//     t.compile_fail( "tests/inc/snipet/cta_true_fail.rs" );
//     // a_id!( 1, 2 );
//   }

// }

//

tests_index!
{
  cta_true_pass,
  // cta_trybuild_tests,
}
