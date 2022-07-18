#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/derive_tools/latest/derive_tools/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// zzz : qqq for Dima : use https://github.com/Peternator7/strum

// #![ feature( trait_alias ) ]
// #![ feature( type_name_of_val ) ]

//!
//! Collection of derives which extend STD.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Internal namespace.
pub( crate ) mod private
{
}

/// Dependencies.
pub mod dependency
{
  pub use ::derive_more;
  #[ cfg( feature = "parse_display" ) ]
  pub use ::parse_display;
  #[ cfg( feature = "clone_dyn" ) ]
  pub use ::clone_dyn;
}

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  pub use ::clone_dyn::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Orphan namespace of the module.
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
  #[ doc( inline ) ]
  pub use ::derive_more::*;

  #[ cfg( feature = "derive_display" ) ]
  #[ doc( inline ) ]
  pub use ::parse_display::Display;

  #[ cfg( feature = "derive_from_str" ) ]
  #[ doc( inline ) ]
  pub use ::parse_display::FromStr;

  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  pub use ::clone_dyn::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{

  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  pub use ::clone_dyn;
  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  pub use ::clone_dyn::prelude::*;
  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  pub use ::clone_dyn::clone_dyn;

}
