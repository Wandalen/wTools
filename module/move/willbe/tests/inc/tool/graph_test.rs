use super::*;

// qqq : for Bohdan : bad. don't import the_module::*
// use the_module::*;
use the_module::graph::toposort;
use test_tools::collection::HashMap;
use petgraph::Graph;
use willbe::graph::topological_sort_with_grouping;

struct IndexMap< T >( HashMap< T, usize > );

impl< T > IndexMap< T >
where
  T : std::hash::Hash + Eq,
{
  pub fn new( elements : Vec< T > ) -> Self
  {
    let index_map = elements.into_iter().enumerate().map( |( index, value )| ( value, index ) ).collect();
    Self( index_map )
  }

  pub fn position( &self, element : &T ) -> usize
  {
    self.0[ element ]
  }
}

#[ test ]
fn no_dependency()
{
  let mut graph = Graph::new();

  let _node1 = graph.add_node( &"A" );
  let _node2 = graph.add_node( &"B" );

  let sorted = toposort( graph ).unwrap();

  let index_map = IndexMap::new( sorted );
  let node1_position = index_map.position( &"A" );
  let node2_position = index_map.position( &"B" );

  assert!( node1_position < node2_position );
}

#[ test ]
fn a_depends_on_b()
{
  let mut graph = Graph::new();

  let node1 = graph.add_node( &"A" );
  let node2 = graph.add_node( &"B" );

  graph.add_edge( node1, node2, &"" );

  let sorted = toposort( graph ).unwrap();

  let index_map = IndexMap::new( sorted );
  let node1_position = index_map.position( &"A" );
  let node2_position = index_map.position( &"B" );

  assert!( node1_position > node2_position );
}

#[ test ]
fn multiple_dependencies()
{
  let mut graph = Graph::new();

  let a = graph.add_node( &"A" );
  let b = graph.add_node( &"B" );
  let c = graph.add_node( &"C" );

  graph.add_edge( a, b, &"" );
  graph.add_edge( a, c, &"" );

  let sorted = toposort( graph ).unwrap();

  let index_map = IndexMap::new( sorted );
  let a_position = index_map.position( &"A" );
  let b_position = index_map.position( &"B" );
  let c_position = index_map.position( &"C" );

  assert!( a_position > b_position );
  assert!( a_position > c_position );
}

#[ test ]
fn transitive_dependencies()
{
  let mut graph = Graph::new();

  let a = graph.add_node( &"A" );
  let b = graph.add_node( &"B" );
  let c = graph.add_node( &"C" );

  graph.add_edge( a, b, &"" );
  graph.add_edge( b, c, &"" );

  let sorted = toposort( graph ).unwrap();

  let index_map = IndexMap::new( sorted );
  let a_position = index_map.position( &"A" );
  let b_position = index_map.position( &"B" );
  let c_position = index_map.position( &"C" );

  assert!( a_position > b_position );
  assert!( b_position > c_position );
}

#[ test ]
#[ should_panic( expected = "Cycle" ) ]
fn cycle()
{
  let mut graph = Graph::new();

  let node1 = graph.add_node( &"A" );
  let node2 = graph.add_node( &"B" );

  graph.add_edge( node1, node2, &"" );
  graph.add_edge( node2, node1, &"" );

  let _sorted = toposort( graph ).unwrap();
}

// input
// B -> A
// C -> A
// output
// [A], [B,C]
#[ test ]
fn simple_case()
{
  let mut graph = Graph::new();

  let a_node = graph.add_node( &"A" );
  let b_node = graph.add_node( &"B" );
  let c_node = graph.add_node( &"C" );

  graph.add_edge( b_node, a_node, &"B->A");
  graph.add_edge( c_node, a_node, &"C->A");

  let groups = topological_sort_with_grouping( graph );

  assert_eq!( groups[ 0 ], vec![ "A" ] );
  assert_eq!( groups[1].len(), 2 );
  assert!( groups[ 1 ].contains( &"C" ) );
  assert!( groups[ 1 ].contains( &"B" ) );
}

