/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  use crate::canonical::*;
  use wtools::prelude::*;
  // use std::collections::HashMap;
  use std::fmt;
  use indexmap::IndexMap;

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

  // #[ derive( Debug ) ]
  pub struct NodeFactory
  {
    /// Map id to node.
    pub id_to_node_map : IndexMap< ID!(), crate::canonical::Node >,
  }

  impl NodeFactory
  {
  }

  // < < Self as NodeFactoryInterface >::NodeHandle as HasId >::Id

  impl fmt::Debug for NodeFactory
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      f.write_fmt( format_args!( "NodeFactory\n" ) )?;
      let mut first = true;
      for ( _id, node ) in self.nodes()
      {
        if !first
        {
          f.write_str( "\n" )?;
        }
        first = false;
        f.write_str( &wtools::string::indentation( "  ", format!( "{:?}", node ), "" ) )?;
      }
      f.write_str( "" )
    }
  }

  // xxx : test
  impl Make0 for NodeFactory
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

  impl GraphBasicInterface
  for NodeFactory
  {
    type NodeHandle = crate::canonical::Node;

    index!
    {
      node,
      node_mut,
      out_nodes,
      nodes,
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
  pub use super::private::NodeFactory;
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
