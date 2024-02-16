#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/derive_tools/latest/derive_tools/" ) ]
// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

// #![ feature( trait_alias ) ]
// #![ feature( type_name_of_val ) ]

//!
//! Collection of derives which extend STD.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ cfg( feature = "enabled" ) ]
pub mod wtools;

// use derive_tools_meta::Deref;
// use derive_tools_meta::VariadicFrom;

/// Dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  #[ cfg( feature = "derive_more" ) ]
  pub use ::derive_more;
  #[ cfg( feature = "strum" ) ]
  pub use ::strum;
  #[ cfg( feature = "parse_display" ) ]
  pub use ::parse_display;
  #[ cfg( feature = "clone_dyn" ) ]
  pub use ::clone_dyn;
  #[ cfg( feature = "clone_dyn" ) ]
  pub use ::clone_dyn::dependency::*;
  #[ cfg( any_derive ) ]
  pub use ::derive_tools_meta;

}

/// Protected namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::clone_dyn::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::wtools::orphan::*;
}

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

#[ cfg( feature = "enabled" ) ]
// #[ doc( inline ) ]
#[ allow( unused_imports ) ]
// pub use exposed::*;

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

  #[ cfg( feature = "derive_more" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::derive_more::*;
  // qqq2 : list instead of asteris

  #[ cfg( feature = "strum" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::strum::*;

  #[ cfg( feature = "derive_display" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::parse_display::Display;

  #[ cfg( feature = "derive_from_str" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::parse_display::FromStr;

  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::clone_dyn::exposed::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::wtools::exposed::*;

  // #[ cfg( any_derive ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::derive_tools_meta::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{

  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::clone_dyn;
  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::clone_dyn::prelude::*;
  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::clone_dyn::clone_dyn;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::wtools::prelude::*;
  #[ doc( no_inline ) ]
  pub use super::wtools;

}
