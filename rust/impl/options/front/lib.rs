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
  pub use ::woptions_runtime as runtime;
  pub use ::woptions_meta as meta;
}

/// Own namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use woptions_runtime as runtime;
  pub use woptions_meta as meta;
  pub use meta::Options;
  pub use former::derive::Former;
  pub use woptions_runtime::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use woptions_runtime::prelude::*;
}
