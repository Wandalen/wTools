#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( trait_alias ) ]
// #![ feature( type_name_of_val ) ]

//!
//! Collection of general purpose derives.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Internal namespace.
mod internal
{
}

/// Own namespace of the module.
pub mod protected
{
  pub use super::exposed::*;
  // use super::internal as i;
}

pub use protected::*;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use ::derive_more::*;
  pub use ::parse_display::*;
  pub use ::parse_display::Display;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
}
