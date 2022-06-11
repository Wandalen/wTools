
// use wtools::former as TheModule;
// use wtools::former;

#[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
use super::TheModule::former as TheModule;
#[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
use TheModule as former;
use wtools::meta::*;

// #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
// #[ allow( unused_imports ) ]
// use TheModule::prelude::*;
// #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
// #[ allow( unused_imports ) ]
// use TheModule::*;
// #[ cfg( all( feature = "meta_former", feature = "meta" ) ) ]
// #[ allow( unused_imports ) ]
// use wtools::meta::prelude::*;

// #[ test_tools::rustversion::stable ]
// #[ test ]
// // #[ cfg( not( feature = "nightly" ) ) ]
// fn trybuild_tests()
// {
//   println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
// }
//
// // stable have different information about error
// // that's why these tests are active only for nightly
// #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
// #[ test_tools::rustversion::nightly ]
// #[ test ]
// fn trybuild_tests()
// {
//   use test_tools::dependencies::trybuild;
//   println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
//   let t = trybuild::TestCases::new();
//   t.compile_fail( "../../../rust/test/former/all/wtools_bad_attr.rs" );
//   t.compile_fail( "../../../rust/test/former/all/wtools_vector_without_parameter.rs" );
//   t.compile_fail( "../../../rust/test/former/all/wtools_hashmap_without_parameter.rs" );
// }

#[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
#[ path = "./all/basic_runtime_common.rs" ]
mod basic_runtime_common;

#[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
#[ path = "./common_front_test.rs" ]
mod common_front_test;

// xxx : qqq : bad!
