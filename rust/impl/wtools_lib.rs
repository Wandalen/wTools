#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

/* qqq : for Dima : wrong header */

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
// pub mod iter;
// // #[ doc( inline ) ]
// // pub use iter::exposed::*;
//
// /// Meta tools.
// pub mod meta;
// // #[ doc( inline ) ]
// // pub use meta::*;
//
// /// Type checking tools.
// pub mod typing;
// // #[ doc( inline ) ]
// // pub use typing::*;
//
// /// Diagnostics tools.
// pub mod diagnostics;
// // #[ doc( inline ) ]
// // pub use diagnostics::*;
//
// /// Collection of primal data types
// pub mod dt;
//
// /// Collection of general purpose time tools.
// pub mod time;

/// Dependencies.
pub mod dependencies
{

  pub use ::former;
  pub use ::woptions;

  pub use ::meta_tools;
  pub use ::impls_index;
  pub use ::mod_interface;

  pub use ::typing_tools;
  pub use ::time_tools;
  pub use ::wstring_tools;
  pub use ::werror;
  pub use ::winterval;
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

  pub use ::iter_tools as iter;
  pub use ::meta_tools as meta;
  pub use ::typing_tools as typing;
  pub use ::diagnostics_tools as diagnostics;
  pub use ::data_type as dt;
  pub use ::time_tools as time;
  pub use ::werror as error;
  pub use ::former as former;
  pub use ::woptions as options;
  pub use ::winterval as interval;
  pub use ::wstring_tools as string;
  pub use ::derive_tools as derive;

//   pub use super::iter;
//   pub use iter::exposed::*;
//
//   pub use super::meta;
//   pub use meta::*;
//
//   pub use super::typing;
//   pub use typing::*;
//
//   pub use super::dt;

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
  pub use super::iter::exposed::*;
  pub use super::meta::exposed::*;
  pub use super::typing::exposed::*;
  #[ cfg( feature = "diagnostics" ) ]
  pub use super::diagnostics::exposed::*;
  pub use super::dt::exposed::*;
  pub use super::time::exposed::*;
  pub use super::time::exposed::*;
  pub use super::error::exposed::*;
  pub use super::former::exposed::*;
  pub use super::options::exposed::*;
  pub use super::interval::exposed::*;
  pub use super::string::exposed::*;
  pub use super::derive::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::iter::prelude::*;
  pub use super::meta::prelude::*;
  pub use super::typing::prelude::*;
  #[ cfg( feature = "diagnostics" ) ]
  pub use super::diagnostics::prelude::*;
  pub use super::dt::prelude::*;
  pub use super::time::prelude::*;
  pub use super::time::prelude::*;
  pub use super::error::prelude::*;
  pub use super::former::prelude::*;
  pub use super::options::prelude::*;
  pub use super::interval::prelude::*;
  pub use super::string::prelude::*;
  pub use super::derive::prelude::*;
}
