/// Internal namespace.
pub( crate ) mod private
{
  // use crate::prelude::*;
  // use std::collections::HashSet;

}

/// Implements canonical factory where each node in a cell.
pub mod cell_factory;
/// Implements canonical edge.
pub mod edge;
/// Implements canonical factory.
pub mod factory;
/// Implements canonical node.
pub mod node;

/// Protected namespace of the module.
pub mod protected
{
  pub use super::exposed::*;
  pub use super::cell_factory::orphan::*;
  pub use super::edge::orphan::*;
  pub use super::factory::orphan::*;
  pub use super::node::orphan::*;
}

pub use protected::*;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::cell_factory::exposed::*;
  pub use super::edge::exposed::*;
  pub use super::factory::exposed::*;
  pub use super::node::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::cell_factory::prelude::*;
  pub use super::edge::prelude::*;
  pub use super::factory::prelude::*;
  pub use super::node::prelude::*;
}
