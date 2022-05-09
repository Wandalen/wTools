#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

//!
//! Former - variation of builder pattern.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

pub use derive::*;
pub use former_runtime as runtime;
pub use former_meta as derive;
