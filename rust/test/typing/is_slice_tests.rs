#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

use is_slice as TheModule;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

#[ path = "./inc/is_slice_test.rs" ]
mod is_slice_test;
