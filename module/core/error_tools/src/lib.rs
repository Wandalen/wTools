#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/error_tools/latest/error_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Assertions.
#[ cfg( feature = "enabled" ) ]
pub mod assert;

/// Alias for std::error::BasicError.
#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
pub mod error;

/// Namespace with dependencies.

#[ cfg( feature = "enabled" ) ]
pub mod dependency
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "error_for_lib" ) ]
  pub use ::thiserror;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "error_for_app" ) ]
  pub use ::anyhow;

}

#[ cfg( feature = "enabled" ) ]
/// Exceptions handling mechanism for libs.
pub mod for_lib
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "error_for_lib" ) ]
  pub use ::thiserror::*;
}

#[ cfg( feature = "enabled" ) ]
// qqq : cover by simple test /* aaa : Dmytro : added trivial test routine `basic` */
/// Exceptions handling mechanism for apps.
pub mod for_app
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "error_for_app" ) ]
  pub use ::anyhow::*;
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

/// Shared with parent namespace of the module
#[ cfg( feature = "enabled" ) ]
pub mod orphan
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;

  #[ cfg( feature = "error_for_app" ) ]
  pub use super::for_app::Result;

}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::assert::exposed::*;

  #[ cfg( not( feature = "no_std" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::error::exposed::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{
}
