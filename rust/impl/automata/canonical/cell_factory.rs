/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  use crate::canonical::*;
  use wtools::prelude::*;
  use std::collections::HashMap;
  // use std::fmt;

  include!( "./factory_impl.rs" );

  impls!
  {

    ///
    /// Iterate output nodes of the node.
    ///

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

  ///
  /// Node factory.
  ///

  #[ derive( Debug ) ]
  pub struct CellNodeFactory
  {
    /// Map id to node.
    pub id_to_node_map : HashMap< ID!(), crate::NodeCell< Node > >,
  }

  impl CellNodeFactory
  {

    // index!
    // {
    //   make,
    // }

  }

  impl Make0 for CellNodeFactory
  {
    fn make_0() -> Self
    {
      let id_to_node_map = HashMap::new();
      Self
      {
        id_to_node_map,
      }
    }
  }

  //

  impl GraphBasicInterface
  for CellNodeFactory
  {
    type NodeHandle = crate::NodeCell< crate::canonical::Node >;

    index!
    {
      node,
      node_mut,
      out_nodes,
    }

  }

  //

  impl GraphExtendableInterface
  for CellNodeFactory
  {

    index!
    {
      node_making,
    }

  }

  //

  impl GraphEditableInterface
  for CellNodeFactory
  {

    index!
    {
      node_extend_out_nodes,
    }

  }

  //

  impl NodeFactoryInterface
  for CellNodeFactory
  {
    // type NodeHandle = crate::canonical::Node;
    type NodeHandle = crate::NodeCell< crate::canonical::Node >;
  }

}

/// Protected namespace of the module.
pub mod protected
{
  // // use super::private as i;
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
  // use super::private as i;
  pub use super::private::CellNodeFactory;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  // // use super::private as i;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  // // use super::private as i;
}
