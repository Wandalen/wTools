/// Internal namespace.
mod internal
{

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
  pub use ::impls_index::orphan::*;
  pub use ::mod_interface::orphan::*;
  pub use ::for_each::orphan::*;
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
  pub use ::impls_index::exposed::*;
  pub use ::mod_interface::exposed::*;
  pub use ::for_each::exposed::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use ::literally::*;
  pub use ::impls_index::prelude::*;
  pub use ::mod_interface::prelude::*;
  pub use ::for_each::prelude::*;
}
