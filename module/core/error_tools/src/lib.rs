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
  #[ cfg( feature = "error_typed" ) ]
  pub use ::thiserror;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "error_untyped" ) ]
  pub use ::anyhow;

}

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "error_typed" ) ]
/// Typed exceptions handling mechanism.
pub mod typed;

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "error_untyped" ) ]
/// Untyped exceptions handling mechanism.
pub mod untyped;

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod protected
{
  #[ allow( unused_imports ) ]
  use super::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use assert::orphan::*;

  #[ cfg( not( feature = "no_std" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use error::orphan::*;

  #[ cfg( feature = "error_untyped" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use untyped::orphan::*;

  #[ cfg( feature = "error_typed" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use typed::orphan::*;

}

/// Shared with parent namespace of the module
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{

  #[ doc( inline ) ]
  pub use super::exposed::*;

}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use assert::exposed::*;

  #[ cfg( not( feature = "no_std" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use error::exposed::*;

  #[ cfg( feature = "error_untyped" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use untyped::exposed::*;

  #[ cfg( feature = "error_typed" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use typed::exposed::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ allow( unused_imports ) ]
  use super::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use assert::prelude::*;

  #[ cfg( not( feature = "no_std" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use error::prelude::*;

  #[ cfg( feature = "error_untyped" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use untyped::prelude::*;

  #[ cfg( feature = "error_typed" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use typed::prelude::*;

}
