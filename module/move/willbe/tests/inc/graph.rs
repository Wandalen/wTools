mod toposort
{
  use crate::TheModule::graph::toposort;
  use std::collections::HashMap;
  use petgraph::Graph;

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
}