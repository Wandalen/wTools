#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Tools for writing and running tests.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Basics.
pub mod basic;

/// Dependencies.
pub mod dependencies
{
  pub use paste;
  pub use trybuild;
  pub use anyhow;
  pub use rustversion;
  pub use meta_tools;
}

pub use dependencies::*;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::basic::exposed::*;
  pub use meta_tools::{ impls, impls1, impls2, impls3, index };
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::basic::prelude::*;
}
