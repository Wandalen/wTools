#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Fundamental data types and type constructors, like Single, Pair, Many.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

#[ path = "./inc.rs" ]
mod inc;
pub use inc::*;

// xxx : implement module include_md
