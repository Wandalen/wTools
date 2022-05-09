#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Basic exceptions handling mechanism.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

mod assert;
mod error;

pub use assert::*;
pub use error::*;
