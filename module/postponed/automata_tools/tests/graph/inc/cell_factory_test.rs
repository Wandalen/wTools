// use super::*;
// #[ cfg( feature = "canonical" ) ]
// use the_module::canonical::CellNodeFactory as GenerativeNodeFactory;
//
// #[ cfg( feature = "canonical" ) ]
// include!( "./factory_impls.rs" );
//
// #[ cfg( feature = "canonical" ) ]
// tests_impls!
// {
//
//   fn nodecell_make()
//   {
//     use the_module::prelude::*;
//
//     let node : the_module::canonical::Node = from!( 13 );
//     a_id!( node.id(), 13.into() );
//     let cellnode : < the_module::canonical::CellNodeFactory as GraphNodesNominalInterface >::NodeHandle = from!( node );
//
//   }
//
// }
//
// //
//
// #[ cfg( feature = "canonical" ) ]
// tests_index!
// {
//
//   node,
//   basic,
//   make_default,
//   make_with_edge_list,
//   make_with_edge_list_string,
//   graph_print,
//
//   nodecell_make,
//
// }
