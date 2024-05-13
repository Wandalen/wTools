#[ allow( unused_imports ) ]
use super::*;

#[ cfg( any( feature = "collection_into_constructors") ) ]
mod into_constructors;

#[ cfg( any( feature = "collection_constructors" ) ) ]
mod constructors;

#[ cfg( any( feature = "collection_std" ) ) ]
mod reexport;

mod components;

// qqq : make subdirectory for each container
// qqq : don't put tests otsude of directory `inc`