// input
// digraph {
//     0 [ label = "0" ]
//     1 [ label = "1" ]
//     2 [ label = "2" ]
//     3 [ label = "3" ]
//     4 [ label = "4" ]
//     5 [ label = "5" ]
//     6 [ label = "6" ]
//     7 [ label = "7" ]
//     4 -> 0 [ label = "" ]
//     5 -> 0 [ label = "" ]
//     6 -> 0 [ label = "" ]
//     1 -> 3 [ label = "" ]
//     2 -> 3 [ label = "" ]
//     7 -> 6 [ label = "" ]
//     3 -> 4 [ label = "" ]
//     3 -> 5 [ label = "" ]
//     3 -> 6 [ label = "" ]
// }
// visualization : https://viz-js.com/?dot=ZGlncmFwaCB7CiAgICAwIFsgbGFiZWwgPSAiMCIgXQogICAgMSBbIGxhYmVsID0gIjEiIF0KICAgIDIgWyBsYWJlbCA9ICIyIiBdCiAgICAzIFsgbGFiZWwgPSAiMyIgXQogICAgNCBbIGxhYmVsID0gIjQiIF0KICAgIDUgWyBsYWJlbCA9ICI1IiBdCiAgICA2IFsgbGFiZWwgPSAiNiIgXQogICAgNyBbIGxhYmVsID0gIjciIF0KICAgIDQgLT4gMCBbIGxhYmVsID0gIiIgXQogICAgNSAtPiAwIFsgbGFiZWwgPSAiIiBdCiAgICA2IC0-IDAgWyBsYWJlbCA9ICIiIF0KICAgIDEgLT4gMyBbIGxhYmVsID0gIiIgXQogICAgMiAtPiAzIFsgbGFiZWwgPSAiIiBdCiAgICA3IC0-IDYgWyBsYWJlbCA9ICIiIF0KICAgIDMgLT4gNCBbIGxhYmVsID0gIiIgXQogICAgMyAtPiA1IFsgbGFiZWwgPSAiIiBdCiAgICAzIC0-IDYgWyBsYWJlbCA9ICIiIF0KfQo~
// output
// [0], [6,5,4], [3], [1,2,7]
#[ test ]
fn complicated_test()
{
  let mut graph = Graph::new();

  let n = graph.add_node( &"0" );
  let n_1 = graph.add_node( &"1" );
  let n_2 = graph.add_node( &"2" );
  let n_3 = graph.add_node( &"3" );
  let n_4 = graph.add_node( &"4" );
  let n_5 = graph.add_node( &"5" );
  let n_6 = graph.add_node( &"6" );
  let n_7 = graph.add_node( &"7" );

  graph.add_edge( n_1, n_3, &"" );
  graph.add_edge( n_2, n_3, &"" );
  graph.add_edge( n_7, n_6, &"" );

  graph.add_edge( n_3, n_4, &"" );
  graph.add_edge( n_3, n_5, &"" );
  graph.add_edge( n_3, n_6, &"" );

  graph.add_edge( n_4, n, &"" );
  graph.add_edge( n_5, n, &"" );
  graph.add_edge( n_6, n, &"" );

  let groups = topological_sort_with_grouping( graph );

  dbg!(&groups);

  assert_eq!( groups[ 0 ], vec![ "0" ] );

  assert_eq!( groups[1].len(), 3 );
  assert!( groups[ 1 ].contains( &"6" ) );
  assert!( groups[ 1 ].contains( &"5" ) );
  assert!( groups[ 1 ].contains( &"4" ) );

  assert_eq!( groups[ 2 ], vec![ "3" ] );

  assert_eq!( groups[3].len(), 3 );
  assert!( groups[ 3 ].contains( &"1" ) );
  assert!( groups[ 3 ].contains( &"2" ) );
  assert!( groups[ 3 ].contains( &"7" ) );
}
