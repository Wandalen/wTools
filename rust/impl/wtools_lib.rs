#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v2_trans_rect_small.png" ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

//!
//! wTools - Collection of general purpose tools for solving problems. Fundamentally extend the language without spoiling, so may be used solely or in conjunction with another module of such kind.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/* zzz : register error_tools */
/* zzz : alias werror -> error_tools  */
/* zzz : register text_tools as alias for wstring */

/* zzz : implement module::mod_at */
/* zzz : implement and publish mod_expose */

/* zzz : use skeptic? */
/* zzz : rename dt -> adt */

/* zzz : make sure CD run test on both stable and nightly channels */
/* zzz : make sure CD run debug tests and release tests */
/* zzz : introduce tag to run fewer tests */

/// Dependencies.
pub mod dependencies
{
  #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  pub use ::meta_tools::former;
  #[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
  pub use ::meta_tools::options;

  #[ cfg( feature = "meta" ) ]
  pub use ::meta_tools;
  // #[ cfg( feature = "impls_index" ) ]
  // pub use ::impls_index;
  // #[ cfg( feature = "mod_interface" ) ]
  // pub use ::mod_interface;
  #[ cfg( feature = "typing" ) ]
  pub use ::typing_tools;
  #[ cfg( feature = "time" ) ]
  pub use ::time_tools;
  #[ cfg( feature = "string" ) ]
  pub use ::wstring_tools;
  #[ cfg( feature = "error" ) ]
  pub use ::werror;
  // #[ cfg( feature = "winterval" ) ]
  // pub use ::winterval;
  #[ cfg( feature = "derive" ) ]
  pub use ::derive_tools;
  #[ cfg( feature = "diagnostics" ) ]
  pub use ::diagnostics_tools;

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;

  #[ cfg( feature = "iter" ) ]
  pub use ::iter_tools as iter;
  #[ cfg( feature = "meta" ) ]
  pub use ::meta_tools as meta;
  #[ cfg( feature = "typing" ) ]
  pub use ::typing_tools as typing;
  #[ cfg( feature = "diagnostics" ) ]
  pub use ::diagnostics_tools as diagnostics;
  #[ cfg( any( feature = "dt", feature = "data_type" ) ) ]
  pub use ::data_type as dt;
  #[ cfg( feature = "time" ) ]
  pub use ::time_tools as time;
  #[ cfg( feature = "error" ) ]
  pub use ::werror as error;
  #[ cfg( feature = "string" ) ]
  pub use ::wstring_tools as string;
  #[ cfg( feature = "derive" ) ]
  pub use ::derive_tools as derive;

  #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  pub use ::meta_tools::former as former;
  #[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
  pub use ::meta_tools::options as options;

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
  #[ cfg( feature = "iter" ) ]
  pub use super::iter::exposed::*;
  #[ cfg( feature = "meta" ) ]
  pub use super::meta::exposed::*;
  #[ cfg( feature = "typing" ) ]
  pub use super::typing::exposed::*;
  #[ cfg( feature = "diagnostics" ) ]
  pub use super::diagnostics::exposed::*;
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
pub mod prelude
{
  #[ cfg( feature = "iter" ) ]
  pub use super::iter::prelude::*;
  #[ cfg( feature = "meta" ) ]
  pub use super::meta::prelude::*;
  #[ cfg( feature = "typing" ) ]
  pub use super::typing::prelude::*;
  #[ cfg( feature = "diagnostics" ) ]
  pub use super::diagnostics::prelude::*;
  #[ cfg( any( feature = "dt", feature = "data_type" ) ) ]
  pub use super::dt::prelude::*;
  #[ cfg( feature = "time" ) ]
  pub use super::time::prelude::*;
  #[ cfg( feature = "error" ) ]
  pub use super::error::prelude::*;
  #[ cfg( feature = "string" ) ]
  pub use super::string::prelude::*;
  #[ cfg( feature = "derive" ) ]
  pub use super::derive::prelude::*;

  #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  pub use super::former::prelude::*;
  #[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
  pub use super::options::prelude::*;

}
