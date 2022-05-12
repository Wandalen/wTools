/// Internal namespace.
pub mod internal
{
  use crate::prelude::*;

  macro_rules! ID
  {
    () => { < < Self as GraphBasicInterface >::NodeHandle as HasId >::Id };
  }

  ///
  /// Graph which know how to iterate output nodes of a given node.
  ///

  pub trait GraphBasicInterface
  {

    /// It's not always possible to operate a node directly, for example it it has to be wrapped by cell ref. For that use NodeHandle.
    /// Otherwise NodeHandle could be &Node.
    type NodeHandle : NodeBasicInterface;

    /// Get node with id.
    fn node( &self, id : ID!() ) -> &Self::NodeHandle;

    /// Get node with id mutably.
    fn node_mut( &mut self, id : ID!() ) -> &mut Self::NodeHandle;

    /// Iterate output nodes of the node.
    fn out_nodes< 'a, 'b >( &'a self, node_id : ID!() )
    ->
    Box< dyn Iterator< Item = ID!() > + 'b >
    where
      'a : 'b,
    ;

  }

  ///
  /// Graph which allow to add more edges between nodes.
  ///

  pub trait GraphEditableInterface
  where
    Self :
      GraphBasicInterface +
    ,
  {

    /// Iterate output nodes of the node.
    fn node_extend_out_nodes< Iter >
    (
      &mut self,
      node_id : ID!(),
      out_nodes_iter : Iter,
    )
    where
      Iter : IntoIterator< Item = ID!() >,
    ;

    /// Iterate output nodes of the node.
    fn node_extend_out_node
    (
      &mut self,
      node_id : ID!(),
      out_node_id : ID!(),
    )
    {
      self.node_extend_out_nodes( node_id, core::iter::once( out_node_id ) );
    }

  }

  ///
  /// Graph which allow to add more nodes.
  ///

  pub trait GraphExtendableInterface
  where
    Self :
      GraphBasicInterface +
      GraphEditableInterface +
    ,
  {

    /// Either make new or get existing node.
    fn node_making< Id >( &mut self, id : Id ) -> ID!()
    where
      Id : Into< ID!() >
    ;

    /// Make edges.
    fn make_edge_list< IntoIter, Id >( &mut self, into_iter : IntoIter )
    where
      Id : Into< ID!() >,
      IntoIter : IntoIterator< Item = Id >,
    {
      use wtools::iter::prelude::*;
      let iter = into_iter.into_iter();

      for [ src, dst ] in &iter.chunks( 2 )
      {
          println!( "{}--{}", src, dst );
      }

      // for id in iter
      // {
      //   let id = id.into();
      // }
    }

  }

  ///
  /// Graph nodes of which has a kind.
  ///

  pub trait GraphKindGetterInterface
  where
    Self : GraphBasicInterface,
  {
    /// Enumerate kinds of the node.
    type NodeKind : crate::NodeKindInterface;
    /// Get kind of the node.
    fn node_kind( &self, node_id : ID!() ) -> Self::NodeKind;
  }

}

/// Parented namespace of the module.
pub mod parented
{
  // use super::internal as i;
  pub use super::exposed::*;
}

pub use parented::*;

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
  pub use super::prelude::*;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::GraphBasicInterface;
  pub use i::GraphEditableInterface;
  pub use i::GraphExtendableInterface;
  pub use i::GraphKindGetterInterface;
}
