#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Collection of primal data types.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Collection of primal data types.
pub mod dt;

/// Owned namespace of the module.
pub mod protected
{
  pub use super::exposed::*;
  pub use super::dt::orphan::*;
}

pub use protected::*;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::dt::exposed::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::dt::prelude::*;
}

// zzz : use
// https://github.com/CAD97/pointer-utils/tree/master/crates/slice-dst
