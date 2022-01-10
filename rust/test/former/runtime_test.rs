
// use meta_tools::*;
// use std::env;

// #[test]
// fn trybuild_tests()
// {
//   println!( "current_dir : {:?}", env::current_dir().unwrap() );
//   // let t = trybuild::TestCases::new();
//   // t.pass( "rust/test/former/test/basic_runtime.rs" );
// }

// include!( "./all/runtime.rs" );

mod basic_runtime { include!( "./all/basic_runtime.rs" ); }
mod string_slice_runtime { include!( "./all/string_slice_runtime.rs" ); }
