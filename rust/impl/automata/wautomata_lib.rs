#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

//!
//! Implementation of automata.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Other interfaces.
pub mod interface;
/// Canonical representation.
pub mod canonical;
/// Matrix representation.
pub mod matrix;

/// Namespace with dependencies.
pub mod dependency
{
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::interface::exposed::*;
  pub use super::canonical::exposed::*;
  pub use super::matrix::exposed::*;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::interface::prelude::*;
  pub use super::canonical::prelude::*;
  pub use super::matrix::prelude::*;
}
