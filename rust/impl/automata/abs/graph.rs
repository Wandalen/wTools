/// Internal namespace.
pub( crate ) mod private
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

    /// Iterate nodes.
    fn nodes< 'a, 'b >( &'a self )
    ->
    Box< dyn Iterator< Item = ( &ID!(), &Self::NodeHandle ) > + 'b >
    where
      'a : 'b,
    ;

    /// Get node with id.
    fn node< Id >( &self, id : Id ) -> &Self::NodeHandle
    where
      Id : Into< ID!() >
    ;

    /// Get node with id mutably.
    fn node_mut< Id >( &mut self, id : Id ) -> &mut Self::NodeHandle
    where
      Id : Into< ID!() >
    ;

    /// Iterate output nodes of the node.
    fn out_nodes< 'a, 'b, Id >( &'a self, node_id : Id )
    ->
    Box< dyn Iterator< Item = ID!() > + 'b >
    where
      Id : Into< ID!() >,
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
    fn node_extend_out_nodes< IntoId1, IntoId2, Iter >
    (
      &mut self,
      node_id : IntoId1,
      out_nodes_iter : Iter,
    )
    where
      IntoId1 : Into< ID!() >,
      IntoId2 : Into< ID!() >,
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
      IntoId1 : Into< ID!() >,
      IntoId1 : Clone,
      IntoId2 : Into< ID!() >,
      IntoId2 : Clone,
    {
      self.node_extend_out_nodes( node_id, core::iter::once( out_node_id ) );
    }

  }

  ///
  /// Graph interface which allow to add more nodes.
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
    fn make_with_edge_list< IntoIter, Id >( &mut self, into_iter : IntoIter )
    where
      Id : Into< ID!() >,
      IntoIter : IntoIterator< Item = Id >,
      IntoIter::IntoIter : core::iter::ExactSizeIterator< Item = Id >,
    {
      use wtools::iter::prelude::*;
      let iter = into_iter.into_iter();

      // debug_assert_eq!( into_iter.len() % 2, 0 );

      for mut chunk in &iter.chunks( 2 )
      {
        let id1 = chunk.next().unwrap().into();
        let id2 = chunk.next().unwrap().into();
        self.node_making( id1 );
        self.node_making( id2 );
        self.node_add_edge_to_node( id1, id2 );
        // println!( "{:?} -> {:?}", id1, id2 );
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
    GraphBasicInterface,
    GraphEditableInterface,
    GraphExtendableInterface,
    GraphKindGetterInterface,
  };
}
