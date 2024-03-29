#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/reflect_tools/latest/reflect_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "reflect_reflect" ) ]
pub mod reflect;

/// Namespace with dependencies.

#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  #[ cfg( any_derive ) ]
  pub use ::reflect_tools_meta;
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
  #[ cfg( feature = "reflect_reflect" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::reflect::orphan::*;
}

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;

  #[ cfg( feature = "reflect_reflect" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::reflect::exposed::*;

  // #[ cfg( any_derive ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::reflect_tools_meta::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{

  #[ cfg( feature = "reflect_reflect" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::reflect::prelude::*;

}
