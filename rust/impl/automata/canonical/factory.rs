/// Internal namespace.
pub mod internal
{
  use crate::prelude::*;
  use crate::canonical::*;
  use std::collections::HashMap;
  use wtools::prelude::*;

  macro_rules! IdOfFactory
  {
    () => { < < Self as NodeFactoryInterface >::Node as HasId >::Id };
  }

  ///
  /// Node factory.
  ///

  #[ derive( Debug ) ]
  pub struct NodeFactory
  {
    /// Map id to node.
    pub id_to_node_map : HashMap< IdOfFactory!(), crate::NodeCell< Node > >,
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

    pub fn node_making_id< Id >( &mut self, id : Id ) -> IdOfFactory!()
    where
      Id : Into< IdOfFactory!() >,
    {
      let id = id.into();

      let result = self.id_to_node_map
      .entry( id )
      .or_insert_with( || crate::NodeCell::make( Node::make_named( id ) ) )
      ;

      result.borrow().id()
    }

    ///
    /// Get node.
    ///

    pub fn node< Id >( &self, id : Id )
    -> &crate::NodeCell< Node >
    // -> &impl NodeCellInterface
    where
      Id : Into< IdOfFactory!() >,
    {
      let id = id.into();
      let got = self.id_to_node_map.get( &id );
      if got.is_some()
      {
        let result : &crate::NodeCell< Node > = got.unwrap().clone();
        return result;
      }
      unreachable!( "No node with id {:?} found", id );
    }

  }

  impl NodeFactory
  {

    index!
    {
      make,
      node_making_id,
      node,
    }

  }

//   impl GraphBasicInterface
//   for NodeFactory
//   {
//
//     ///
//     /// Get node.
//     ///
//
//     pub fn node< Id >( &self, id : Id )
//     -> &crate::NodeCell< Node >
//     // -> &impl NodeCellInterface
//     where
//       Id : Into< IdOfFactory!() >,
//     {
//       let id = id.into();
//       let got = self.id_to_node_map.get( &id );
//       if got.is_some()
//       {
//         let result : &crate::NodeCell< Node > = got.unwrap().clone();
//         return result;
//       }
//       unreachable!( "No node with id {:?} found", id );
//     }
//
//   }

  impl NodeFactoryInterface
  for NodeFactory
  {
    type Node = Node;
    type NodeHandle = crate::NodeCell< Self::Node >;

  }

  // impl GraphBasicInterface
  // for NodeFactory
  // {
  //
  // }

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
