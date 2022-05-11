/// Internal namespace.
pub mod internal
{
  // use crate::prelude::*;
  // use std::collections::HashSet;

}

/// Implements canonical factory.
pub mod factory;

/// Own namespace of the module.
pub mod own
{
  // use super::internal as i;
  pub use super::exposed::*;
  pub use super::factory::parented::*;
}

pub use own::*;

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
  pub use super::prelude::*;
  pub use super::factory::exposed::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
  pub use super::factory::prelude::*;
}
