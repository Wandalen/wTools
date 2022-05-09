#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Mechanism to define map of options for a fuction and its defaults laconically.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

// pub use woptions_runtime as runtime;
// pub use woptions_meta as meta;
pub use woptions as options;
pub use meta::Options;
// pub use meta::options;

pub use former::derive::Former;
