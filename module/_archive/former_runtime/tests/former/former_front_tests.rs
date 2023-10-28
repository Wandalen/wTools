#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

include!( "../_conditional/terminal_module.rs" );

#[ path = "./common_front_test.rs" ]
mod common_front_test;

//

// stable have different information about error
// that's why these tests are active only for nightly
#[ test_tools::nightly ]
#[ test ]
fn trybuild_tests()
{
  use test_tools::dependency::trybuild;
  println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
  let t = trybuild::TestCases::new();
  t.compile_fail( "tests/test/former/all/wtools_bad_attr.rs" );
  t.pass( "tests/test/former/all/wtools_vector_without_parameter.rs" );
  t.pass( "tests/test/former/all/wtools_hashmap_without_parameter.rs" );
}