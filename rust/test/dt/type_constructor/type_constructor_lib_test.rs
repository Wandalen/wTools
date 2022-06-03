#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( trace_macros ) ]

use type_constructor as TheModule;

#[ path = "./inc.rs" ]
mod inc;

// zzz : move to inc after implementing macro to check presence of a dependency
#[ cfg( feature = "use_std" ) ]
#[ test_tools::rustversion::nightly ]
#[ test ]
fn trybuild_tests()
{
  use test_tools::trybuild;
  println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
  #[ allow( unused_variables ) ]
  let t = trybuild::TestCases::new();
  #[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
  t.compile_fail( "../../../rust/test/dt/type_constructor/dynamic/make/*.rs" );
  /* xxx : rewiew */
  t.compile_fail( "../../../rust/test/dt/type_constructor/dynamic/types/*.rs" );
}

