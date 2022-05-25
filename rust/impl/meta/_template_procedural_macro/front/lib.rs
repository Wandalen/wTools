#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

//!
//! Protocol of modularity unifying interface of a module.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Namespace with dependencies.
pub mod dependency
{
  pub use procedural_macro_runtime;
  pub use procedural_macro_meta;
}

/// Own namespace of the module.
pub mod protected
{
  pub use super::exposed::*;
}

pub use protected::*;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use procedural_macro_runtime as runtime;
  pub use procedural_macro_meta as meta;
  pub use meta::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
