#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
// #![ feature( concat_idents ) ]

//!
//! wTools - Collection of general purpose tools for solving problems. Fundamentally extend the language without spoiling, so may be used solely or in conjunction with another module of such kind.
//!

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

///
/// Prelude to use: `use wtools::prelude::*`.
///

pub mod prelude
{
  pub use super::*;
}
