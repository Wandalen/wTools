/// Internal namespace.
mod internal
{

}

/// Dependencies.
pub mod dependencies
{
  pub use ::literally;
  pub use ::for_each;
  pub use ::impls_index;
}

// /// Several macro on functions.
// pub mod func;
// /// Several macro to encourage to write indexed code to improve readibility.
// pub mod impls;

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
  pub use ::impls_index::exposed::*;
  // pub use super::impls::exposed::*;
  // pub use super::func::exposed::*;
  pub use super::dependencies::for_each::exposed::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use ::literally::*;
  pub use ::impls_index::prelude::*;
  // pub use super::impls::prelude::*;
  // pub use super::func::prelude::*;
  pub use super::dependencies::for_each::prelude::*;
}
