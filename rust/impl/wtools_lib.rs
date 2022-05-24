#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

/* qqq : for Dima : wrong header */

//!
//! wTools - Collection of general purpose tools for solving problems. Fundamentally extend the language without spoiling, so may be used solely or in conjunction with another module of such kind.
//!
//! ### Sample
//! ```
//! use wtools::*;
//!
//! fn main()
//! {
//!   println!( "implements!( 13_i32 => Copy ) : {}", implements!( 13_i32 => Copy ) );
//!   println!( "implements!( Box::new( 13_i32 ) => Copy ) : {}", implements!( Box::new( 13_i32 ) => Copy ) );
//! }
//! ```

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

/// Iter tools.
#[ cfg( feature = "iter_tools" ) ]
pub mod iter;
#[ cfg( feature = "iter_tools" ) ]
#[ doc( inline ) ]
pub use iter::exposed::*;

/// Meta tools.
#[ cfg( feature = "meta_tools" ) ]
pub mod meta;
#[ cfg( feature = "meta_tools" ) ]
#[ doc( inline ) ]
pub use meta::*;

/// Type checking tools.
#[ cfg( feature = "typing_tools" ) ]
pub mod typing;
#[ cfg( feature = "typing_tools" ) ]
#[ doc( inline ) ]
pub use typing::*;

/// Collection of primal data types
#[ cfg( any( feature = "dt", feature = "data_type" ) ) ]
pub mod dt;

///
/// Collection of general purpose time tools.
///

#[ cfg( feature = "time_tools" ) ]
pub mod time;

/// Dependencies.
pub mod dependencies
{
  #[ cfg( feature = "former" ) ]
  pub use ::former;
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
  pub use ::wstring_tools;
  #[ cfg( feature = "werror" ) ]
  pub use ::werror;
  pub use ::winterval;
  #[ cfg( feature = "derive_tools" ) ]
  pub use ::derive_tools;

  // #[ cfg( debug_assertions ) ]
  // pub use ::wtest_basic;

}

/// Own namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  #[ cfg( feature = "mod_interface" ) ]
  pub use ::mod_interface::orphan::*;

  #[ cfg( feature = "werror" ) ]
  pub use ::werror as error;
  #[ cfg( feature = "former" ) ]
  pub use ::former as former;
  pub use ::woptions as options;
  pub use ::winterval as interval;
  pub use ::wstring_tools as string;
  #[ cfg( feature = "derive_tools" ) ]
  pub use ::derive_tools as derive;

  #[ cfg( feature = "iter_tools" ) ]
  pub use super::iter;
  #[ cfg( feature = "iter_tools" ) ]
  pub use iter::exposed::*;

  #[ cfg( feature = "meta_tools" ) ]
  pub use super::meta;
  #[ cfg( feature = "meta_tools" ) ]
  pub use meta::*;

  #[ cfg( feature = "typing_tools" ) ]
  pub use super::typing;
  #[ cfg( feature = "typing_tools" ) ]
  pub use typing::*;

  #[ cfg( any( feature = "dt", feature = "data_type" ) ) ]
  pub use super::dt;

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
  #[ cfg( feature = "mod_interface" ) ]
  pub use ::mod_interface::exposed::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // pub use mod_interface::*;
  // pub use ::mod_interface::prelude::*;

  // pub use super::*; /* zzz : remove later */
  #[ cfg( feature = "meta_tools" ) ]
  pub use super::meta::prelude::*;

}

