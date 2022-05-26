/// Internal namespace.
mod internal
{
}

/* zzz : use name protected */
/* zzz : use for implementing of macro mod_interface */

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  #[ cfg( feature = "impls_index" ) ]
  pub use ::impls_index::orphan::*;
  #[ cfg( feature = "mod_interface" ) ]
  pub use ::mod_interface::orphan::*;
  #[ cfg( feature = "for_each" ) ]
  pub use ::for_each::orphan::*;
  #[ cfg( feature = "options" ) ]
  pub use ::woptions::orphan::*;
  #[ cfg( feature = "former" ) ]
  pub use ::former::orphan::*;
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

  #[ cfg( feature = "impls_index" ) ]
  pub use ::impls_index::exposed::*;
  #[ cfg( feature = "mod_interface" ) ]
  pub use ::mod_interface::exposed::*;
  #[ cfg( feature = "for_each" ) ]
  pub use ::for_each::exposed::*;
  #[ cfg( feature = "options" ) ]
  pub use ::woptions::exposed::*;
  #[ cfg( feature = "former" ) ]
  pub use ::former::exposed::*;

  #[ cfg( feature = "options" ) ]
  pub use ::woptions as options;
  #[ cfg( feature = "former" ) ]
  pub use ::former as former;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "impls_index" ) ]
  pub use ::impls_index::prelude::*;
  #[ cfg( feature = "mod_interface" ) ]
  pub use ::mod_interface::prelude::*;
  #[ cfg( feature = "for_each" ) ]
  pub use ::for_each::prelude::*;
  #[ cfg( feature = "options" ) ]
  pub use ::woptions::prelude::*;
  #[ cfg( feature = "former" ) ]
  pub use ::former::prelude::*;
  #[ cfg( feature = "collection_make" ) ]
  pub use ::literally::*;
}
