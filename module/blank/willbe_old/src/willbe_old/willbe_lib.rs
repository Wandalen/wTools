#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/_blank/latest/_blank/" ) ]
// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

//!
//! ___.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

//
use mod_interface::mod_interface;

mod_interface!
{
  /// Features of Application Programming Interface that 100% should be implemented
  #[ cfg( not( feature = "no_std" ) ) ]
  layer core;

  /// Library of utility to work with commands.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer commands;

  /// Operate over files.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer files;

  protected( crate ) use ::wtools::prelude::*;
}
