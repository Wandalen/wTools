use super::*;

#[ allow( unused_imports ) ]
use test_tools::meta::*;

mod a_primitives_manual_test;
mod a_containers_without_runtime_manual_test;
mod a_containers_without_runtime_test;
mod a_containers_with_runtime_manual_test;
mod a_containers_with_runtime_test;

mod default_container;
mod default_primitive;
mod former_hashmap_without_parameter;
mod former_vector_without_parameter;

mod string_slice_manual_test;
mod string_slice_test;

mod default_user_type;
mod user_type_no_default;
mod user_type_no_debug;

mod alias_test;
mod conflict;
mod unsigned_primitive_types;
mod perform;

mod parametrized_struct_manual;

mod subformer_basic_manual;
// mod subformer_basic;
mod subformer_wrap_hashmap_manual;

//

only_for_terminal_module!
{

  // stable have different information about error
  // that's why these tests are active only for nightly
  #[ test_tools::nightly ]
  #[ test ]
  fn trybuild_tests()
  {

    println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
    let t = test_tools::compiletime::TestCases::new();

    t.compile_fail( "tests/inc/compiletime/former_bad_attr.rs" );
    t.pass( "tests/inc/compiletime/former_hashmap_without_parameter.rs" );
    t.pass( "tests/inc/compiletime/former_vector_without_parameter.rs" );

  }

}
