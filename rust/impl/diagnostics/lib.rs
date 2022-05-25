#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Diagnostics tools.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Collection of general purpose tools for type checking.
pub mod diagnostics;

/// Dependencies.
pub mod dependencies
{
  #[ cfg( feature = "a_pretty" ) ]
  pub use ::pretty_assertions;
}

#[ doc( inline ) ]
pub use diagnostics::*;
