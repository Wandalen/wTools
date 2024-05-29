// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

use type_constructor as the_module;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

#[ path = "./inc.rs" ]
mod inc;

// zzz : move to inc after implementing macro to check presence of a dependency
#[ cfg( not( feature = "no_std" ) ) ]
#[ test_tools::nightly ]
#[ test ]
fn trybuild_tests()
{
  // use test_tools::trybuild;
  println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
  #[ allow( unused_variables ) ]
  // let t = trybuild::TestCases::new();
  let t = test_tools::compiletime::TestCases::new();

  #[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
  t.compile_fail( "tests/test/dt/type_constructor/dynamic/make/*.rs" );

  #[ cfg( all( any( not( feature = "no_std" ), feature = "use_alloc" ), feature = "many" ) ) ]
  t.compile_fail( "tests/test/dt/type_constructor/dynamic/types_many_yes/*.rs" );

  #[ cfg( any( not( any( not( feature = "no_std" ), feature = "use_alloc" ) ), not( feature = "many" ) ) ) ]
  t.compile_fail( "tests/test/dt/type_constructor/dynamic/types_many_no/*.rs" );

  t.compile_fail( "tests/test/dt/type_constructor/single/single_nested_type_test.rs" );
  t.compile_fail( "tests/test/dt/type_constructor/single/single_with_two_args_test.rs" );
  t.compile_fail( "tests/test/dt/type_constructor/single/single_not_completed_type_test.rs" );
  t.compile_fail( "tests/test/dt/type_constructor/single/single_redefinition_test.rs" );

  t.compile_fail( "tests/test/dt/type_constructor/pair/homo_pair_double_difinition_test.rs" );
  t.compile_fail( "tests/test/dt/type_constructor/pair/homo_pair_mismatched_types_test.rs" );

  t.compile_fail( "tests/test/dt/type_constructor/pair/pair_without_args_test.rs" );
  t.compile_fail( "tests/test/dt/type_constructor/pair/pair_three_elements_test.rs" );

  t.compile_fail( "tests/test/dt/type_constructor/many/many_without_args_test.rs" );
  t.compile_fail( "tests/test/dt/type_constructor/many/many_with_two_args_test.rs" );
  t.compile_fail( "tests/test/dt/type_constructor/many/many_from_tuple_test.rs" );
}
