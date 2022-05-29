/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  use crate::canonical::*;
  use wtools::prelude::*;
  use indexmap::IndexMap;
  use std::fmt;

  include!( "./factory_impl.rs" );

  impls!
  {

    ///
    /// Iterate output nodes of the node.
    ///

    fn node_add_out_nodes< IntoId1, IntoId2, Iter >
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
    {
      let out_nodes_iter2 = out_nodes_iter.into_iter()
      .map( | id |
      {
        let id = id.into();
        self.node( id );
        id
      });
      self.node( node_id.into() ).borrow_mut().extend( out_nodes_iter2 );
    }

    //

    fn out_nodes< 'a, 'b, IntoId >( &'a self, node_id : IntoId )
    ->
    Box< dyn Iterator< Item = ID!() > + 'b >
    where
      IntoId : Into< ID!() >,
      'a : 'b,
    {
      let node = self.node( node_id ).borrow();
      let collected : Vec< ID!() > = node.out_nodes.iter().cloned().collect();
      let iterator : Box< dyn Iterator< Item = ID!() > > = Box::new( collected.into_iter() );
      iterator
    }

  }

  ///
  /// Node factory.
  ///

  pub struct CellNodeFactory< Id = crate::IdentityWithInt, Kind = crate::NodeKindless >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
    CellNodeFactory< Id, Kind > : crate::NodeFactoryInterface,
  {
    /// Map id to node.
    pub id_to_node_map : IndexMap< ID!(), crate::NodeCell< Node< Id, Kind > > >,
  }

  //

  impl< Id, Kind > CellNodeFactory< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {
  }

  //

  impl< Id, Kind > fmt::Debug
  for CellNodeFactory< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {
    index!( fmt );
  }

  //

  impl< Id, Kind > Make0
  for CellNodeFactory< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {
    fn make_0() -> Self
    {
      let id_to_node_map = IndexMap::new();
      Self
      {
        id_to_node_map,
      }
    }
  }

  //

  impl< Id, Kind > GraphBasicInterface
  for CellNodeFactory< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {
    type NodeHandle = crate::NodeCell< crate::canonical::Node< Id, Kind > >;

    index!
    {
      node,
      nodes,
      node_mut,
      out_nodes,
    }

  }

  //

  impl< Id, Kind > GraphExtendableInterface
  for CellNodeFactory< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {

    index!
    {
      node_making,
    }

  }

  //

  impl< Id, Kind > GraphEditableInterface
  for CellNodeFactory< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {

    index!
    {
      node_add_out_nodes,
    }

  }

  //

  impl< Id, Kind > NodeFactoryInterface
  for CellNodeFactory< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
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
  pub use super::private::CellNodeFactory;
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
