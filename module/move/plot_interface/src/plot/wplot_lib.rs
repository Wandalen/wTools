#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/wplot/latest/wplot/" ) ]
// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

// #![ feature( trace_macros ) ]

//!
//! Plot interface.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

// pub use ::wmath as math;
// use ::wtools::prelude::*;
use ::wtools::mod_interface;

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  pub use ::image;
  pub use ::open;
  pub use ::wmath;
  pub use ::rgb;
}

crate::mod_interface!
{

  /// Describe colors.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer color;
  /// Abstraction.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer abs;
  /// Concrete system.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer sys;

  use super::math;
  own use ::wmath as math;
  protected( crate ) use ::wtools::prelude::*;

}
