#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]
#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]

use inspect_type as TheModule;

#[ path = "./impls/inspect_type_test.rs" ]
mod inspect_type_test;

