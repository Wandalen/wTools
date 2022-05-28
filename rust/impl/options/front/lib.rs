#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
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
  // #[ cfg( any( feature = "runtime", feature = "woptions_runtime" ) ) ]
  pub use ::woptions_runtime as runtime;
  // #[ cfg( any( feature = "meta", feature = "woptions_meta" ) ) ]
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
  // #[ cfg( any( feature = "runtime", feature = "woptions_runtime" ) ) ]
  pub use woptions_runtime as runtime;
  // #[ cfg( any( feature = "meta", feature = "woptions_meta" ) ) ]
  pub use woptions_meta as meta;
  // #[ cfg( any( feature = "meta", feature = "woptions_meta" ) ) ]
  pub use meta::Options;
  // #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  pub use former::derive::Former;
  // #[ cfg( any( feature = "runtime", feature = "woptions_runtime" ) ) ]
  pub use woptions_runtime::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  // #[ cfg( any( feature = "runtime", feature = "woptions_runtime" ) ) ]
  pub use woptions_runtime::prelude::*;
}
