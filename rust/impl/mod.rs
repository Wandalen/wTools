#![warn( missing_docs )]
// #![ feature( concat_idents ) ]

//!
//! wTools - Collection of general purpose tools for solving problems. Fundamentally extend the language without spoiling, so may be used solely or in conjunction with another module of such kind.
//!

/// Meta tools.
pub mod meta;
pub use inspect_type::*;
pub use is_slice::*;
pub use implements::*;

pub use former as former;
pub use werror as error;

#[ cfg( feature = "proc_macro" ) ]
pub use wproc_macro as proc_macro;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::*;
}
