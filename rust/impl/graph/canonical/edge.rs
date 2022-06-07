/// Internal namespace.
pub( crate ) mod private
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
  pub struct Edge< EdgeId = crate::IdentityWithInt, NodeId = crate::IdentityWithInt, Kind = crate::EdgeKindless >
  where
    EdgeId : IdentityInterface,
    NodeId : IdentityInterface,
    Kind : EdgeKindInterface,
  {
    /// Input node.
    pub in_node : NodeId,
    /// Output node.
    pub out_node : NodeId,
    /// Kind of the edge.
    pub kind : Kind,
    /// Identifier.
    pub id : EdgeId,
  }

  //

  impl< EdgeId, NodeId, Kind > HasId
  for Edge< EdgeId, NodeId, Kind >
  where
    EdgeId : IdentityInterface,
    NodeId : IdentityInterface,
    Kind : EdgeKindInterface,
  {
    type Id = EdgeId;
    fn id( &self ) -> Self::Id
    {
      self.id
    }
  }

  //

  impl< EdgeId, NodeId, Kind > EdgeBasicInterface
  for Edge< EdgeId, NodeId, Kind >
  where
    EdgeId : IdentityInterface,
    NodeId : IdentityInterface,
    Kind : EdgeKindInterface,
  {
  }

  //

  impl< EdgeId, NodeId, Kind > PartialEq
  for Edge< EdgeId, NodeId, Kind >
  where
    EdgeId : IdentityInterface,
    NodeId : IdentityInterface,
    Kind : EdgeKindInterface,
  {
    fn eq( &self, other : &Self ) -> bool
    {
      self.id() == other.id()
    }
  }

  impl< EdgeId, NodeId, Kind > Eq
  for Edge< EdgeId, NodeId, Kind >
  where
    EdgeId : IdentityInterface,
    NodeId : IdentityInterface,
    Kind : EdgeKindInterface,
  {}

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
  pub use super::private::Edge;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
