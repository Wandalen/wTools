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


pub use mod_interface::mod_interface;
/// Micro wtools
pub mod wtools;

wtools::meta::mod_interface!
{
  /// The tools for operating over packages.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer tools;
  /// Commands library.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer commands;

  /// Commands library.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer command;

  /// Endpoints library.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer endpoint;

  /// Package library.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer package;

  /// Version library.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer version;

  /// Git library.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer git;
  
  /// Cargo library.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer cargo;

  /// Metadata cache.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer cache;

  #[ cfg( not( feature = "no_std" ) ) ]
  prelude use ::std::env;
  // protected( crate ) use wtools::prelude::*;
}
