// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

use ::impls_index as TheModule;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

#[ path = "./impls_index/mod.rs" ]
mod impls_index;
