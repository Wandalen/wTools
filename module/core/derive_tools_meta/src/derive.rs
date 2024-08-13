
//!
//! Implement couple of derives of general-purpose.
//!

#[ allow( unused_imports ) ]
use macro_tools::prelude::*;
#[ allow( unused_imports ) ]
pub use iter_tools as iter;

#[ cfg( feature = "derive_as_mut" ) ]
pub mod as_mut;
#[ cfg( feature = "derive_as_ref" ) ]
pub mod as_ref;
#[ cfg( feature = "derive_deref" ) ]
pub mod deref;
#[ cfg( feature = "derive_deref_mut" ) ]
pub mod deref_mut;
#[ cfg( feature = "derive_from" ) ]
pub mod from;
#[ cfg( feature = "derive_index" ) ]
pub mod index;
#[ cfg( feature = "derive_index_mut" ) ]
pub mod index_mut;
#[ cfg( feature = "derive_inner_from" ) ]
pub mod inner_from;
#[ cfg( feature = "derive_new" ) ]
pub mod new;
#[ cfg( feature = "derive_variadic_from" ) ]
pub mod variadic_from;
#[ cfg( feature = "derive_not" ) ]
pub mod not;
#[ cfg( feature = "derive_phantom" ) ]
pub mod phantom;
