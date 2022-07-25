#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

include!( "../_conditional/local_module.rs" );

mod common_front_test;

//

// stable have different information about error
// that's why these tests are active only for nightly
#[ test_tools::rustversion::nightly ]
#[ test ]
fn trybuild_tests()
{
  use test_tools::dependency::trybuild;
  println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
  let t = trybuild::TestCases::new();
  t.compile_fail( "../../../rust/test/former/all/wtools_bad_attr.rs" );
  t.compile_fail( "../../../rust/test/former/all/wtools_vector_without_parameter.rs" );
  t.compile_fail( "../../../rust/test/former/all/wtools_hashmap_without_parameter.rs" );
}
