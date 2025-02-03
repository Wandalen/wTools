/// Define a private namespace for all its items.
mod private
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
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
  #[ doc( inline ) ]
  pub use ::impls_index_meta::*;
}

/// Shared with parent namespace of the module
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*;
  #[ doc( inline ) ]
  pub use impls::exposed::*;
  #[ doc( inline ) ]
  pub use func::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  #[ doc( inline ) ]
  pub use impls::prelude::*;
  #[ doc( inline ) ]
  pub use func::prelude::*;
}
