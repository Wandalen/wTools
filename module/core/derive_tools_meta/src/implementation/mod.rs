
//!
//! Implement couple of derives of general-purpose.
//!

use macro_tools::prelude::*;
pub use macro_tools::Result;

pub mod input;
use input::*;
#[ cfg( feature = "derive_as_mut" ) ]
pub mod as_mut;
#[ cfg( feature = "derive_as_ref" ) ]
pub mod as_ref;
#[ cfg( feature = "derive_deref" ) ]
pub mod deref;
#[ cfg( feature = "derive_deref_mut" ) ]
pub mod deref_mut;
#[ cfg( feature = "derive_from" ) ]
pub mod from_inner;
#[ cfg( feature = "derive_from" ) ]
pub mod inner_from;
