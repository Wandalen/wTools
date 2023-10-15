#[ allow( unused_imports ) ]
use super::*;

// #[ path = "./meta/mod.rs" ]
// mod meta;

#[ cfg( any( feature = "for_each", feature = "meta_for_each" ) ) ]
#[ path = "../../../for_each/tests/inc/mod.rs" ]
// #[ path = concat!( env!( "MODULES_PATH" ), "/", "core/for_each/tests/inc/mod.rs" ) ]
mod for_each_test;

#[ cfg( any( feature = "collection_make", feature = "meta_collection_make" ) ) ]
// #[ path = "meta/collection_make_test.rs" ]
mod collection_make_test;

#[ cfg( any( feature = "idents_concat", feature = "meta_idents_concat" ) ) ]
// #[ path = "meta/indents_concat_test.rs" ]
mod indents_concat_test;

#[ cfg( any( feature = "impls_index", feature = "meta_impls_index" ) ) ]
// #[ path = "./impls_index/mod.rs" ]
#[ path = "../../../impls_index/tests/inc/mod.rs" ]
mod impls_index;
//
// // #[ cfg( any( feature = "mod_interface", feature = "meta_mod_interface" ) ) ]
// // #[ path = "./mod_interface/mod.rs" ]
// #[ path = "../../../mod_interface/tests/inc/mod.rs" ]
// mod mod_interface;

// xxx : move former / options tests here
