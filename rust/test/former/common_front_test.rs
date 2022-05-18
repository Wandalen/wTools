use super::*;
use std::env;

#[ test ]
#[ rustversion::stable ]
// #[ cfg( not( feature = "nightly" ) ) ]
fn trybuild_tests()
{
  println!( "current_dir : {:?}", env::current_dir().unwrap() );
}

// stable have different information about error
// that's why these tests are active only for nightly
#[ test ]
// #[ cfg( feature = "nightly" ) ]
#[ rustversion::nightly ]
fn trybuild_tests()
{
  use test_tools::dependencies::trybuild;
  println!( "current_dir : {:?}", env::current_dir().unwrap() );
  let t = trybuild::TestCases::new();
  t.compile_fail( "../../../rust/test/former/all/bad_attr.rs" );
  t.compile_fail( "../../../rust/test/former/all/vector_without_parameter.rs" );
  t.compile_fail( "../../../rust/test/former/all/hashmap_without_parameter.rs" );
}

// /* zzz : use mod_at */
//
// mod basic_runtime
// {
//   #[cfg( not( feature = "in_wtools" ) )]
//   use meta_tools::*;
//
//   #[cfg( feature = "in_wtools" )]
//   use wtools::*;
//   #[cfg( not( feature = "in_wtools" ) )]
//   mod former
//   {
//     pub use former_runtime as runtime;
//   }
//
//   include!( "./all/basic_runtime_common.rs" );
// }

#[ path = "./all/basic.rs" ]
mod basic;
#[ path = "./all/conflict.rs" ]
mod conflict;
#[ path = "./all/string_slice_runtime.rs" ]
mod string_slice_runtime;
#[ path = "./all/string_slice.rs" ]
mod string_slice;

#[ path = "./all/default_user_type.rs" ]
mod default_user_type;
#[ path = "./all/default_primitive.rs" ]
mod default_primitive;
#[ path = "./all/default_container.rs" ]
mod default_container;
#[ path = "./all/perform.rs" ]
mod perform;