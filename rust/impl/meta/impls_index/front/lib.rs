#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Several of macros to put each function under a named macro to index every function in a class.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Collection of general purpose meta tools.
#[ path = "./mod.rs" ]
pub mod impls_index;

/// Dependencies.
pub mod dependencies
{
  // pub use ::literally;
  // pub use ::for_each;
}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  pub use super::impls_index::orphan::*;
}

pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::impls_index::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::impls_index::prelude::*;
}
