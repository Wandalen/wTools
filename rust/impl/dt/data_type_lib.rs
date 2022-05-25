#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Collection of primal data types.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Collection of primal data types.
pub mod dt;

/// Dependencies.
pub mod dependencies
{
  pub use ::either;
}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  pub use super::dt::orphan::*;
}

pub use protected::*;

/// Shared with parent namespace of the module
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::dt::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::dt::prelude::*;
}

// zzz : use
// https://github.com/CAD97/pointer-utils/tree/master/crates/slice-dst
