/// Internal namespace.
mod internal
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
    fn node_extend_out_nodes< Id, Iter >
    (
      &mut self,
      node_id : Id,
      out_nodes_iter : Iter,
    )
    where
      Id : Into< ID!() >,
      Iter : IntoIterator< Item = Id >,
      Iter::IntoIter : Clone,
    ;

    /// Iterate output nodes of the node.
    fn node_extend_out_node< Id >
    (
      &mut self,
      node_id : Id,
      out_node_id : Id,
    )
    where
      Id : Into< ID!() >,
      // ID!() : Into< ID!() >,
      // Id : < < Self as GraphBasicInterface >::NodeHandle as HasId >::Id,
      // core::iter::Once< Id > : Clone,
      Id : Clone,
    {
      // let out_node_id : ID!() = out_node_id.into();
      // self.node_extend_out_nodes( node_id, core::iter::once( out_node_id ) );
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
        self.node_extend_out_node( id1, id2 );
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

/// Own namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  // use super::internal as i;
  pub use super::exposed::*;
}

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
