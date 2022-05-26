#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
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
  #[ cfg( any( feature = "runtime", feature = "former_runtime" ) ) ]
  pub use former_runtime;
  #[ cfg( any( feature = "meta", feature = "former_meta" ) ) ]
  pub use former_meta;
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
  #[ cfg( any( feature = "runtime", feature = "former_runtime" ) ) ]
  pub use former_runtime as runtime;
  #[ cfg( any( feature = "meta", feature = "former_meta" ) ) ]
  pub use former_meta as derive;
  #[ cfg( any( feature = "meta", feature = "former_meta" ) ) ]
  pub use derive::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
