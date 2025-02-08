use super::*;

use derive_tools::From;
use the_module::abs;
use iter_tools::{ _IterTrait, IterTrait, BoxedIter };

#[ derive( Debug ) ]
pub struct Node< 'a >
{
  pub id : NodeId,
  pub children : Vec< &'a Node< 'a > >,
}

impl< 'a > the_module::abs::Node for Node< 'a > {}

impl< 'a > Node< 'a >
{
  pub fn new< IntoId : Into< NodeId > >( id : IntoId ) -> Node< 'a >
  {
    Node
    {
      id : id.into(),
      children : Vec::new(),
    }
  }

  pub fn add_child( &mut self, child : &'a Node< 'a > ) -> &mut Self
  {
    self.children.push( child );
    self
  }
}

#[ derive( Default ) ]
pub struct Graph< 'a >
{
  nodes : HashMap< NodeId, &'a Node< 'a > >,
}

impl< 'a > Graph< 'a >
{

  pub fn add_node( &mut self, node : &'a Node< 'a > )
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
pub struct NodeId( usize );

impl the_module::abs::NodeId for NodeId {}
