/// Internal namespace.
pub( crate ) mod private
{

}

/// Several macro on functions.
pub mod func;
/// Several macro to encourage to write indexed code to improve readibility.
pub mod impls;

/* zzz : use name protected */
/* zzz : use for implementing of macro mod_interface */

// /// Namespace with dependencies.
// #[ cfg( feature = "enabled" ) ]
// pub mod dependency
// {
//   // #[ cfg( any( feature = "meta", feature = "impls_index_meta" ) ) ]
//   pub use ::impls_index_meta;
// }

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

/// Shared with parent namespace of the module
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
  // pub use super::dependency;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::impls::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::func::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::impls::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::func::prelude::*;
  // #[ cfg( any( feature = "meta", feature = "impls_index_meta" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::impls_index_meta::*;
}
