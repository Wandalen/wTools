#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Collection of general purpose time tools.
//!

///
/// Collection of general purpose time tools.
///

pub mod time
{
  include!( "./now.rs" );
}

pub use time::*;
