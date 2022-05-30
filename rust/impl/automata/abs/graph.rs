/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;

  macro_rules! NODE_ID
  {
    () => { < < Self as GraphNodesInterface >::NodeHandle as HasId >::Id };
  }

  macro_rules! EDGE_ID
  {
    () => { < < Self as GraphEdgesInterface >::EdgeHandle as HasId >::Id };
  }

  ///
  /// Graph which know how to iterate neighbourhood of a node and capable to convert id of a node into a node.
  ///

  pub trait GraphNodesInterface
  {

    /// Handle of a node - entity representing a node or the node itself.
    /// It's not always possible to operate a node directly, for example it it has to be wrapped by cell ref. For that use NodeHandle.
    /// Otherwise NodeHandle could be &Node.
    type NodeHandle : NodeBasicInterface;

    /// Get node with id.
    fn node< Id >( &self, id : Id ) -> &Self::NodeHandle
    where
      Id : Into< NODE_ID!() >
    ;

    /// Iterate over all nodes.
    fn nodes< 'a, 'b >( &'a self )
    ->
    Box< dyn Iterator< Item = ( &NODE_ID!(), &Self::NodeHandle ) > + 'b >
    where
      'a : 'b,
    ;

    /// Iterate over neighbourhood of the node.
    fn out_nodes< 'a, 'b, Id >( &'a self, node_id : Id )
    ->
    Box< dyn Iterator< Item = NODE_ID!() > + 'b >
    where
      Id : Into< NODE_ID!() >,
      'a : 'b,
    ;

  }

  ///
  /// Graph which know how to iterate neighbourhood of a node and capable to convert id of a node into a node.
  ///

  pub trait GraphEdgesInterface
  where
    Self : GraphNodesInterface,
  {

    /// Handle of an edge - entity representing an edge or the edge itself.
    /// It's not always possible to operate an edge directly, for example it it has to be wrapped by cell ref. For that use NodeHandle.
    /// Otherwise EdgeHandle could be &Node.
    type EdgeHandle : EdgeBasicInterface;

    /// Get edge with id.
    fn edge< Id >( &self, id : Id ) -> &Self::EdgeHandle
    where
      Id : Into< EDGE_ID!() >
    ;

    /// Iterate over all edges.
    fn edges< 'a, 'b >( &'a self )
    ->
    Box< dyn Iterator< Item = ( &EDGE_ID!(), &Self::EdgeHandle ) > + 'b >
    where
      'a : 'b,
    ;

    /// Iterate over output edges of the node.
    fn out_edges< 'a, 'b, IntoId >( &'a self, node_id : IntoId )
    ->
    Box< dyn Iterator< Item = EDGE_ID!() > + 'b >
    where
      IntoId : Into< NODE_ID!() >,
      'a : 'b,
    ;

  }

  ///
  /// Graph which allow to add more edges between nodes.
  ///

  pub trait GraphEditableInterface
  where
    Self :
      GraphNodesInterface +
    ,
  {

    /// Get node with id mutably.
    fn node_mut< Id >( &mut self, id : Id ) -> &mut Self::NodeHandle
    where
      Id : Into< NODE_ID!() >
    ;

    /// Iterate output nodes of the node.
    fn node_add_out_nodes< IntoId1, IntoId2, Iter >
    (
      &mut self,
      node_id : IntoId1,
      out_nodes_iter : Iter,
    )
    where
      IntoId1 : Into< NODE_ID!() >,
      IntoId2 : Into< NODE_ID!() >,
      Iter : IntoIterator< Item = IntoId2 >,
      Iter::IntoIter : Clone,
    ;

    /// Iterate output nodes of the node.
    fn node_add_edge_to_node< IntoId1, IntoId2 >
    (
      &mut self,
      node_id : IntoId1,
      out_node_id : IntoId2,
    )
    where
      IntoId1 : Into< NODE_ID!() >,
      IntoId1 : Clone,
      IntoId2 : Into< NODE_ID!() >,
      IntoId2 : Clone,
    {
      self.node_add_out_nodes( node_id, core::iter::once( out_node_id ) );
    }

  }

  ///
  /// Graph interface which allow to add more nodes.
  ///

  pub trait GraphExtendableInterface
  where
    Self :
      GraphNodesInterface +
      GraphEditableInterface +
    ,
  {

    /// Either make new or get existing node.
    fn node_making< Id >( &mut self, id : Id ) -> NODE_ID!()
    where
      Id : Into< NODE_ID!() >
    ;

    /// Make edges.
    fn make_with_edge_list< IntoIter, Id >( &mut self, into_iter : IntoIter )
    where
      Id : Into< NODE_ID!() >,
      IntoIter : IntoIterator< Item = Id >,
      IntoIter::IntoIter : core::iter::ExactSizeIterator< Item = Id >,
    {
      use wtools::iter::prelude::*;
      let iter = into_iter.into_iter();
      debug_assert_eq!( iter.len() % 2, 0 );
      for mut chunk in &iter.chunks( 2 )
      {
        let id1 = chunk.next().unwrap().into();
        let id2 = chunk.next().unwrap().into();
        self.node_making( id1 );
        self.node_making( id2 );
        self.node_add_edge_to_node( id1, id2 );
      }

    }

  }

  ///
  /// Graph nodes of which has a kind.
  ///

  pub trait GraphKindGetterInterface
  where
    Self : GraphNodesInterface,
  {
    /// Enumerate kinds of the node.
    type NodeKind : crate::NodeKindInterface;
    /// Get kind of the node.
    fn node_kind( &self, node_id : NODE_ID!() ) -> Self::NodeKind;
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
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
    GraphNodesInterface,
    GraphEdgesInterface,
    GraphEditableInterface,
    GraphExtendableInterface,
    GraphKindGetterInterface,
  };
}
