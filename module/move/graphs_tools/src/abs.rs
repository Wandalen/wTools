
/// Define a private namespace for all its items.
mod private
{

  pub use iter_tools::{ _IterTrait, IterTrait, BoxedIter };

  use std::
  {
    hash::Hash,
    fmt,
  };

  ///
  /// Interface to identify an instance of somthing, for exampel a node.
  ///

  pub trait IdentityInterface
  where
    Self :
      'static +
      Copy +
      Hash +
      fmt::Debug +
      PartialEq +
      Eq
    ,
  {
  }

  impl< T > IdentityInterface for T
  where
    T :
      'static +
      Copy +
      Hash +
      fmt::Debug +
      PartialEq +
      Eq
    ,
  {
  }

  /// Uniquely identify a node.
  pub trait NodeId : IdentityInterface
  {
  }

  /// Node itsef.
  pub trait Node
  {
  }

  /// Represent directed graph. Can be zero-sized structure if nodes own all the information.
  pub trait GraphDirected< 'a >
  {
    /// Uniquely identify a node.
    type NodeId : NodeId;
    /// Node itself.
    type Node : Node + 'a;

    /// Get a reference on a node by its id.
    fn node_ref( &'a self, node_id : Self::NodeId ) -> &'a Self::Node;
    /// Get id by its node reference.
    fn node_id( &self, node_id : &'a Self::Node ) -> Self::NodeId;

    /// Iterate over out nodes of
    fn node_out_nodes( &'a self, node_id : Self::NodeId ) -> BoxedIter< 'a, Self::NodeId >;

  }

}

crate::mod_interface!
{
  own use
  {
    // _IterTrait,
    IdentityInterface,
    NodeId,
    Node,
    GraphDirected,

  };
}
