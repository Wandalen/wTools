
use super::
{
  the_module,
  only_for_terminal_module,
  a_id,
};

mod func_test;
mod impls_basic_test;
mod impls1_test;
mod impls2_test;
mod impls3_test;

mod index_test;
mod tests_index_test;

only_for_terminal_module!
{

  // stable have different information about error
  // that's why these tests are active only for nightly
  #[ test_tools::nightly ]
  #[ cfg( feature = "enabled" ) ]
  #[ test ]
  fn former_trybuild()
  {

    println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
    let t = test_tools::compiletime::TestCases::new();
    // xxx : enable and use process::run

    // t.compile_fail( "tests/inc/compiletime/former_bad_attr.rs" );
    // t.pass( "tests/inc/compiletime/former_hashmap_without_parameter.rs" );
    // t.pass( "tests/inc/compiletime/former_vector_without_parameter.rs" );

    //t.compile_fail( "tests/inc/compiletime/components_component_from_debug.rs" );

  }

}
