#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Collection of general purpose tools for type checking.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Collection of general purpose tools for type checking.
pub mod typing;

/// Dependencies.
pub mod dependencies
{
  pub use ::inspect_type;
  pub use ::is_slice;
  pub use ::implements;
}

#[ doc( inline ) ]
pub use typing::*;
