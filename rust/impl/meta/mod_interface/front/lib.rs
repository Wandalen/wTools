#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
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
  // #[ cfg( any( feature = "runtime", feature = "mod_interface_runtime" ) ) ]
  pub use mod_interface_runtime;
  // #[ cfg( any( feature = "meta", feature = "mod_interface_meta" ) ) ]
  pub use mod_interface_meta;
}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
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
  // #[ cfg( any( feature = "runtime", feature = "mod_interface_runtime" ) ) ]
  pub use mod_interface_runtime as runtime;
  // #[ cfg( any( feature = "meta", feature = "mod_interface_meta" ) ) ]
  pub use mod_interface_meta as meta;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  // #[ cfg( any( feature = "meta", feature = "mod_interface_meta" ) ) ]
  pub use mod_interface_meta::*;
}
