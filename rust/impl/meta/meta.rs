/// Internal namespace.
pub( crate ) mod private
{
}

/* zzz : use name protected */
/* zzz : use for implementing of macro mod_interface */

#[ doc( inline ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
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
  #[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
  #[ doc( inline ) ]
  pub use ::woptions::orphan::*;
  #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  #[ doc( inline ) ]
  pub use ::former::orphan::*;
}

/// Shared with parent namespace of the module
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
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
  #[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
  #[ doc( inline ) ]
  pub use ::woptions::exposed::*;
  #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  #[ doc( inline ) ]
  pub use ::former::exposed::*;

  #[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
  #[ doc( inline ) ]
  pub use ::woptions as options;
  #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  #[ doc( inline ) ]
  pub use ::former as former;

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

  pub use ::mod_interface::prelude::mod_interface;

  #[ cfg( feature = "for_each" ) ]
  #[ doc( inline ) ]
  pub use ::for_each::prelude::*;
  #[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
  #[ doc( inline ) ]
  pub use ::woptions::prelude::*;
  #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  #[ doc( inline ) ]
  pub use ::former::prelude::*;
  #[ cfg( feature = "collection_make" ) ]
  #[ doc( inline ) ]
  pub use ::literally::*;
}
