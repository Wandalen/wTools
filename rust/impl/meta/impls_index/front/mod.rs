/// Internal namespace.
mod internal
{

}

/// Several macro on functions.
pub mod func;
/// Several macro to encourage to write indexed code to improve readibility.
pub mod impls;

/* zzz : use name protected */
/* zzz : use for implementing of macro mod_interface */

/// Namespace with dependencies.
pub mod dependency
{
  // #[ cfg( any( feature = "meta", feature = "impls_index_meta" ) ) ]
  pub use ::impls_index_meta;
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
  // pub use super::dependencies;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::impls::exposed::*;
  pub use super::func::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::impls::prelude::*;
  pub use super::func::prelude::*;
  // #[ cfg( any( feature = "meta", feature = "impls_index_meta" ) ) ]
  pub use ::impls_index_meta::*;
}
