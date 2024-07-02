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
#[ allow( unused_imports ) ]
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

/// Shared with parent namespace of the module
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
  // pub use super::dependency;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::impls::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::func::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
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
