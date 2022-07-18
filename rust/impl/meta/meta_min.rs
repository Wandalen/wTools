/// Internal namespace.
pub( crate ) mod private
{
}

/* zzz : use name protected */
/* zzz : use for implementing of macro mod_interface */

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
  #[ cfg( feature = "impls_index" ) ]
  #[ doc( inline ) ]
  pub use ::impls_index::orphan::*;
  #[ cfg( feature = "mod_interface" ) ]
  #[ doc( inline ) ]
  pub use ::mod_interface::orphan::*;
  #[ cfg( feature = "for_each" ) ]
  #[ doc( inline ) ]
  pub use ::for_each::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Shared with parent namespace of the module
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;

  #[ cfg( feature = "impls_index" ) ]
  #[ doc( inline ) ]
  pub use ::impls_index::exposed::*;
  #[ cfg( feature = "mod_interface" ) ]
  #[ doc( inline ) ]
  pub use ::mod_interface::exposed::*;
  #[ cfg( feature = "for_each" ) ]
  #[ doc( inline ) ]
  pub use ::for_each::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "impls_index" ) ]
  #[ doc( inline ) ]
  pub use ::impls_index::prelude::*;
  #[ cfg( feature = "mod_interface" ) ]
  #[ doc( inline ) ]
  pub use ::mod_interface::prelude::*;
  #[ cfg( feature = "for_each" ) ]
  #[ doc( inline ) ]
  pub use ::for_each::prelude::*;
  #[ cfg( feature = "collection_make" ) ]
  #[ doc( inline ) ]
  pub use ::literally::*;
}
