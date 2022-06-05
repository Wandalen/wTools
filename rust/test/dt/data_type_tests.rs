#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( trace_macros ) ]

use data_type as TheModule;
#[ allow( unused_imports ) ]
use test_tools::*;

#[ path = "./inc.rs" ]
mod inc;
