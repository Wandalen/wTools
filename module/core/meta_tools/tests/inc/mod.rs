#[ allow( unused_imports ) ]
use super::*;

// #[ cfg( any( feature = "meta_constructors", feature = "meta_constructors" ) ) ]
// mod meta_constructor_test;

#[ cfg( any( feature = "meta_idents_concat", feature = "meta_idents_concat" ) ) ]
mod indents_concat_test;

#[ cfg( any( feature = "meta_for_each" ) ) ]
#[ path = "../../../for_each/tests/inc/mod.rs" ]
mod for_each_test;

#[ cfg( any( feature = "meta_impls_index" ) ) ]
#[ path = "../../../impls_index/tests/inc/mod.rs" ]
mod impls_index;

// #[ cfg( any( feature = "mod_interface", feature = "meta_mod_interface" ) ) ]
#[ path = "../../../mod_interface/tests/inc/mod.rs" ]
mod mod_interface;
