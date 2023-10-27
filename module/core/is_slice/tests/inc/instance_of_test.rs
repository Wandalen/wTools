#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

// use test_tools::exposed::*;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

use super::*;

tests_impls!
{

  //

  fn implements_basic_test()
  {

    let src = Box::new( true );
    a_id!( TheModule::implements!( src => Copy ), false );
    a_id!( TheModule::implements!( src => Clone ), true );

  }

  //

  fn instance_of_basic_test()
  {

    let src = Box::new( true );
    a_id!( TheModule::instance_of!( src => Copy ), false );
    a_id!( TheModule::instance_of!( src => Clone ), true );

  }

}

//

tests_index!
{
  implements_basic_test,
  instance_of_basic_test,
}