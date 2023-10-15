#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/wcensor/latest/wcensor/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

//!
//! Library of utility to operate files from a command line.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

use wtools::meta::mod_interface;

crate::mod_interface!
{
  /// Result of parsing.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer instruction;
  /// Properties parsing.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer props;
}
