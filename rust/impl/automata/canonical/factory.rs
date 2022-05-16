/// Internal namespace.
mod internal
{
  use crate::prelude::*;
  use crate::canonical::*;
  use std::collections::HashMap;
  use wtools::prelude::*;

  include!( "./factory_impl.rs" );

  impls!
  {

    ///
    /// Iterate output nodes of the node.
    ///

    fn node_extend_out_nodes< Id, Iter >
    (
      &mut self,
      node_id : Id,
      out_nodes_iter : Iter,
    )
    where
      Iter : IntoIterator< Item = Id >,
      Iter::IntoIter : Clone,
      Id : Into< ID!() >
    {

      let iter = out_nodes_iter.into_iter();
      let iter2 = iter.clone();

      #[ cfg( debug_assertions ) ]
      iter
      .map( | id |
      {
        let node = self.node( id );
      })
      .fold( (), | acc, e | () )
      ;

      let iter3 = iter2.into_iter()
      .map( | id |
      {
        let id = id.into();
        id
      })
      ;

      self.node_mut( node_id.into() ).extend( iter3 );
    }

    //

    fn out_nodes< 'a, 'b, Id >( &'a self, node_id : Id )
    ->
    Box< dyn Iterator< Item = ID!() > + 'b >
    where
      Id : Into< ID!() >,
      'a : 'b,
    {
      let node = self.node( node_id );
      let iterator
        : Box< dyn Iterator< Item = ID!() > >
        = Box::new( node.out_nodes.iter().cloned() );
      iterator
    }

  }

  ///
  /// Node factory.
  ///

  #[ derive( Debug ) ]
  pub struct NodeFactory
  {
    /// Map id to node.
    pub id_to_node_map : HashMap< ID!(), crate::canonical::Node >,
  }

  impl NodeFactory
  {

    index!
    {
      make,
    }

  }

  //

  impl GraphBasicInterface
  for NodeFactory
  {
    type NodeHandle = crate::canonical::Node;

    index!
    {
      node,
      node_mut,
      out_nodes,
    }

  }

  //

  impl GraphExtendableInterface
  for NodeFactory
  {

    index!
    {
      node_making,
    }

  }

  //

  impl GraphEditableInterface
  for NodeFactory
  {

    index!
    {
      node_extend_out_nodes,
    }

  }

  //

  impl NodeFactoryInterface
  for NodeFactory
  {
    type NodeHandle = crate::canonical::Node;
  }

}

/// Own namespace of the module.
pub mod protected
{
  // use super::internal as i;
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
  use super::internal as i;
  pub use i::NodeFactory;
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
