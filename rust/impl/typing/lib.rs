#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Collection of general purpose tools for type checking.
//!

///
/// Collection of general purpose tools for type checking.
///

pub mod typing
{
  pub use inspect_type::*;
  pub use is_slice::*;
  pub use implements::*;
}

pub use typing::*;
