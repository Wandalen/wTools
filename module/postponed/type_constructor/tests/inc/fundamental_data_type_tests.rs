// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

// #![ feature( trace_macros ) ]

use fundamental_data_type as the_module;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

#[ path = "./inc.rs" ]
mod inc;
