#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]
// #![ feature( concat_idents ) ]
// #![ feature( type_name_of_val ) ]
// #![ feature( inspect_type_of, inspect_to_str_type_of ) ]

#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]

use wtools as TheModule;
use test_tools::*;

#[ path = "./mod.rs" ]
mod tests;
