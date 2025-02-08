use super::*;

use derive_tools::From;
use the_module::abs;
use iter_tools::{ _IterTrait, IterTrait, BoxedIter };

#[ derive( Debug ) ]
pub struct Node
{
  pub id : NodeId,
  pub children : Vec< NodeId >,
}

impl the_module::abs::Node for Node {}

impl Node
{
  pub fn new< IntoId : Into< NodeId > >( id : IntoId ) -> Node
  {
    Node
    {
      id : id.into(),
      children : Vec::new(),
    }
  }

  pub fn add_child( &mut self, child : &Node ) -> &mut Self
  {
    self.children.push( child.id );
    self
  }
}

#[ derive( Default ) ]
pub struct Graph
{
  nodes : HashMap< NodeId, Node >,
}

impl Graph
{

  pub fn add_node( &mut self, node : Node )
  {
    self.nodes.insert( node.id, node );
  }

}

impl< 'a > abs::GraphDirected< 'a > for Graph
{

  type NodeId = NodeId;
  type Node = Node;

  fn node_ref( &'a self, node_id : NodeId ) -> &'a Node
  {
    self.nodes.get( &node_id ).expect( "If id exist then node shoudl also exist" )
  }

  fn node_id( &self, node : &Node ) -> NodeId
  {
    node.id
  }

  fn node_out_nodes( &'a self, node_id : NodeId ) -> BoxedIter< 'a, Self::NodeId >
  {
    if let Some( node ) = self.nodes.get( &node_id )
    {
      Box::new( node.children.iter().cloned() )
    }
    else
    {
      Box::new( std::iter::empty() )
    }
  }
}

#[ derive( Debug, Copy, Clone, Hash, PartialEq, Eq, From ) ]
pub struct NodeId( usize );

impl the_module::abs::NodeId for NodeId {}

// xxx

use std::cell::RefCell;
impl Graph
{

  pub fn duplet() -> Self
  {

    // Create nodes
    let mut node0 = Node::new( 0 );
    let node1 = Node::new( 1 );
    let node2 = Node::new( 2 );

    // Set up the graph structure
    node0
    .add_child( &node1 )
    .add_child( &node2 )
    ;

    let mut graph = Self::default();
    graph.add_node( node0 );
    graph.add_node( node1 );
    graph.add_node( node2 );

    graph
  }

}
