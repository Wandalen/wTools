#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

use ::mod_interface as TheModule;

// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

include!( "../_conditional/local_module.rs" );
#[ allow( unused_imports ) ]
use test_tools::*;

mod mod_interface;
