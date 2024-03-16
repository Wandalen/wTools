// use super::*;
//
// #[ cfg( feature = "cell_factory" ) ]
// tests_impls!
// {
//
//   fn node_make()
//   {
//     use TheModule::prelude::*;
//
//     let node : TheModule::canonical::Node = from!( 13 );
//     a_id!( node.id(), 13.into() );
//
//   }
//
//   fn nodecell_make()
//   {
//     use TheModule::prelude::*;
//
//     let node : TheModule::canonical::Node = from!( 13 );
//     a_id!( node.id(), 13.into() );
//     let cellnode : TheModule::NodeCell< _ > = from!( node );
//
//   }
//
// }
//
// //
//
// #[ cfg( feature = "cell_factory" ) ]
// tests_index!
// {
//
//   node_make,
//   nodecell_make,
//
// }
