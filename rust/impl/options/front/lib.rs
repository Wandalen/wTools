#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Mechanism to define map of options for a function and its defaults laconically.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Namespace with dependencies.
pub mod dependency
{
  #[ cfg( any( feature = "runtime", feature = "woptions_runtime" ) ) ]
  pub use ::woptions_runtime as runtime;
  #[ cfg( any( feature = "meta", feature = "woptions_meta" ) ) ]
  pub use ::woptions_meta as meta;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  #[ cfg( any( feature = "runtime", feature = "woptions_runtime" ) ) ]
  pub use woptions_runtime as runtime;
  #[ cfg( any( feature = "meta", feature = "woptions_meta" ) ) ]
  pub use woptions_meta as meta;
  #[ cfg( any( feature = "meta", feature = "woptions_meta" ) ) ]
  pub use meta::Options;
  #[ cfg( feature = "former" ) ]
  pub use former::derive::Former;
  #[ cfg( any( feature = "runtime", feature = "woptions_runtime" ) ) ]
  pub use woptions_runtime::exposed::*;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  #[ cfg( any( feature = "runtime", feature = "woptions_runtime" ) ) ]
  pub use woptions_runtime::prelude::*;
}
