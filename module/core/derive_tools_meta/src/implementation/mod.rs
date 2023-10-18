
//!
//! Implement couple of derives of general-purpose.
//!

use macro_tools::prelude::*;
pub use macro_tools::Result;

pub mod input;
use input::*;
pub mod as_mut;
pub mod as_ref;
pub mod deref;
pub mod deref_mut;
pub mod from_inner;
pub mod inner_from;
