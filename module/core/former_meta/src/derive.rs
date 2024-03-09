
//!
//! Implement couple of derives of general-purpose.
//!

#[ allow( unused_imports ) ]
use macro_tools::prelude::*;
// pub use macro_tools::{ Result, Many };
// pub use iter_tools as iter;

#[ cfg( feature = "derive_former" ) ]
pub mod former;
#[ cfg( feature = "derive_component_from" ) ]
pub mod component_from;
