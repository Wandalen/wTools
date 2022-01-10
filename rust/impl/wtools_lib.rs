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

// pub mod meta;
// pub use meta::*;

// pub use meta_tools as meta;
// pub use meta::*;

///
/// Type checking tools.
///

pub mod typing;
pub use typing::*;

// pub use typing_tools as typing;
// pub use typing::*;

///
/// Exporting/importing tools to serialize/deserialize structures.
///

pub mod exporting;
pub use exporting::*;

// pub mod typing;
// pub use inspect_type::*;
// pub use is_slice::*;
// pub use implements::*;

///
/// Collection of general purpose time tools.
///

pub mod time;

//

pub use former as former;
pub use werror as error;

#[ cfg( feature = "with_proc_macro" ) ]
pub use wproc_macro as proc_macro;

///
/// Prelude to use: `use wtools::prelude::*`.
///

pub mod prelude
{
  pub use super::*;
}
