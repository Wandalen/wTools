#[ allow( unused_imports ) ]
use super::TheModule;

#[ path = "./meta/mod.rs" ]
mod meta;
#[ cfg( feature = "impls_index" ) ]
#[ path = "./impls_index/mod.rs" ]
mod impls_index;
#[ cfg( feature = "mod_interface" ) ]
#[ path = "./mod_interface/mod.rs" ]
mod mod_interface;
