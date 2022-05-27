
#[ cfg( any( feature = "for_each", feature = "collection_make" ) ) ]
use super::TheModule;
#[ cfg( feature = "for_each" ) ]
mod for_each_test;
#[ cfg( feature = "collection_make" ) ]
mod generator_test;
