#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

//!
//! Former - variation of builder pattern.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Namespace with dependencies.
pub mod dependency
{
  pub use former_runtime;
  pub use former_meta;
}

/// Own namespace of the module.
pub mod own
{
  pub use super::exposed::*;
}

pub use own::*;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use former_runtime as runtime;
  pub use former_meta as derive;
  pub use derive::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
}
