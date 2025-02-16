use super::*;

use graph::map_of_nodes::
{
  Node, NodeId, Graph,
};

// =

#[ test ]
fn _print_as_tree()
{
  use the_module::tree_print::GraphDirectedPrintAsTree;
  let graph = Graph::duplet_assymetric();

  let mut got = String::new();
  let r = graph._print_as_tree( &mut got, 0.into() );
  let exp = "node::1";
  assert_eq!( got, exp );
  assert!( r.is_ok() );

}

//

#[ test ]
fn print_as_tree()
{
  use the_module::tree_print::GraphDirectedPrintAsTree;
  let graph = Graph::triplet_with_double_legs();

  let got = graph.print_as_tree( 0.into() );
  let exp = "node::1";
  println!( "{}", got );
  assert_eq!( got, exp );

}
