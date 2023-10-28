#[ allow( unused_imports ) ]
use super::*;
// use test_tools::exposed::*;
#[ allow( unused_imports ) ]
use TheModule::prelude::*;

// qqq : do negative testing /* aaa : Dmytro : done */

tests_impls!
{

  #[ cfg( any( feature = "diagnostics_compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) ) ]
  fn cta_true_pass()
  {
    // test.case( "check feature, true" );
    cta_true!( any( feature = "diagnostics_compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) );
    // zzz : try ( 1 + 2 == 3 )
  }

  #[ cfg( feature = "diagnostics_compiletime_assertions" ) ]
  #[ test_tools::rustversion::nightly ]
  fn cta_trybuild_tests()
  {
    use test_tools::dependency::trybuild;
    let t = trybuild::TestCases::new();
    t.compile_fail( "tests/inc/snipet/cta_true_fail.rs" );
  }

}

//

tests_index!
{
  cta_true_pass,
  cta_trybuild_tests,
}
