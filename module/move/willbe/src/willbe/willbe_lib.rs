#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/willbe/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

//!
//! Utility with set of tools for managing developer routines.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// The tools for operating over packages.
#[ cfg( not( feature = "no_std" ) ) ]
pub mod tools;
/// Commands library.
#[ cfg( not( feature = "no_std" ) ) ]
pub mod commands;

#[ cfg( not( feature = "no_std" ) ) ]
pub use ::std::env;
use ::wtools::prelude::*;

// wtools::meta::mod_interface!
// {
//   /// The tools for operating over packages.
//   #[ cfg( not( feature = "no_std" ) ) ]
//   layer tools;
//   /// Commands library.
//   #[ cfg( not( feature = "no_std" ) ) ]
//   layer commands;
//
//   #[ cfg( not( feature = "no_std" ) ) ]
//   prelude use ::std::env;
//   protected( crate ) use ::wtools::prelude::*;
// }
