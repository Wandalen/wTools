
#[ allow( unused_imports ) ]
use super::TheModule;

#[ cfg( any( feature = "for_each", feature = "meta_for_each" ) ) ]
mod for_each_test;

#[ cfg( any( feature = "collection_make", feature = "meta_collection_make" ) ) ]
mod collection_make_test;
