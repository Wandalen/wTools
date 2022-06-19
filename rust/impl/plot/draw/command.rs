/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  /// Interface of command to draw something.
  pub trait DrawCommandInterface
  where
    Self : fmt::Debug,
  {
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
  pub use super::
  {
    exposed::*,
    private::DrawCommandInterface,
  };
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::
  {
    prelude::*,
  };
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
  };
}
