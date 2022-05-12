/// Internal namespace.
pub mod internal
{
  use crate::prelude::*;
  use crate::canonical::*;
  use std::collections::HashMap;
  use wtools::prelude::*;

  include!( "./factory_impl.rs" );

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
