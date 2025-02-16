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
  println!( "{}", got );
  let exp = r#"node::0
├─ node::1
│  ├─ node::4
│  ├─ node::5
├─ node::2
├─ node::3
│  ├─ node::6
│  ├─ node::7
"#;
  assert_eq!( got, exp );

}

//

#[ test ]
fn string_with_bfs_tree()
{
  use the_module::tree_print::GraphDirectedPrintAsTree;
  let graph = Graph::triplet_with_double_legs();

  let got = graph.string_with_bfs_tree( 0.into() );
  println!( "{}", got );
  let exp = r#"node::0
├─ node::1
│  ├─ node::4
│  ├─ node::5
├─ node::2
├─ node::3
│  ├─ node::6
│  ├─ node::7
"#;
  assert_eq!( got, exp );

}
