#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

/* qqq : for Dima : wrong header */

//!
//! wTools - Collection of general purpose tools for solving problems. Fundamentally extend the language without spoiling, so may be used solely or in conjunction with another module of such kind.
//!
//! # Sample
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

///
/// Iter tools.
///

pub mod iter;
pub use iter::exposed::*;

///
/// Meta tools.
///

pub mod meta;
pub use meta::*;

///
/// Type checking tools.
///

pub mod typing;
pub use typing::*;

///
/// Collection of primal data types
///

pub mod dt;

// ///
// /// Exporting/importing serialize/deserialize encoding/decoding macros, algorithms and structures for that.
// ///
//
// pub mod convert;
// pub use convert::*;

///
/// Collection of general purpose time tools.
///

pub mod time;

//

pub use werror as error;

// #[ cfg( feature = "with_proc_macro" ) ]
// pub use proc_macro_tools as proc_macro;

pub use ::former as former;
pub use ::woptions as options;
pub use ::winterval as interval;
pub use ::wstring_tools as string;
pub use ::derive_tools as derive;

///
/// Prelude to use: `use wtools::prelude::*`.
///

pub mod prelude
{
  pub use super::*;
  pub use super::meta::prelude::*;
}

///
/// Dependencies.
///

pub mod dependencies
{

  pub use ::former;
  pub use ::woptions;
  pub use ::meta_tools;
  pub use ::typing_tools;
  pub use ::time_tools;
  pub use ::wstring_tools;
  pub use ::werror;
  pub use ::winterval;
  pub use ::derive_tools;

  // #[ cfg( debug_assertions ) ]
  // pub use ::wtest_basic;

}

// zzz : try to use instead of feature::nightly
// #![cfg_attr(feature = "nightly", feature(unsize))]

// /// Internal namespace.
// mod internal
// {
//
//   /// X2 Vector of cgmath
//   pub type X2< Scalar > = cgmath::Vector2< Scalar >;
//
// }
//
// /// Trait to interpret math data structures of other math libs as their analogs in cgmath to use operations of cgmath.
// pub mod as_native;
// #[
//   cfg( all
//   (
//     not( feature = "nalgebra_ops" ),
//     not( all( feature = "default_ops", feature = "nalgebra" ) ),
//     any( feature = "default_ops", feature = "cgmath_ops" ),
//   ))
// ]
// /// Use cgmath's operations.
// pub mod ops;
// /// Implement interfaces for objects of the math library.
// pub mod x2;
//
// /// Own namespace of the module.
// pub mod protected
// {
//   pub use super::exposed::*;
//   use super::internal as i;
//   pub use i::X2;
// }
//
// pub use protected::*;
//
// /// Exposed namespace of the module.
// pub mod exposed
// {
//   pub use super::prelude::*;
//   pub use super::as_native::exposed::*;
//   #[
//     cfg( all
//     (
//       not( feature = "nalgebra_ops" ),
//       not( all( feature = "default_ops", feature = "nalgebra" ) ),
//       any( feature = "default_ops", feature = "cgmath_ops" ),
//     ))
//   ]
//   pub use super::ops::exposed::*;
//   pub use super::x2::exposed::*;
// }
//
// pub use exposed::*;
//
// /// Prelude to use: `use wtools::prelude::*`.
// pub mod prelude
// {
//   pub use super::as_native::prelude::*;
//   #[
//     cfg( all
//     (
//       not( feature = "nalgebra_ops" ),
//       not( all( feature = "default_ops", feature = "nalgebra" ) ),
//       any( feature = "default_ops", feature = "cgmath_ops" ),
//     ))
//   ]
//   pub use super::ops::prelude::*;
//   pub use super::x2::prelude::*;
// }
//
// /* zzz : implement macro mod_adopt? */
// mod_adopt!
// {
//
//   /// Trait to interpret math data structures of other math libs as their analogs in cgmath to use operations of cgmath.
//   mod as_native;
//
//   #[
//     cfg( all
//     (
//       not( feature = "nalgebra_ops" ),
//       not( all( feature = "default_ops", feature = "nalgebra" ) ),
//       any( feature = "default_ops", feature = "cgmath_ops" ),
//     ))
//   ]
//   /// Use cgmath's operations.
//   mod ops;
//
//   /// Implement interfaces for objects of the math library.
//   mod x2;
//
//   own X2;
//
// }
