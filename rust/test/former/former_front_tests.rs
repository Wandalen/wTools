#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

include!( "../_conditional/local_module.rs" );

// use former as TheModule;
// // use test_tools::*;

// #[ test ]
// #[ rustversion::stable ]
// fn trybuild_tests()
// {
//   println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
// }
//
// // stable have different information about error
// // that's why these tests are active only for nightly
// #[ test ]
// #[ rustversion::nightly ]
// fn trybuild_tests()
// {
//   use test_tools::dependencies::trybuild;
//   println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
//   let t = trybuild::TestCases::new();
//   t.compile_fail( "../../../rust/test/former/all/former_bad_attr.rs" );
//   t.compile_fail( "../../../rust/test/former/all/former_vector_without_parameter.rs" );
//   t.compile_fail( "../../../rust/test/former/all/former_hashmap_without_parameter.rs" );
// }

// #[ allow( unused_imports ) ]
// use TheModule::prelude::*;
// #[ allow( unused_imports ) ]
// use TheModule::*;
// #[ allow( unused_imports ) ]
// use meta_tools::prelude::*;

#[ path = "./common_front_test.rs" ]
mod common_front_test;

// xxx : qqq : bad!
