#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

/* qqq : for Dima : wrong header */ /* aaa : Dmytro : used valid header in modules */

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

// /// Iter tools.
// #[ cfg( feature = "iter_tools" ) ]
// pub mod iter;
// #[ cfg( feature = "iter_tools" ) ]
// #[ doc( inline ) ]
// pub use iter::exposed::*;
//
// /// Meta tools.
// #[ cfg( feature = "meta_tools" ) ]
// pub mod meta;
// #[ cfg( feature = "meta_tools" ) ]
// #[ doc( inline ) ]
// pub use meta::*;
//
// /// Type checking tools.
// #[ cfg( feature = "typing_tools" ) ]
// pub mod typing;
// #[ cfg( feature = "typing_tools" ) ]
// #[ doc( inline ) ]
// pub use typing::*;
//
// /// Collection of primal data types
// #[ cfg( any( feature = "dt", feature = "data_type" ) ) ]
// pub mod dt;
//
// ///
// /// Collection of general purpose time tools.
// ///
//
// #[ cfg( feature = "time_tools" ) ]
// pub mod time;

/// Dependencies.
pub mod dependencies
{
  #[ cfg( feature = "former" ) ]
  pub use ::former;
  #[ cfg( feature = "woptions" ) ]
  pub use ::woptions;

  #[ cfg( feature = "meta_tools" ) ]
  pub use ::meta_tools;
  #[ cfg( feature = "impls_index" ) ]
  pub use ::impls_index;
  #[ cfg( feature = "mod_interface" ) ]
  pub use ::mod_interface;

  #[ cfg( feature = "typing_tools" ) ]
  pub use ::typing_tools;
  #[ cfg( feature = "time_tools" ) ]
  pub use ::time_tools;
  #[ cfg( feature = "wstring_tools" ) ]
  pub use ::wstring_tools;
  #[ cfg( feature = "werror" ) ]
  pub use ::werror;
  #[ cfg( feature = "winterval" ) ]
  pub use ::winterval;
  #[ cfg( feature = "derive_tools" ) ]
  pub use ::derive_tools;
  #[ cfg( feature = "diagnostics" ) ]
  pub use ::diagnostics_tools;

  // #[ cfg( debug_assertions ) ]
  // pub use ::wtest_basic;

}

/// Own namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  // pub use ::mod_interface::orphan::*;

  #[ cfg( feature = "iter_tools" ) ]
  pub use ::iter_tools as iter;
  #[ cfg( feature = "meta_tools" ) ]
  pub use ::meta_tools as meta;
  #[ cfg( feature = "typing_tools" ) ]
  pub use ::typing_tools as typing;
  #[ cfg( feature = "diagnostics" ) ]
  pub use ::diagnostics_tools as diagnostics;
  #[ cfg( any( feature = "dt", feature = "data_type" ) ) ]
  pub use ::data_type as dt;
  #[ cfg( feature = "time_tools" ) ]
  pub use ::time_tools as time;
  #[ cfg( feature = "werror" ) ]
  pub use ::werror as error;
  #[ cfg( feature = "former" ) ]
  pub use ::former as former;
  #[ cfg( feature = "woptions" ) ]
  pub use ::woptions as options;
  #[ cfg( feature = "winterval" ) ]
  pub use ::winterval as interval;
  #[ cfg( feature = "wstring_tools" ) ]
  pub use ::wstring_tools as string;
  #[ cfg( feature = "derive_tools" ) ]
  pub use ::derive_tools as derive;

  // #[ cfg( feature = "iter_tools" ) ]
  // pub use super::iter;
  // #[ cfg( feature = "iter_tools" ) ]
  // pub use iter::exposed::*;
  //
  // #[ cfg( feature = "meta_tools" ) ]
  // pub use super::meta;
  // #[ cfg( feature = "meta_tools" ) ]
  // pub use meta::*;
  //
  // #[ cfg( feature = "typing_tools" ) ]
  // pub use super::typing;
  // #[ cfg( feature = "typing_tools" ) ]
  // pub use typing::*;
  //
  // #[ cfg( any( feature = "dt", feature = "data_type" ) ) ]
  // pub use super::dt;
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
  #[ cfg( feature = "iter_tools" ) ]
  pub use super::iter::exposed::*;
  #[ cfg( feature = "meta_tools" ) ]
  pub use super::meta::exposed::*;
  #[ cfg( feature = "typing_tools" ) ]
  pub use super::typing::exposed::*;
  #[ cfg( feature = "diagnostics" ) ]
  pub use super::diagnostics::exposed::*;
  #[ cfg( any( feature = "dt", feature = "data_type" ) ) ]
  pub use super::dt::exposed::*;
  #[ cfg( feature = "time_tools" ) ]
  pub use super::time::exposed::*;
  #[ cfg( feature = "werror" ) ]
  pub use super::error::exposed::*;
  #[ cfg( feature = "former" ) ]
  pub use super::former::exposed::*;
  #[ cfg( feature = "woptions" ) ]
  pub use super::options::exposed::*;
  #[ cfg( feature = "winterval" ) ]
  pub use super::interval::exposed::*;
  #[ cfg( feature = "wstring_tools" ) ]
  pub use super::string::exposed::*;
  #[ cfg( feature = "derive_tools" ) ]
  pub use super::derive::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "iter_tools" ) ]
  pub use super::iter::prelude::*;
  #[ cfg( feature = "meta_tools" ) ]
  pub use super::meta::prelude::*;
  #[ cfg( feature = "typing_tools" ) ]
  pub use super::typing::prelude::*;
  #[ cfg( feature = "diagnostics" ) ]
  pub use super::diagnostics::prelude::*;
  #[ cfg( any( feature = "dt", feature = "data_type" ) ) ]
  pub use super::dt::prelude::*;
  #[ cfg( feature = "time_tools" ) ]
  pub use super::time::prelude::*;
  #[ cfg( feature = "werror" ) ]
  pub use super::error::prelude::*;
  #[ cfg( feature = "former" ) ]
  pub use super::former::prelude::*;
  #[ cfg( feature = "woptions" ) ]
  pub use super::options::prelude::*;
  #[ cfg( feature = "winterval" ) ]
  pub use super::interval::prelude::*;
  #[ cfg( feature = "wstring_tools" ) ]
  pub use super::string::prelude::*;
  #[ cfg( feature = "derive_tools" ) ]
  pub use super::derive::prelude::*;
}
