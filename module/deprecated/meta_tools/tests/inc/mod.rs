#[ allow( unused_imports ) ]
use super :: *;

#[ cfg( feature = "meta_idents_concat" ) ]
mod indents_concat_test;

#[ cfg( feature = "meta_for_each" ) ]
#[ path = "../../../for_each/tests/inc/mod.rs" ]
mod for_each_test;

#[ cfg( feature = "meta_impls_index" ) ]
#[ path = "../../../../core/impls_index/tests/inc/mod.rs" ]
mod impls_index;

// #[ cfg( any( feature = "mod_interface", feature = "meta_mod_interface" ) ) ]
#[ allow( unused_imports ) ]
#[ path = "../../../../core/mod_interface/tests/inc/mod.rs" ]
mod mod_interface;
