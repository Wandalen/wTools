#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( type_name_of_val ) ]

//!
//! Tools for writing procedural macroses.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Helpers.
pub mod helper;
/// Trait name.
pub mod name;
/// Split with name.
pub mod split_with_name;

///
/// Dependencies of the module.
///

pub mod dependencies
{
  pub use syn;
  pub use proc_macro2;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;

  pub use super::helper::exposed::*;
  pub use super::name::exposed::*;
  pub use super::split_with_name::exposed::*;

}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::helper::prelude::*;
  pub use super::name::prelude::*;
  pub use super::split_with_name::prelude::*;
}
