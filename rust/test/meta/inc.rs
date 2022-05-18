
// mod for_each_test
// {
//   use super::TheModule;
//   include!( "./all/for_each_test.rs" );
// }
//
// mod generator_test
// {
//   use super::TheModule;
//   include!( "./all/generator_test.rs" );
// }
//
// mod impls_index
// {
//   use super::TheModule;
//   include!( "./impls_index/mod.rs" );
// }
//
// mod mod_interface
// {
//   // use super::TheModule;
//   include!( "./mod_interface/mod.rs" );
// }

use super::TheModule;

#[ path = "./meta/mod.rs" ]
mod meta;
#[ path = "./impls_index/mod.rs" ]
mod impls_index;
#[ path = "./mod_interface/mod.rs" ]
mod mod_interface;
