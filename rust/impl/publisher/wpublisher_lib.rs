#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/wpublisher/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Utility to publish modules on `crates.io` from a command line.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

#[ cfg( feature = "use_std" ) ]
pub use std::env;
#[ allow( unused_imports ) ]
pub use wca::instruction;
#[ cfg( feature = "use_std" ) ]
pub use crate::wpublisher::
{
  bool,
  digest,
  files,
  http,
  manifest,
  process,
};

// qqq : for Dima : mod interface /* aaa : Dmytro : replaced */
wtools::mod_interface!
{
  layer wpublisher;
  layer commands;
}

