#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/wca/latest/wca/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! The tool to make CLI ( commands user interface ). It is able to aggregate external binary applications, as well as functions, which are written in your language.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Requests parser.
#[ cfg( feature = "use_std" ) ]
pub mod string
{
  pub use wtools::string::*;
}

use wtools::meta::mod_interface;

crate::mod_interface!
{
  /// Commands aggregator library.
  #[ cfg( feature = "use_std" ) ]
  layer ca;

  // protected( crate ) use super::
  // {
  //   field_str,
  //   field_map_str_str,
  //   field_map_str_vec_str,
  //   field_routine,
  // };

}

#[ cfg( feature = "use_std" ) ]
pub use ca::
{
  commands_aggregator,
  common,
  command,
  input,
  instruction,
};
