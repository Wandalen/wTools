
#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

tests_impls!
{

  fn trybuild_tests()
  {
    use test_tools::dependencies::trybuild;
    println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
    let t = trybuild::TestCases::new();

    t.pass( "../../../rust/test/meta/mod_interface/front/layer/trybuild.rs" );

    // t.compile_fail( "../../../rust/test/former/all/wtools_vector_without_parameter.rs" );
  }

}

tests_index!
{
  trybuild_tests,
}
