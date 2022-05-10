/// Internal namespace.
pub mod internal
{
  // use crate::prelude::*;
  // use std::collections::HashSet;

}

/// Implements canonical edge.
pub mod edge;
/// Implements canonical factory.
pub mod factory;
/// Implements canonical node.
pub mod node;

/// Own namespace of the module.
pub mod own
{
  // use super::internal as i;
  pub use super::exposed::*;
  pub use super::edge::parented::*;
  pub use super::factory::parented::*;
  pub use super::node::parented::*;
}

pub use own::*;

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
  pub use super::prelude::*;
  pub use super::edge::exposed::*;
  pub use super::factory::exposed::*;
  pub use super::node::exposed::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
  pub use super::edge::prelude::*;
  pub use super::factory::prelude::*;
  pub use super::node::prelude::*;
}
