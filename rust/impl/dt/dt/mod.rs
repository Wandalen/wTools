/// Internal namespace.
mod internal
{
}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Shared with parent namespace of the module
pub mod orphan
{
  pub use super::exposed::*;

}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use ::either::Either;
  pub use ::fundamental_data_type::exposed::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use either::*;
  pub use ::fundamental_data_type::prelude::*;
}
