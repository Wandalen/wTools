// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

// #![ cfg_attr( feature = "type_name_of_val", feature( type_name_of_val ) ) ]
// #![ feature( trace_macros ) ]

#[ allow( unused_imports ) ]
use diagnostics_tools as the_module;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;
// #[ path="../../../../module/step/meta/src/module/terminal.rs" ]
// mod terminal;

mod inc;
