#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/wca/latest/wca/" ) ]
#![ deny( rust_2021_compatibility ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]
#![ deny( unused_imports ) ]

//!
//! The tool to make CLI ( commands user interface ). It is able to aggregate external binary applications, as well as functions, which are written in your language.
//!
// qqq : when trying to use wca in a project we get the following message:
// wca-0.3.0/../../../doc/modules/wca/wca.md: The system cannot find the specified path. (os error 3)
// solution options symbolic link or moving the contents of the nue folder to level 1 with the project
// #![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/../../../", "doc/modules/wca/", "wca.md" ) ) ]

// #![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#![ allow( where_clauses_object_safety ) ] // https://github.com/chris-morgan/anymap/issues/31

pub use mod_interface::mod_interface;
/// Tools
pub mod wtools;

/// Errors.
#[ cfg( not( feature = "no_std" ) ) ]
use wtools::error::{ BasicError };
// xxx : check

crate::mod_interface!
{
  /// Commands aggregator library.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer ca;
}
