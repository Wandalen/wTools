/// Internal namespace.
pub( crate ) mod private
{
  // use crate::prelude::*;
  // use std::collections::HashSet;

}

// /// Implements canonical factory.
// pub mod factory;

/// Protected namespace of the module.
pub mod protected
{
  // // use super::private as i;
  pub use super::exposed::*;
  // pub use super::factory::orphan::*;
}

pub use protected::*;

/// Exposed namespace of the module.
pub mod exposed
{
  // // use super::private as i;
  pub use super::prelude::*;
  // pub use super::factory::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  // // use super::private as i;
  // pub use super::factory::prelude::*;
}
