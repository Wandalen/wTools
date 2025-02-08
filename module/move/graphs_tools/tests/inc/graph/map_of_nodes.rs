use super::*;

use derive_tools::From;
use the_module::abs;
use iter_tools::{ _IterTrait, IterTrait, BoxedIter };
use std::fmt;

#[ derive( Debug ) ]
pub struct Node
{
  pub id : NodeId,
  pub children : Vec< NodeId >,
}

impl the_module::abs::Node for Node {}

#[ allow( dead_code ) ]
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

  pub fn child_add( &mut self, child : &Node ) -> &mut Self
  {
    self.children.push( child.id );
    self
  }

  pub fn children_add< 'a, I >( &mut self, nodes : I ) -> &mut Self
  where
    I : IntoIterator< Item = &'a Node >,
  {
    for node in nodes
    {
      self.children.push( node.id );
    }
    self
  }

}

#[ derive( Default ) ]
pub struct Graph
{
  nodes : HashMap< NodeId, Node >,
}

#[ allow( dead_code ) ]
impl Graph
{

  pub fn node_add( &mut self, node : Node )
  {
    self.nodes.insert( node.id, node );
  }

  pub fn nodes_add< 'a, I >( &mut self, nodes : I ) -> &mut Self
  where
    I : IntoIterator< Item = Node >,
  {
    for node in nodes
    {
      self.nodes.insert( node.id, node );
    }
    self
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

#[ derive( Copy, Clone, Hash, PartialEq, Eq, From ) ]
pub struct NodeId( usize );

impl fmt::Debug for NodeId
{
  fn fmt( &self, c : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    c
    .write_fmt( format_args!( "node::{:?}", self.0 ) )
  }
}

impl the_module::abs::NodeId for NodeId {}

// Constructors

#[ allow( dead_code ) ]
impl Graph
{

  pub fn duplet() -> Self
  {

    // Create nodes
    let mut node0 = Node::new( 0 );
    let node1 = Node::new( 1 );
    let node2 = Node::new( 2 );

    // Set up the graph structure
    node0.children_add([ &node1, &node2 ]);

    let mut graph = Self::default();
    graph.nodes_add([ node0, node1, node2 ]);

    graph
  }

  pub fn duplet_assymetric() -> Self
  {

    // Create nodes
    let mut node0 = Node::new( 0 );
    let node1 = Node::new( 1 );
    let mut node2 = Node::new( 2 );
    let node3 = Node::new( 3 );

    node0.children_add([ &node1, &node2 ]);
    node2.children_add([ &node3 ]);

    let mut graph = Self::default();
    graph.nodes_add([ node0, node1, node2, node3 ]);

    graph
  }

}
