#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/error_tools/latest/error_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Alias for std::error::BasicError.
#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
pub mod error;

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{

  #[ doc( inline ) ]
  #[ cfg( feature = "error_typed" ) ]
  pub use ::thiserror;

  #[ doc( inline ) ]
  #[ cfg( feature = "error_untyped" ) ]
  pub use ::anyhow;

}

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;

  #[ doc( inline ) ]
  pub use error::own::*;

}

/// Shared with parent namespace of the module
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;

  #[ doc( inline ) ]
  pub use exposed::*;

  #[ doc( inline ) ]
  pub use error::orphan::*;

}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  pub use error::exposed::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  #[ doc( inline ) ]
  pub use error::prelude::*;

}
