#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

include!( "../../../../module/step/meta/src/module/terminal.rs" );

#[ allow( unused_imports ) ]
use test_tools::exposed::*;
#[ allow( unused_imports ) ]
use former as TheModule;
// #[ allow( unused_imports ) ]
// use meta_tools::prelude::*;

// mod former
// {
//   pub mod runtime
//   {
//     pub use former_runtime::*;
//   }
// }

mod inc;
