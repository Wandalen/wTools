#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
// #![ feature( concat_idents ) ]

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
/// Exporting/importing serialize/deserialize encoding/decoding macros, algorithms and structures for that.
///

pub mod convert;
pub use convert::*;

///
/// Collection of general purpose time tools.
///

pub mod time;

//

pub use werror as error;

// #[ cfg( feature = "with_proc_macro" ) ]
// pub use proc_macro_tools as proc_macro;

pub use former as former;
pub use woptions as options;
pub use winterval as interval;
pub use wstring_tools as string;

///
/// Prelude to use: `use wtools::prelude::*`.
///

pub mod prelude
{
  pub use super::*;
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
  pub use ::parse_display; /* xxx : move to stringing */

  // #[ cfg( debug_assertions ) ]
  // pub use ::wtest_basic;

}
