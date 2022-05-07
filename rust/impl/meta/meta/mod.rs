/// Internal namespace.
pub mod internal
{

}

/// Dependencies.
pub mod dependencies
{
  pub use ::literally;
  pub use ::either;
}

/// Several macro to encourage write readable code.
pub mod impls;

/* xxx2 : use name protected */
/* zzz : use for implementing of macro mod_interface */

/// Owned namespace of the module.
pub mod own
{
  pub use super::parented::*;
}

pub use own::*;

/// Shared with parent namespace of the module
pub mod parented
{
  pub use super::exposed::*;
  pub use super::dependencies;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::impls::exposed::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use literally::*;
  pub use either::*;
  pub use super::impls::prelude::*;
}
