/// Internal namespace.
pub mod internal
{
  use crate::prelude::*;
  use crate::canonical::*;
  use std::collections::HashMap;
  use wtools::prelude::*;

  macro_rules! ID
  {
    () => { < < Self as NodeFactoryInterface >::NodeHandle as HasId >::Id };
  }

  ///
  /// Node factory.
  ///

  #[ derive( Debug ) ]
  pub struct CellNodeFactory
  {
    /// Map id to node.
    pub id_to_node_map : HashMap< ID!(), crate::NodeCell< Node > >,
  }

  impls!
  {

    ///
    /// Constructor.
    ///

    pub fn make() -> Self
    {
      let id_to_node_map = HashMap::new();
      Self
      {
        id_to_node_map,
      }
    }

    ///
    /// Get node, making a new one if no such exist. Returns id of the node.
    ///

    pub fn node_making< Id >( &mut self, id : Id ) -> ID!()
    where
      Id : Into< ID!() >,
    {
      let id = id.into();

      let result = self.id_to_node_map
      .entry( id )
      .or_insert_with( || crate::NodeCell::make( Node::make_named( id ) ) )
      ;

      result.borrow().id()
    }

  }

  impl CellNodeFactory
  {

    index!
    {
      make,
      node_making,
    }

  }

  impl GraphBasicInterface
  for CellNodeFactory
  {
    type NodeHandle = crate::NodeCell< crate::canonical::Node >;

    fn node< Id >( &self, id : Id ) -> &Self::NodeHandle
    where
      Id : Into< ID!() >,
    {
      let id = id.into();
      let got = self.id_to_node_map.get( &id );
      if got.is_some()
      {
        let result : &Self::NodeHandle = got.unwrap().clone();
        return result;
      }
      unreachable!( "No node with id {:?} found", id );
    }

    fn node_mut< Id >( &mut self, id : Id ) -> &mut Self::NodeHandle
    where
      Id : Into< ID!() >,
    {
      let id = id.into();
      let got = self.id_to_node_map.get_mut( &id );
      if got.is_some()
      {
        let result : &mut Self::NodeHandle = got.unwrap();
        return result;
      }
      unreachable!( "No node with id {:?} found", id );
    }

    fn out_nodes< 'a, 'b, Id >( &'a self, node_id : Id )
    ->
    Box< dyn Iterator< Item = ID!() > + 'b >
    where
      Id : Into< ID!() >,
      'a : 'b,
    {
      let node = self.node( node_id ).borrow();
      let collected : Vec< ID!() > = node.out_nodes.iter().cloned().collect();
      let iterator : Box< dyn Iterator< Item = ID!() > > = Box::new( collected.into_iter() );
      iterator
    }

  }

  //

  impl GraphEditableInterface
  for CellNodeFactory
  {

    /// Iterate output nodes of the node.
    fn node_extend_out_nodes< Id, Iter >
    (
      &mut self,
      node_id : Id,
      out_nodes_iter : Iter,
    )
    where
      Iter : IntoIterator< Item = Id >,
      Id : Into< ID!() >,
    {
      let out_nodes_iter2 = out_nodes_iter.into_iter()
      .map( | id |
      {
        let id = id.into();
        self.node( id );
        id
      });
      self.node( node_id.into() ).borrow_mut().extend( out_nodes_iter2 );
      // self.node_mut( node_id.into() ).extend( out_nodes_iter );
    }

  }

  //

  impl NodeFactoryInterface
  for CellNodeFactory
  {
    // type Node = Node;
    type NodeHandle = crate::NodeCell< crate::canonical::Node >;
  }

}

/// Own namespace of the module.
pub mod own
{
  // use super::internal as i;
  pub use super::parented::*;
}

pub use own::*;

/// Parented namespace of the module.
pub mod parented
{
  pub use super::exposed::*;
  use super::internal as i;
  pub use i::CellNodeFactory;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  // use super::internal as i;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
}
