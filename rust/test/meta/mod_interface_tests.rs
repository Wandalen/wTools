#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

#[ allow( unused_imports ) ]
use ::mod_interface as TheModule;
#[ allow( unused_imports ) ]
use ::mod_interface::mod_interface;

// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

include!( "../_conditional/local_module.rs" );
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

mod mod_interface;
