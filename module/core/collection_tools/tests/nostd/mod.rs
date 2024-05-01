#[ allow( unused_imports ) ]
use super::*;

#[ cfg( any( feature = "collection_constructors" ) ) ]
mod constructors;

// aaa : xxx : does not work for `use_alloc`, make it working -- Made by switching from std collections to alloc / hashbrown
// #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( any( feature = "collection_into_constructors" ) ) ]
mod into_constructors;

#[ cfg( any( feature = "collection_std" ) ) ]
mod reexport;
