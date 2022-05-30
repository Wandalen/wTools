#[ allow( unused_imports ) ]
use super::*;
// #[ allow( unused_imports ) ]
// use test_tools::*;

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

//

#[ test_tools::rustversion::stable ]
#[ test ]
fn trybuild_tests()
{
  println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
}

// stable have different information about error
// that's why these tests are active only for nightly
#[ cfg( any( feature = "meta_former", feature = "former" ) ) ]
#[ test_tools::rustversion::nightly ]
#[ test ]
fn trybuild_tests()
{
  use test_tools::dependencies::trybuild;
  println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
  let t = trybuild::TestCases::new();
  t.compile_fail( "../../../rust/test/former/all/wtools_bad_attr.rs" );
  t.compile_fail( "../../../rust/test/former/all/wtools_vector_without_parameter.rs" );
  t.compile_fail( "../../../rust/test/former/all/wtools_hashmap_without_parameter.rs" );
}
