use super::*;

use derive_tools::From;
use the_module::abs;
use iter_tools::{ _IterTrait, IterTrait, BoxedIter };

#[ derive( Debug ) ]
struct Node< 'a >
{
  id : NodeId,
  children : Vec< &'a Node< 'a > >,
}

impl< 'a > the_module::abs::Node for Node< 'a > {}

impl< 'a > Node< 'a >
{
  fn new< IntoId : Into< NodeId > >( id : IntoId ) -> Node< 'a >
  {
    Node
    {
      id : id.into(),
      children : Vec::new(),
    }
  }

  fn add_child( &mut self, child : &'a Node< 'a > ) -> &mut Self
  {
    self.children.push( child );
    self
  }
}

#[ derive( Default ) ]
struct Graph< 'a >
{
  nodes : HashMap< NodeId, &'a Node< 'a > >,
}

impl< 'a > Graph< 'a >
{

  // fn new() -> Graph< 'a >
  // {
  //   Graph
  //   {
  //     nodes : HashMap::new(),
  //   }
  // }

  fn add_node( &mut self, node : &'a Node< 'a > )
  {
    self.nodes.insert( node.id, node );
  }

}

impl< 'a > abs::GraphDirected< 'a > for Graph< 'a >
{

  type NodeId = NodeId;
  type Node = Node< 'a >;

  fn node_ref( &self, node_id : NodeId ) -> &'a Node< 'a >
  {
    self.nodes.get( &node_id ).expect( "If id exist then node shoudl also exist" )
  }

  fn node_id( &self, node : &'a Node< 'a > ) -> NodeId
  {
    node.id
  }

  fn node_out_nodes( &self, node_id : NodeId ) -> BoxedIter< 'a, Self::NodeId >
  {
    if let Some( node ) = self.nodes.get( &node_id )
    {
      Box::new( node.children.iter().map( | child | child.id ) )
    }
    else
    {
      Box::new( std::iter::empty() )
    }
  }
}

#[ derive( Debug, Copy, Clone, Hash, PartialEq, Eq, From ) ]
struct NodeId( usize );

impl the_module::abs::NodeId for NodeId {}

// =

#[ test ]
fn test_dfs()
{
  use the_module::search;
  use the_module::abs;
  use search::ForGraphDirected;

  // Create nodes
  let mut node1 = Node::new( NodeId( 1 ) );
  let node2 = Node::new( NodeId( 2 ) );
  let node3 = Node::new( NodeId( 3 ) );
  let node4 = Node::new( NodeId( 4 ) );

  // Set up the graph structure
  node1
  .add_child( &node2 )
  .add_child( &node3 )
  .add_child( &node4 );

  let mut graph = Graph::default();
  graph.add_node( &node1 );
  graph.add_node( &node2 );
  graph.add_node( &node3 );
  graph.add_node( &node4 );

  // Prepare a vector to collect visited nodes
  let mut visited_nodes = Vec::new();

  // Define the visit function
  let visit = | node : &'_ Node< '_ > |
  {
    visited_nodes.push( node.id );
  };

  // Create search options
  let search_options : search::Options< '_, search::Dfs, _, _ > = search::Options
  // let search_options = search::Options
  {
    start_id : NodeId( 1 ),
    visit,
    _extra : (),
    _phantom : std::marker::PhantomData,
  };

  // Perform DFS
  graph.search( search_options );

  // Assert the order of visited nodes
  assert_eq!( visited_nodes, vec![ NodeId( 1 ), NodeId( 4 ), NodeId( 3 ), NodeId( 2 ) ] );

}
