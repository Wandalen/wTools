#![ cfg_attr( feature = "no_std", no_std ) ] // zzz
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico")]
#![ doc( html_root_url = "https://docs.rs/wtools/latest/wtools/")]
// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

//!
//! wTools - Collection of general purpose tools for solving problems. Fundamentally extend the language without spoiling, so may be used solely or in conjunction with another module of such kind.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Namespace with dependencies.

#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  pub use ::meta_tools::former;
  #[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
  pub use ::meta_tools::options;
  #[ cfg( feature = "meta" ) ]
  pub use ::meta_tools;
  #[ cfg( feature = "mem" ) ]
  pub use ::mem_tools;
  // zzz
  // #[ cfg( feature = "impls_index" ) ]
  // pub use ::impls_index;
  // // #[ cfg( feature = "mod_interface" ) ]
  // pub use ::mod_interface;
  #[ cfg( feature = "typing" ) ]
  pub use ::typing_tools;
  #[ cfg( feature = "time" ) ]
  pub use ::time_tools;
  #[ cfg( feature = "string" ) ]
  pub use ::strs_tools;
  #[ cfg( feature = "error" ) ]
  pub use ::error_tools;
  // #[ cfg( feature = "winterval" ) ]
  // pub use ::winterval;
  #[ cfg( feature = "derive" ) ]
  pub use ::derive_tools;
  #[ cfg( feature = "diagnostics" ) ]
  pub use ::diagnostics_tools;

}

/// Protected namespace of the module.
#[ allow( unused_imports ) ]
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;

  #[ cfg( feature = "iter" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::iter_tools as iter;
  #[ cfg( feature = "meta" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::meta_tools as meta;
  #[ cfg( feature = "mem" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::mem_tools as mem;
  #[ cfg( feature = "typing" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::typing_tools as typing;
  #[ cfg( feature = "diagnostics" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::diagnostics_tools as diagnostics;
  #[ cfg( any( feature = "dt", feature = "data_type" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::data_type as dt;
  #[ cfg( feature = "time" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::time_tools as time;
  #[ cfg( feature = "error" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::error_tools as error;
  #[ cfg( feature = "string" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::strs_tools as string;
  #[ cfg( feature = "derive" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::derive_tools as derive;

  #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::meta_tools::former as former;
  #[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::meta_tools::options as options;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use super::prelude::*;
  #[ cfg( feature = "iter" ) ]
  pub use super::iter::exposed::*;
  #[ cfg( feature = "meta" ) ]
  pub use super::meta::exposed::*;
  #[ cfg( feature = "mem" ) ]
  pub use super::mem::exposed::*;
  #[ cfg( feature = "typing" ) ]
  pub use super::typing::exposed::*;
  #[ cfg( feature = "diagnostics" ) ]
  pub use super::diagnostics::diag::exposed::*;
  #[ cfg( any( feature = "dt", feature = "data_type" ) ) ]
  pub use super::dt::exposed::*;
  #[ cfg( feature = "time" ) ]
  pub use super::time::exposed::*;
  #[ cfg( feature = "error" ) ]
  pub use super::error::exposed::*;
  #[ cfg( feature = "string" ) ]
  pub use super::string::exposed::*;
  #[ cfg( feature = "derive" ) ]
  pub use super::derive::exposed::*;

  #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  pub use super::former::exposed::*;
  #[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
  pub use super::options::exposed::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ cfg( feature = "iter" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::iter::prelude::*;
  #[ cfg( feature = "meta" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::meta::prelude::*;
  #[ cfg( feature = "mem" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::mem::prelude::*;
  #[ cfg( feature = "typing" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::typing::prelude::*;
  #[ cfg( feature = "diagnostics" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::diagnostics::diag::prelude::*;
  #[ cfg( any( feature = "dt", feature = "data_type" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::dt::prelude::*;
  #[ cfg( feature = "time" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::time::prelude::*;
  #[ cfg( feature = "error" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::error::prelude::*;
  #[ cfg( feature = "string" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::string::prelude::*;

  #[ cfg( feature = "derive" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::derive::prelude::*;
  // zzz
  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::derive::prelude::clone_dyn;

  #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::former::prelude::*;
  #[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::options::prelude::*;
}
