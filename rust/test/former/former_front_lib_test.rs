
include!( "../_conditional/local_module.rs" );

use former as TheModule;

#[ test ]
#[ rustversion::stable ]
// #[ cfg( not( feature = "nightly" ) ) ]
fn trybuild_tests()
{
  println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
}

// stable have different information about error
// that's why these tests are active only for nightly
#[ test ]
// #[ cfg( feature = "nightly" ) ]
#[ rustversion::nightly ]
fn trybuild_tests()
{
  use test_tools::dependencies::trybuild;
  println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
  let t = trybuild::TestCases::new();
  t.compile_fail( "../../../rust/test/former/all/bad_attr.rs" );
  t.compile_fail( "../../../rust/test/former/all/vector_without_parameter.rs" );
  t.compile_fail( "../../../rust/test/former/all/hashmap_without_parameter.rs" );
}

#[ allow( unused_imports ) ]
use TheModule::prelude::*;
#[ allow( unused_imports ) ]
use TheModule::*;
#[ allow( unused_imports ) ]
use meta_tools::prelude::*;

#[ path = "./common_front_test.rs" ]
mod  common_front_test;
