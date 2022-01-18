#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Former - variation of builder pattern. Implementation of its runtime.
//!
//! Not intended to be used without derive. This module and derive is aggregate in module::former is [here](https://github.com/Wandalen/wTools/tree/master/module/rust/former).
//!

mod vector;
mod hash_map;
mod hash_set;

pub use vector::*;
pub use hash_map::*;
pub use hash_set::*;
