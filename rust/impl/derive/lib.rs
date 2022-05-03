#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( trait_alias ) ]
// #![ feature( generic_associated_types ) ]
// #![ feature( type_name_of_val ) ]
// #![ feature( generic_associated_types ) ]

//!
//! Collection of general purpose derives.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Internal namespace.
pub mod internal
{
}

pub use derive_more::*;
pub use parse_display::*;
pub use parse_display::Display;

/// Own namespace of the module.
pub mod own
{
  // use super::internal as i;
}

pub use own::*;

/// Exposed namespace of the module.
pub mod exposed
{
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
}
