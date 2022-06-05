#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]
#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]

#[ allow( unused_imports ) ]
use inspect_type as TheModule;
#[ allow( unused_imports ) ]
use test_tools::*;

#[ path = "./inc/inspect_type_test.rs" ]
mod inspect_type_test;

