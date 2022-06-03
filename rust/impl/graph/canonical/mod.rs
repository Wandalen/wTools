/// Internal namespace.
pub( crate ) mod private
{
  // use crate::prelude::*;
  // use std::collections::HashSet;

}

/// Implements canonical factory where each node in a cell.
#[ cfg( feature = "cell_factory" ) ]
pub mod cell_factory;
/// Implements canonical edge.
pub mod edge;
/// Implements canonical factory.
pub mod factory;
/// Implements several identities.
pub mod identity;
/// Implements canonical node.
pub mod node;
/// Implements node cell.
#[ cfg( feature = "cell_factory" ) ]
pub mod node_cell;


/// Protected namespace of the module.
pub mod protected
{
  pub use super::exposed::*;
  #[ cfg( feature = "cell_factory" ) ]
  pub use super::cell_factory::orphan::*;
  pub use super::edge::orphan::*;
  pub use super::factory::orphan::*;
  pub use super::identity::orphan::*;
  pub use super::node::orphan::*;
  #[ cfg( feature = "cell_factory" ) ]
  pub use super::node_cell::orphan::*;
}

pub use protected::*;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  #[ cfg( feature = "cell_factory" ) ]
  pub use super::cell_factory::exposed::*;
  pub use super::edge::exposed::*;
  pub use super::factory::exposed::*;
  pub use super::identity::exposed::*;
  pub use super::node::exposed::*;
  #[ cfg( feature = "cell_factory" ) ]
  pub use super::node_cell::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "cell_factory" ) ]
  pub use super::cell_factory::prelude::*;
  pub use super::edge::prelude::*;
  pub use super::factory::prelude::*;
  pub use super::identity::prelude::*;
  pub use super::node::prelude::*;
  #[ cfg( feature = "cell_factory" ) ]
  pub use super::node_cell::prelude::*;
}
