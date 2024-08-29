/// Internal namespace.
mod private
{
  use crate::prelude::*;

  // macro_rules! NODE_ID
  // {
  //   () => { < Node as HasId >::Id };
  // }

  ///
  /// Canonical implementation of edge.
  ///

  #[ derive( Debug, Copy, Clone ) ]
  pub struct Edge< EdgeId = crate::IdentityWithInt, NodeId = crate::IdentityWithInt >
  where
    EdgeId : IdentityInterface,
    NodeId : IdentityInterface,
  {
    /// Input node.
    pub in_node : NodeId,
    /// Output node.
    pub out_node : NodeId,
    // /// Kind of the edge.
    // pub kind : Kind,
    /// Identifier.
    pub id : EdgeId,
  }

  //

  impl< EdgeId, NodeId > HasId
  for Edge< EdgeId, NodeId >
  where
    EdgeId : IdentityInterface,
    NodeId : IdentityInterface,

  {
    type Id = EdgeId;
    fn id( &self ) -> Self::Id
    {
      self.id
    }
  }

  //

  impl< EdgeId, NodeId > EdgeBasicInterface
  for Edge< EdgeId, NodeId >
  where
    EdgeId : IdentityInterface,
    NodeId : IdentityInterface,
  {
  }

  //

  impl< EdgeId, NodeId > PartialEq
  for Edge< EdgeId, NodeId >
  where
    EdgeId : IdentityInterface,
    NodeId : IdentityInterface,
  {
    fn eq( &self, other : &Self ) -> bool
    {
      self.id() == other.id()
    }
  }

  impl< EdgeId, NodeId > Eq
  for Edge< EdgeId, NodeId >
  where
    EdgeId : IdentityInterface,
    NodeId : IdentityInterface,
  {}
}

//

crate::mod_interface!
{
  orphan use super::private::Edge;
}
