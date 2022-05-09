#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! The tool to make CLI ( commands user interface ). It is able to aggregate external binary applications, as well as functions, which are written in your language.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Commands aggregator library.
pub mod ca
{
  include!( "./lib.rs" );
}

pub use ca::*;

/// Exposed namespace of the module.
pub mod exposed
{
}
pub use exposed::*;

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  pub use super::ca::prelude::*;
}
