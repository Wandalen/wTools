#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

use ::impls_index as TheModule;
#[ allow( unused_imports ) ]
use test_tools::*;

#[ path = "./impls_index/mod.rs" ]
mod impls_index;
