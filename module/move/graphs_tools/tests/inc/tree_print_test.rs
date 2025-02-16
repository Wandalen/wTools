use super::*;

use graph::map_of_nodes::
{
  Node, NodeId, Graph,
};

// =

#[ test ]
fn write_as_dfs_tree()
{
  use the_module::tree_print::GraphDirectedPrintAsTree;
  let graph = Graph::duplet_assymetric();

  let mut got = String::new();
  let r = graph.write_as_dfs_tree( &mut got, 0.into() );
  let exp = "node::1";
  assert_eq!( got, exp );
  assert!( r.is_ok() );

}

//

#[ test ]
fn string_with_dfs_tree()
{
  use the_module::tree_print::GraphDirectedPrintAsTree;
  let graph = Graph::triplet_with_double_legs();

  let got = graph.string_with_dfs_tree( 0.into() );
  let exp = "node::1";
  println!( "{}", got );
  assert_eq!( got, exp );

}
