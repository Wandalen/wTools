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

  #[ derive( Debug, PartialEq, Copy, Clone ) ]
  pub struct Edge< Id = crate::IdentityWithInt, Kind = crate::EdgeKindless, Node = crate::canonical::Node >
  where
    Id : IdentityInterface,
    Kind : EdgeKindInterface,
    Node : NodeBasicInterface,
  {
    /// Input node.
    pub in_node : < Node as HasId >::Id,
    /// Output node.
    pub out_node : < Node as HasId >::Id,
    /// Kind of the edge.
    pub kind : Kind,
    /// Identifier.
    pub id : Id,
  }

  //

  impl< Id, Kind, Node > HasId
  for Edge< Id, Kind, Node >
  where
    Id : IdentityInterface,
    Kind : EdgeKindInterface,
    Node : NodeBasicInterface,
  {
    type Id = Id;
    fn id( &self ) -> Self::Id
    {
      self.id
    }
  }

  //

  impl< Id, Kind, Node > EdgeBasicInterface
  for Edge< Id, Kind, Node >
  where
    Id : IdentityInterface,
    Kind : EdgeKindInterface,
    Node : NodeBasicInterface,
  {
  }

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
