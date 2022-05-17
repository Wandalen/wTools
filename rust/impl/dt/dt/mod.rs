/// Internal namespace.
mod internal
{

}

/// Dependencies.
pub mod dependencies
{
  pub use ::either;
}

/* zzz : use name protected */
/* zzz : use for implementing of macro mod_interface */

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
  pub use super::dependencies;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use ::either::Either;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use either::*;
}
