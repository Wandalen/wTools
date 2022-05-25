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
  pub use ::type_constructor::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use either::*;
  pub use ::type_constructor::prelude::*;
}
