#![ no_std ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/clone_dyn/latest/clone_dyn/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Namespace with dependencies.

#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  #[ cfg( feature = "clone_dyn_meta" ) ]
  pub use ::clone_dyn_meta;
  #[ cfg( feature = "clone_dyn_types" ) ]
  pub use ::clone_dyn_types;
}

/// Internal namespace.
#[ cfg( feature = "enabled" ) ]
pub( crate ) mod private
{
}

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod orphan
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "clone_dyn_types" ) ]
  pub use super::dependency::clone_dyn_types::exposed::*;

}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "clone_dyn_meta" ) ]
  pub use ::clone_dyn_meta::clone_dyn;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "clone_dyn_types" ) ]
  pub use super::dependency::clone_dyn_types::prelude::*;

}
